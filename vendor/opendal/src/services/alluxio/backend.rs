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
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use serde::Deserialize;

use super::core::AlluxioCore;
use super::lister::AlluxioLister;
use super::writer::AlluxioWriter;
use super::writer::AlluxioWriters;
use crate::raw::*;
use crate::*;

/// Config for alluxio services support.
#[derive(Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct AlluxioConfig {
    /// root of this backend.
    ///
    /// All operations will happen under this root.
    ///
    /// default to `/` if not set.
    pub root: Option<String>,
    /// endpoint of this backend.
    ///
    /// Endpoint must be full uri, mostly like `http://127.0.0.1:39999`.
    pub endpoint: Option<String>,
}

impl Debug for AlluxioConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AlluxioConfig");

        d.field("root", &self.root)
            .field("endpoint", &self.endpoint);

        d.finish_non_exhaustive()
    }
}

/// [Alluxio](https://www.alluxio.io/) services support.
#[doc = include_str!("docs.md")]
#[derive(Default)]
pub struct AlluxioBuilder {
    config: AlluxioConfig,

    http_client: Option<HttpClient>,
}

impl Debug for AlluxioBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AlluxioBuilder");

        d.field("config", &self.config);
        d.finish_non_exhaustive()
    }
}

impl AlluxioBuilder {
    /// Set root of this backend.
    ///
    /// All operations will happen under this root.
    pub fn root(&mut self, root: &str) -> &mut Self {
        self.config.root = if root.is_empty() {
            None
        } else {
            Some(root.to_string())
        };

        self
    }

    /// endpoint of this backend.
    ///
    /// Endpoint must be full uri, mostly like `http://127.0.0.1:39999`.
    pub fn endpoint(&mut self, endpoint: &str) -> &mut Self {
        if !endpoint.is_empty() {
            // Trim trailing `/` so that we can accept `http://127.0.0.1:39999/`
            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string())
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
}

impl Builder for AlluxioBuilder {
    const SCHEME: Scheme = Scheme::Alluxio;
    type Accessor = AlluxioBackend;

    /// Converts a HashMap into an AlluxioBuilder instance.
    ///
    /// # Arguments
    ///
    /// * `map` - A HashMap containing the configuration values.
    ///
    /// # Returns
    ///
    /// Returns an instance of AlluxioBuilder.
    fn from_map(map: HashMap<String, String>) -> Self {
        // Deserialize the configuration from the HashMap.
        let config = AlluxioConfig::deserialize(ConfigDeserializer::new(map))
            .expect("config deserialize must succeed");

        // Create an AlluxioBuilder instance with the deserialized config.
        AlluxioBuilder {
            config,
            http_client: None,
        }
    }

    /// Builds the backend and returns the result of AlluxioBackend.
    fn build(&mut self) -> Result<Self::Accessor> {
        debug!("backend build started: {:?}", &self);

        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
        debug!("backend use root {}", &root);

        let endpoint = match &self.config.endpoint {
            Some(endpoint) => Ok(endpoint.clone()),
            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
                .with_operation("Builder::build")
                .with_context("service", Scheme::Alluxio)),
        }?;
        debug!("backend use endpoint {}", &endpoint);

        let client = if let Some(client) = self.http_client.take() {
            client
        } else {
            HttpClient::new().map_err(|err| {
                err.with_operation("Builder::build")
                    .with_context("service", Scheme::Alluxio)
            })?
        };

        Ok(AlluxioBackend {
            core: Arc::new(AlluxioCore {
                root,
                endpoint,
                client,
            }),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AlluxioBackend {
    core: Arc<AlluxioCore>,
}

#[async_trait]
impl Accessor for AlluxioBackend {
    type Reader = IncomingAsyncBody;
    type Writer = AlluxioWriters;
    type Lister = oio::PageLister<AlluxioLister>;
    type BlockingReader = ();
    type BlockingWriter = ();
    type BlockingLister = ();

    fn info(&self) -> AccessorInfo {
        let mut am = AccessorInfo::default();
        am.set_scheme(Scheme::Alluxio)
            .set_root(&self.core.root)
            .set_native_capability(Capability {
                stat: true,

                read: true,

                write: true,
                write_can_multi: true,

                create_dir: true,
                delete: true,

                list: true,

                ..Default::default()
            });

        am
    }

    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
        self.core.create_dir(path).await?;
        Ok(RpCreateDir::default())
    }

    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
        let file_info = self.core.get_status(path).await?;

        Ok(RpStat::new(file_info.try_into()?))
    }

    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
        let stream_id = self.core.open_file(path).await?;

        let resp = self.core.read(stream_id, args.range()).await?;

        let size = parse_content_length(resp.headers())?;
        Ok((RpRead::new().with_size(size), resp.into_body()))
    }

    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        let w = AlluxioWriter::new(self.core.clone(), args.clone(), path.to_string());

        Ok((RpWrite::default(), w))
    }

    async fn delete(&self, path: &str, _: OpDelete) -> Result<RpDelete> {
        self.core.delete(path).await?;

        Ok(RpDelete::default())
    }

    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
        let l = AlluxioLister::new(self.core.clone(), path);
        Ok((RpList::default(), oio::PageLister::new(l)))
    }

    async fn rename(&self, from: &str, to: &str, _: OpRename) -> Result<RpRename> {
        self.core.rename(from, to).await?;

        Ok(RpRename::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_builder_from_map() {
        let mut map = HashMap::new();
        map.insert("root".to_string(), "/".to_string());
        map.insert("endpoint".to_string(), "http://127.0.0.1:39999".to_string());

        let builder = AlluxioBuilder::from_map(map);

        assert_eq!(builder.config.root, Some("/".to_string()));
        assert_eq!(
            builder.config.endpoint,
            Some("http://127.0.0.1:39999".to_string())
        );
    }

    #[test]
    fn test_builder_build() {
        let mut builder = AlluxioBuilder::default();
        builder.root("/root").endpoint("http://127.0.0.1:39999");

        let builder = builder.build();

        assert!(builder.is_ok());
    }
}
