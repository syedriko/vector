// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Buf;
use http::StatusCode;
use http::Uri;
use log::debug;
use reqsign::AliyunConfig;
use reqsign::AliyunLoader;
use reqsign::AliyunOssSigner;

use super::core::*;
use super::error::parse_error;
use super::lister::OssLister;
use super::writer::OssWriter;
use crate::raw::*;
use crate::services::oss::writer::OssWriters;
use crate::*;

const DEFAULT_BATCH_MAX_OPERATIONS: usize = 1000;

/// Aliyun Object Storage Service (OSS) support
#[doc = include_str!("docs.md")]
#[derive(Default)]
pub struct OssBuilder {
    root: Option<String>,

    endpoint: Option<String>,
    presign_endpoint: Option<String>,
    bucket: String,

    // OSS features
    server_side_encryption: Option<String>,
    server_side_encryption_key_id: Option<String>,
    allow_anonymous: bool,

    // authenticate options
    access_key_id: Option<String>,
    access_key_secret: Option<String>,

    http_client: Option<HttpClient>,
    /// batch_max_operations
    batch_max_operations: Option<usize>,
}

impl Debug for OssBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Builder");
        d.field("root", &self.root)
            .field("bucket", &self.bucket)
            .field("endpoint", &self.endpoint)
            .field("allow_anonymous", &self.allow_anonymous);

        d.finish_non_exhaustive()
    }
}

impl OssBuilder {
    /// Set root of this backend.
    ///
    /// All operations will happen under this root.
    pub fn root(&mut self, root: &str) -> &mut Self {
        self.root = if root.is_empty() {
            None
        } else {
            Some(root.to_string())
        };

        self
    }

    /// Set bucket name of this backend.
    pub fn bucket(&mut self, bucket: &str) -> &mut Self {
        self.bucket = bucket.to_string();

        self
    }

    /// Set endpoint of this backend.
    pub fn endpoint(&mut self, endpoint: &str) -> &mut Self {
        if !endpoint.is_empty() {
            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
            self.endpoint = Some(endpoint.trim_end_matches('/').to_string())
        }

        self
    }

    /// Set a endpoint for generating presigned urls.
    ///
    /// You can offer a public endpoint like <https://oss-cn-beijing.aliyuncs.com> to return a presinged url for
    /// public accessors, along with an internal endpoint like <https://oss-cn-beijing-internal.aliyuncs.com>
    /// to access objects in a faster path.
    ///
    /// - If presign_endpoint is set, we will use presign_endpoint on generating presigned urls.
    /// - if not, we will use endpoint as default.
    pub fn presign_endpoint(&mut self, endpoint: &str) -> &mut Self {
        if !endpoint.is_empty() {
            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
            self.presign_endpoint = Some(endpoint.trim_end_matches('/').to_string())
        }

        self
    }

    /// Set access_key_id of this backend.
    ///
    /// - If access_key_id is set, we will take user's input first.
    /// - If not, we will try to load it from environment.
    pub fn access_key_id(&mut self, v: &str) -> &mut Self {
        if !v.is_empty() {
            self.access_key_id = Some(v.to_string())
        }

        self
    }

    /// Set access_key_secret of this backend.
    ///
    /// - If access_key_secret is set, we will take user's input first.
    /// - If not, we will try to load it from environment.
    pub fn access_key_secret(&mut self, v: &str) -> &mut Self {
        if !v.is_empty() {
            self.access_key_secret = Some(v.to_string())
        }

        self
    }

    /// Specify the http client that used by this service.
    ///
    /// # Notes
    ///
    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
    /// during minor updates.
    pub fn http_client(&mut self, client: HttpClient) -> &mut Self {
        self.http_client = Some(client);
        self
    }

    /// preprocess the endpoint option
    fn parse_endpoint(&self, endpoint: &Option<String>, bucket: &str) -> Result<(String, String)> {
        let (endpoint, host) = match endpoint.clone() {
            Some(ep) => {
                let uri = ep.parse::<Uri>().map_err(|err| {
                    Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
                        .with_context("service", Scheme::Oss)
                        .with_context("endpoint", &ep)
                        .set_source(err)
                })?;
                let host = uri.host().ok_or_else(|| {
                    Error::new(ErrorKind::ConfigInvalid, "endpoint host is empty")
                        .with_context("service", Scheme::Oss)
                        .with_context("endpoint", &ep)
                })?;
                let full_host = format!("{bucket}.{host}");
                let endpoint = match uri.scheme_str() {
                    Some(scheme_str) => match scheme_str {
                        "http" | "https" => format!("{scheme_str}://{full_host}"),
                        _ => {
                            return Err(Error::new(
                                ErrorKind::ConfigInvalid,
                                "endpoint protocol is invalid",
                            )
                            .with_context("service", Scheme::Oss));
                        }
                    },
                    None => format!("https://{full_host}"),
                };
                (endpoint, full_host)
            }
            None => {
                return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
                    .with_context("service", Scheme::Oss));
            }
        };
        Ok((endpoint, host))
    }

    /// Set server_side_encryption for this backend.
    ///
    /// Available values: `AES256`, `KMS`.
    ///
    /// Reference: <https://www.alibabacloud.com/help/en/object-storage-service/latest/server-side-encryption-5>
    /// Brief explanation:
    /// There are two server-side encryption methods available:
    /// SSE-AES256:
    ///     1. Configure the bucket encryption mode as OSS-managed and specify the encryption algorithm as AES256.
    ///     2. Include the `x-oss-server-side-encryption` parameter in the request and set its value to AES256.
    /// SSE-KMS:
    ///     1. To use this service, you need to first enable KMS.
    ///     2. Configure the bucket encryption mode as KMS, and specify the specific CMK ID for BYOK (Bring Your Own Key)
    ///        or not specify the specific CMK ID for OSS-managed KMS key.
    ///     3. Include the `x-oss-server-side-encryption` parameter in the request and set its value to KMS.
    ///     4. If a specific CMK ID is specified, include the `x-oss-server-side-encryption-key-id` parameter in the request, and set its value to the specified CMK ID.
    pub fn server_side_encryption(&mut self, v: &str) -> &mut Self {
        if !v.is_empty() {
            self.server_side_encryption = Some(v.to_string())
        }
        self
    }

    /// Set server_side_encryption_key_id for this backend.
    ///
    /// # Notes
    ///
    /// This option only takes effect when server_side_encryption equals to KMS.
    pub fn server_side_encryption_key_id(&mut self, v: &str) -> &mut Self {
        if !v.is_empty() {
            self.server_side_encryption_key_id = Some(v.to_string())
        }
        self
    }

    /// Set maximum batch operations of this backend.
    pub fn batch_max_operations(&mut self, batch_max_operations: usize) -> &mut Self {
        self.batch_max_operations = Some(batch_max_operations);

        self
    }

    /// Allow anonymous will allow opendal to send request without signing
    /// when credential is not loaded.
    pub fn allow_anonymous(&mut self) -> &mut Self {
        self.allow_anonymous = true;
        self
    }
}

impl Builder for OssBuilder {
    const SCHEME: Scheme = Scheme::Oss;
    type Accessor = OssBackend;

    fn from_map(map: HashMap<String, String>) -> Self {
        let mut builder = OssBuilder::default();

        map.get("root").map(|v| builder.root(v));
        map.get("bucket").map(|v| builder.bucket(v));
        map.get("endpoint").map(|v| builder.endpoint(v));
        map.get("presign_endpoint")
            .map(|v| builder.presign_endpoint(v));
        map.get("access_key_id").map(|v| builder.access_key_id(v));
        map.get("access_key_secret")
            .map(|v| builder.access_key_secret(v));
        map.get("server_side_encryption")
            .map(|v| builder.server_side_encryption(v));
        map.get("server_side_encryption_key_id")
            .map(|v| builder.server_side_encryption_key_id(v));
        map.get("batch_max_operations")
            .map(|v| builder.batch_max_operations(v.parse::<usize>().unwrap()));
        map.get("allow_anonymous")
            .filter(|v| *v == "on" || *v == "true")
            .map(|_| builder.allow_anonymous());

        builder
    }

    fn build(&mut self) -> Result<Self::Accessor> {
        debug!("backend build started: {:?}", &self);

        let root = normalize_root(&self.root.clone().unwrap_or_default());
        debug!("backend use root {}", &root);

        // Handle endpoint, region and bucket name.
        let bucket = match self.bucket.is_empty() {
            false => Ok(&self.bucket),
            true => Err(
                Error::new(ErrorKind::ConfigInvalid, "The bucket is misconfigured")
                    .with_context("service", Scheme::Oss),
            ),
        }?;

        let client = if let Some(client) = self.http_client.take() {
            client
        } else {
            HttpClient::new().map_err(|err| {
                err.with_operation("Builder::build")
                    .with_context("service", Scheme::Oss)
            })?
        };

        // Retrieve endpoint and host by parsing the endpoint option and bucket. If presign_endpoint is not
        // set, take endpoint as default presign_endpoint.
        let (endpoint, host) = self.parse_endpoint(&self.endpoint, bucket)?;
        debug!("backend use bucket {}, endpoint: {}", &bucket, &endpoint);

        let presign_endpoint = if self.presign_endpoint.is_some() {
            self.parse_endpoint(&self.presign_endpoint, bucket)?.0
        } else {
            endpoint.clone()
        };
        debug!("backend use presign_endpoint: {}", &presign_endpoint);

        let server_side_encryption = match &self.server_side_encryption {
            None => None,
            Some(v) => Some(
                build_header_value(v)
                    .map_err(|err| err.with_context("key", "server_side_encryption"))?,
            ),
        };

        let server_side_encryption_key_id = match &self.server_side_encryption_key_id {
            None => None,
            Some(v) => Some(
                build_header_value(v)
                    .map_err(|err| err.with_context("key", "server_side_encryption_key_id"))?,
            ),
        };

        let mut cfg = AliyunConfig::default();
        // Load cfg from env first.
        cfg = cfg.from_env();

        if let Some(v) = self.access_key_id.take() {
            cfg.access_key_id = Some(v);
        }

        if let Some(v) = self.access_key_secret.take() {
            cfg.access_key_secret = Some(v);
        }

        let loader = AliyunLoader::new(client.client(), cfg);

        let signer = AliyunOssSigner::new(bucket);

        let batch_max_operations = self
            .batch_max_operations
            .unwrap_or(DEFAULT_BATCH_MAX_OPERATIONS);
        debug!("Backend build finished");

        Ok(OssBackend {
            core: Arc::new(OssCore {
                root,
                bucket: bucket.to_owned(),
                endpoint,
                host,
                presign_endpoint,
                allow_anonymous: self.allow_anonymous,
                signer,
                loader,
                client,
                server_side_encryption,
                server_side_encryption_key_id,
                batch_max_operations,
            }),
        })
    }
}

#[derive(Debug, Clone)]
/// Aliyun Object Storage Service backend
pub struct OssBackend {
    core: Arc<OssCore>,
}

#[async_trait]
impl Accessor for OssBackend {
    type Reader = IncomingAsyncBody;
    type Writer = OssWriters;
    type Lister = oio::PageLister<OssLister>;
    type BlockingReader = ();
    type BlockingWriter = ();
    type BlockingLister = ();

    fn info(&self) -> AccessorInfo {
        let mut am = AccessorInfo::default();
        am.set_scheme(Scheme::Oss)
            .set_root(&self.core.root)
            .set_name(&self.core.bucket)
            .set_native_capability(Capability {
                stat: true,
                stat_with_if_match: true,
                stat_with_if_none_match: true,

                read: true,
                read_can_next: true,
                read_with_range: true,
                read_with_if_match: true,
                read_with_if_none_match: true,

                write: true,
                write_can_empty: true,
                write_can_append: true,
                write_can_multi: true,
                write_with_cache_control: true,
                write_with_content_type: true,
                write_with_content_disposition: true,
                // The min multipart size of OSS is 100 KiB.
                //
                // ref: <https://www.alibabacloud.com/help/en/oss/user-guide/multipart-upload-12>
                write_multi_min_size: Some(100 * 1024),
                // The max multipart size of OSS is 5 GiB.
                //
                // ref: <https://www.alibabacloud.com/help/en/oss/user-guide/multipart-upload-12>
                write_multi_max_size: if cfg!(target_pointer_width = "64") {
                    Some(5 * 1024 * 1024 * 1024)
                } else {
                    Some(usize::MAX)
                },

                delete: true,
                copy: true,

                list: true,
                list_with_limit: true,
                list_with_start_after: true,
                list_with_recursive: true,

                presign: true,
                presign_stat: true,
                presign_read: true,
                presign_write: true,

                batch: true,
                batch_max_operations: Some(self.core.batch_max_operations),

                ..Default::default()
            });

        am
    }

    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
        let resp = self
            .core
            .oss_head_object(path, args.if_match(), args.if_none_match())
            .await?;

        let status = resp.status();

        match status {
            StatusCode::OK => parse_into_metadata(path, resp.headers()).map(RpStat::new),
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
        let resp = self
            .core
            .oss_get_object(
                path,
                args.range(),
                args.if_match(),
                args.if_none_match(),
                args.override_content_disposition(),
            )
            .await?;

        let status = resp.status();

        match status {
            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
                let size = parse_content_length(resp.headers())?;
                let range = parse_content_range(resp.headers())?;
                Ok((
                    RpRead::new().with_size(size).with_range(range),
                    resp.into_body(),
                ))
            }
            StatusCode::RANGE_NOT_SATISFIABLE => {
                resp.into_body().consume().await?;
                Ok((RpRead::new().with_size(Some(0)), IncomingAsyncBody::empty()))
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        let writer = OssWriter::new(self.core.clone(), path, args.clone());

        let w = if args.append() {
            OssWriters::Two(oio::AppendWriter::new(writer))
        } else {
            OssWriters::One(oio::MultipartWriter::new(writer, args.concurrent()))
        };

        Ok((RpWrite::default(), w))
    }

    async fn delete(&self, path: &str, _: OpDelete) -> Result<RpDelete> {
        let resp = self.core.oss_delete_object(path).await?;
        let status = resp.status();
        match status {
            StatusCode::NO_CONTENT | StatusCode::NOT_FOUND => {
                resp.into_body().consume().await?;
                Ok(RpDelete::default())
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
        let l = OssLister::new(
            self.core.clone(),
            path,
            args.recursive(),
            args.limit(),
            args.start_after(),
        );
        Ok((RpList::default(), oio::PageLister::new(l)))
    }

    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
        let resp = self.core.oss_copy_object(from, to).await?;
        let status = resp.status();

        match status {
            StatusCode::OK => {
                resp.into_body().consume().await?;
                Ok(RpCopy::default())
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
        // We will not send this request out, just for signing.
        let mut req = match args.operation() {
            PresignOperation::Stat(v) => {
                self.core
                    .oss_head_object_request(path, true, v.if_match(), v.if_none_match())?
            }
            PresignOperation::Read(v) => self.core.oss_get_object_request(
                path,
                v.range(),
                true,
                v.if_match(),
                v.if_none_match(),
                v.override_content_disposition(),
            )?,
            PresignOperation::Write(v) => {
                self.core
                    .oss_put_object_request(path, None, v, AsyncBody::Empty, true)?
            }
        };

        self.core.sign_query(&mut req, args.expire()).await?;

        // We don't need this request anymore, consume it directly.
        let (parts, _) = req.into_parts();

        Ok(RpPresign::new(PresignedRequest::new(
            parts.method,
            parts.uri,
            parts.headers,
        )))
    }

    async fn batch(&self, args: OpBatch) -> Result<RpBatch> {
        let ops = args.into_operation();
        // Sadly, OSS will not return failed keys, so we will build
        // a set to calculate the failed keys.
        let mut keys = HashSet::new();

        let ops_len = ops.len();
        if ops_len > 1000 {
            return Err(Error::new(
                ErrorKind::Unsupported,
                "oss services only allow delete up to 1000 keys at once",
            )
            .with_context("length", ops_len.to_string()));
        }

        let paths = ops
            .into_iter()
            .map(|(p, _)| {
                keys.insert(p.clone());
                p
            })
            .collect();

        let resp = self.core.oss_delete_objects(paths).await?;

        let status = resp.status();

        if let StatusCode::OK = status {
            let bs = resp.into_body().bytes().await?;

            let result: DeleteObjectsResult =
                quick_xml::de::from_reader(bs.reader()).map_err(new_xml_deserialize_error)?;

            let mut batched_result = Vec::with_capacity(ops_len);
            for i in result.deleted {
                let path = build_rel_path(&self.core.root, &i.key);
                keys.remove(&path);
                batched_result.push((path, Ok(RpDelete::default().into())));
            }
            // TODO: we should handle those errors with code.
            for i in keys {
                batched_result.push((
                    i,
                    Err(Error::new(
                        ErrorKind::Unexpected,
                        "oss delete this key failed for reason we don't know",
                    )),
                ));
            }

            Ok(RpBatch::new(batched_result))
        } else {
            Err(parse_error(resp).await?)
        }
    }
}
