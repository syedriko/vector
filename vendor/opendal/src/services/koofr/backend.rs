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
use http::StatusCode;
use log::debug;
use serde::Deserialize;
use tokio::sync::Mutex;
use tokio::sync::OnceCell;

use super::core::File;
use super::core::KoofrCore;
use super::core::KoofrSigner;
use super::error::parse_error;
use super::lister::KoofrLister;
use super::writer::KoofrWriter;
use super::writer::KoofrWriters;
use crate::raw::*;
use crate::*;

/// Config for backblaze Koofr services support.
#[derive(Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct KoofrConfig {
    /// root of this backend.
    ///
    /// All operations will happen under this root.
    pub root: Option<String>,
    /// Koofr endpoint.
    pub endpoint: String,
    /// Koofr email.
    pub email: String,
    /// password of this backend. (Must be the application password)
    pub password: Option<String>,
}

impl Debug for KoofrConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Config");

        ds.field("root", &self.root);
        ds.field("email", &self.email);

        ds.finish()
    }
}

/// [Koofr](https://app.koofr.net/) services support.
#[doc = include_str!("docs.md")]
#[derive(Default)]
pub struct KoofrBuilder {
    config: KoofrConfig,

    http_client: Option<HttpClient>,
}

impl Debug for KoofrBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("KoofrBuilder");

        d.field("config", &self.config);
        d.finish_non_exhaustive()
    }
}

impl KoofrBuilder {
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

    /// endpoint.
    ///
    /// It is required. e.g. `https://api.koofr.net/`
    pub fn endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.config.endpoint = endpoint.to_string();

        self
    }

    /// email.
    ///
    /// It is required. e.g. `test@example.com`
    pub fn email(&mut self, email: &str) -> &mut Self {
        self.config.email = email.to_string();

        self
    }

    /// Koofr application password.
    ///
    /// Go to https://app.koofr.net/app/admin/preferences/password.
    /// Click "Generate Password" button to generate a new application password.
    ///
    /// # Notes
    ///
    /// This is not user's Koofr account password.
    /// Please use the application password instead.
    /// Please also remind users of this.
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.config.password = if password.is_empty() {
            None
        } else {
            Some(password.to_string())
        };

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

impl Builder for KoofrBuilder {
    const SCHEME: Scheme = Scheme::Koofr;
    type Accessor = KoofrBackend;

    /// Converts a HashMap into an KoofrBuilder instance.
    ///
    /// # Arguments
    ///
    /// * `map` - A HashMap containing the configuration values.
    ///
    /// # Returns
    ///
    /// Returns an instance of KoofrBuilder.
    fn from_map(map: HashMap<String, String>) -> Self {
        // Deserialize the configuration from the HashMap.
        let config = KoofrConfig::deserialize(ConfigDeserializer::new(map))
            .expect("config deserialize must succeed");

        // Create an KoofrBuilder instance with the deserialized config.
        KoofrBuilder {
            config,
            http_client: None,
        }
    }

    /// Builds the backend and returns the result of KoofrBackend.
    fn build(&mut self) -> Result<Self::Accessor> {
        debug!("backend build started: {:?}", &self);

        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
        debug!("backend use root {}", &root);

        if self.config.endpoint.is_empty() {
            return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
                .with_operation("Builder::build")
                .with_context("service", Scheme::Koofr));
        }

        debug!("backend use endpoint {}", &self.config.endpoint);

        if self.config.email.is_empty() {
            return Err(Error::new(ErrorKind::ConfigInvalid, "email is empty")
                .with_operation("Builder::build")
                .with_context("service", Scheme::Koofr));
        }

        debug!("backend use email {}", &self.config.email);

        let password = match &self.config.password {
            Some(password) => Ok(password.clone()),
            None => Err(Error::new(ErrorKind::ConfigInvalid, "password is empty")
                .with_operation("Builder::build")
                .with_context("service", Scheme::Koofr)),
        }?;

        let client = if let Some(client) = self.http_client.take() {
            client
        } else {
            HttpClient::new().map_err(|err| {
                err.with_operation("Builder::build")
                    .with_context("service", Scheme::Koofr)
            })?
        };

        let signer = Arc::new(Mutex::new(KoofrSigner::default()));

        Ok(KoofrBackend {
            core: Arc::new(KoofrCore {
                root,
                endpoint: self.config.endpoint.clone(),
                email: self.config.email.clone(),
                password,
                mount_id: OnceCell::new(),
                signer,
                client,
            }),
        })
    }
}

/// Backend for Koofr services.
#[derive(Debug, Clone)]
pub struct KoofrBackend {
    core: Arc<KoofrCore>,
}

#[async_trait]
impl Accessor for KoofrBackend {
    type Reader = IncomingAsyncBody;
    type Writer = KoofrWriters;
    type Lister = oio::PageLister<KoofrLister>;
    type BlockingReader = ();
    type BlockingWriter = ();
    type BlockingLister = ();

    fn info(&self) -> AccessorInfo {
        let mut am = AccessorInfo::default();
        am.set_scheme(Scheme::Koofr)
            .set_root(&self.core.root)
            .set_native_capability(Capability {
                stat: true,

                create_dir: true,

                read: true,

                write: true,
                write_can_empty: true,

                delete: true,

                rename: true,

                copy: true,

                list: true,

                ..Default::default()
            });

        am
    }

    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
        self.core.ensure_dir_exists(path).await?;
        self.core
            .create_dir(&build_abs_path(&self.core.root, path))
            .await?;
        Ok(RpCreateDir::default())
    }

    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
        let path = build_rooted_abs_path(&self.core.root, path);
        let resp = self.core.info(&path).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
                let bs = resp.into_body().bytes().await?;

                let file: File = serde_json::from_slice(&bs).map_err(new_json_deserialize_error)?;

                let mode = if file.ty == "dir" {
                    EntryMode::DIR
                } else {
                    EntryMode::FILE
                };

                let mut md = Metadata::new(mode);

                md.set_content_length(file.size)
                    .set_content_type(&file.content_type)
                    .set_last_modified(parse_datetime_from_from_timestamp_millis(file.modified)?);

                Ok(RpStat::new(md))
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn read(&self, path: &str, _args: OpRead) -> Result<(RpRead, Self::Reader)> {
        let resp = self.core.get(path).await?;

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
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn write(&self, path: &str, _args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        let writer = KoofrWriter::new(self.core.clone(), path.to_string());

        let w = oio::OneShotWriter::new(writer);

        Ok((RpWrite::default(), w))
    }

    async fn delete(&self, path: &str, _: OpDelete) -> Result<RpDelete> {
        let resp = self.core.remove(path).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => Ok(RpDelete::default()),
            // Allow 404 when deleting a non-existing object
            StatusCode::NOT_FOUND => Ok(RpDelete::default()),
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
        let l = KoofrLister::new(self.core.clone(), path);
        Ok((RpList::default(), oio::PageLister::new(l)))
    }

    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
        self.core.ensure_dir_exists(to).await?;

        if from == to {
            return Ok(RpCopy::default());
        }

        let resp = self.core.remove(to).await?;

        let status = resp.status();

        if status != StatusCode::OK && status != StatusCode::NOT_FOUND {
            return Err(parse_error(resp).await?);
        }

        let resp = self.core.copy(from, to).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
                resp.into_body().consume().await?;

                Ok(RpCopy::default())
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
        self.core.ensure_dir_exists(to).await?;

        if from == to {
            return Ok(RpRename::default());
        }

        let resp = self.core.remove(to).await?;

        let status = resp.status();

        if status != StatusCode::OK && status != StatusCode::NOT_FOUND {
            return Err(parse_error(resp).await?);
        }

        let resp = self.core.move_object(from, to).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
                resp.into_body().consume().await?;

                Ok(RpRename::default())
            }
            _ => Err(parse_error(resp).await?),
        }
    }
}
