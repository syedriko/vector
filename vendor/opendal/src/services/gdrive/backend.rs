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

use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use http::Request;
use http::StatusCode;
use serde_json::json;

use super::core::GdriveCore;
use super::error::parse_error;
use super::lister::GdriveLister;
use super::writer::GdriveWriter;
use crate::raw::*;
use crate::services::gdrive::core::GdriveFile;
use crate::types::Result;
use crate::*;

#[derive(Clone, Debug)]
pub struct GdriveBackend {
    pub core: Arc<GdriveCore>,
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Accessor for GdriveBackend {
    type Reader = IncomingAsyncBody;
    type Writer = oio::OneShotWriter<GdriveWriter>;
    type Lister = oio::PageLister<GdriveLister>;
    type BlockingReader = ();
    type BlockingWriter = ();
    type BlockingLister = ();

    fn info(&self) -> AccessorInfo {
        let mut ma = AccessorInfo::default();
        ma.set_scheme(Scheme::Gdrive)
            .set_root(&self.core.root)
            .set_native_capability(Capability {
                stat: true,

                read: true,

                list: true,

                write: true,

                create_dir: true,
                delete: true,
                rename: true,
                copy: true,
                ..Default::default()
            });

        ma
    }

    async fn create_dir(&self, path: &str, _args: OpCreateDir) -> Result<RpCreateDir> {
        let path = build_abs_path(&self.core.root, path);
        let _ = self.core.path_cache.ensure_dir(&path).await?;

        Ok(RpCreateDir::default())
    }

    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
        let resp = self.core.gdrive_stat(path).await?;

        if resp.status() != StatusCode::OK {
            return Err(parse_error(resp).await?);
        }

        let bs = resp.into_body().bytes().await?;
        let gdrive_file: GdriveFile =
            serde_json::from_slice(&bs).map_err(new_json_deserialize_error)?;

        if gdrive_file.mime_type == "application/vnd.google-apps.folder" {
            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
        };

        let mut meta = Metadata::new(EntryMode::FILE);
        if let Some(v) = gdrive_file.size {
            meta = meta.with_content_length(v.parse::<u64>().map_err(|e| {
                Error::new(ErrorKind::Unexpected, "parse content length").set_source(e)
            })?);
        }
        if let Some(v) = gdrive_file.modified_time {
            meta = meta.with_last_modified(v.parse::<chrono::DateTime<Utc>>().map_err(|e| {
                Error::new(ErrorKind::Unexpected, "parse last modified time").set_source(e)
            })?);
        }
        Ok(RpStat::new(meta))
    }

    async fn read(&self, path: &str, _args: OpRead) -> Result<(RpRead, Self::Reader)> {
        let resp = self.core.gdrive_get(path).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
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

    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        let path = build_abs_path(&self.core.root, path);

        // As Google Drive allows files have the same name, we need to check if the file exists.
        // If the file exists, we will keep its ID and update it.
        let file_id = self.core.path_cache.get(&path).await?;

        Ok((
            RpWrite::default(),
            oio::OneShotWriter::new(GdriveWriter::new(self.core.clone(), path, file_id)),
        ))
    }

    async fn delete(&self, path: &str, _: OpDelete) -> Result<RpDelete> {
        let path = build_abs_path(&self.core.root, path);
        let file_id = self.core.path_cache.get(&path).await?;
        let file_id = if let Some(id) = file_id {
            id
        } else {
            return Ok(RpDelete::default());
        };

        let resp = self.core.gdrive_trash(&file_id).await?;
        let status = resp.status();
        if status != StatusCode::OK {
            return Err(parse_error(resp).await?);
        }

        self.core.path_cache.remove(&path).await;
        resp.into_body().consume().await?;
        return Ok(RpDelete::default());
    }

    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
        let path = build_abs_path(&self.core.root, path);
        let l = GdriveLister::new(path, self.core.clone());
        Ok((RpList::default(), oio::PageLister::new(l)))
    }

    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
        let from = build_abs_path(&self.core.root, from);

        let from_file_id = self.core.path_cache.get(&from).await?.ok_or(Error::new(
            ErrorKind::NotFound,
            "the file to copy does not exist",
        ))?;

        let to_name = get_basename(to);
        let to_path = build_abs_path(&self.core.root, to);
        let to_parent_id = self
            .core
            .path_cache
            .ensure_dir(get_parent(&to_path))
            .await?;

        // copy will overwrite `to`, delete it if exist
        if let Some(id) = self.core.path_cache.get(&to_path).await? {
            let resp = self.core.gdrive_trash(&id).await?;
            let status = resp.status();
            if status != StatusCode::OK {
                return Err(parse_error(resp).await?);
            }

            self.core.path_cache.remove(&to_path).await;
            resp.into_body().consume().await?;
        }

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/copy",
            from_file_id
        );

        let request_body = &json!({
            "name": to_name,
            "parents": [to_parent_id],
        });
        let body = AsyncBody::Bytes(Bytes::from(request_body.to_string()));

        let mut req = Request::post(&url)
            .body(body)
            .map_err(new_request_build_error)?;
        self.core.sign(&mut req).await?;

        let resp = self.core.client.send(req).await?;

        match resp.status() {
            StatusCode::OK => Ok(RpCopy::default()),
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
        let source = build_abs_path(&self.core.root, from);
        let target = build_abs_path(&self.core.root, to);

        // rename will overwrite `to`, delete it if exist
        if let Some(id) = self.core.path_cache.get(&target).await? {
            let resp = self.core.gdrive_trash(&id).await?;
            let status = resp.status();
            if status != StatusCode::OK {
                return Err(parse_error(resp).await?);
            }

            self.core.path_cache.remove(&target).await;
            resp.into_body().consume().await?;
        }

        let resp = self
            .core
            .gdrive_patch_metadata_request(&source, &target)
            .await?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
                let body = resp.into_body().bytes().await?;
                let meta = serde_json::from_slice::<GdriveFile>(&body)
                    .map_err(new_json_deserialize_error)?;

                let cache = &self.core.path_cache;

                cache.remove(&build_abs_path(&self.core.root, from)).await;
                cache
                    .insert(&build_abs_path(&self.core.root, to), &meta.id)
                    .await;

                Ok(RpRename::default())
            }
            _ => Err(parse_error(resp).await?),
        }
    }
}
