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

use std::sync::Arc;

use async_trait::async_trait;
use http::StatusCode;

use super::core::GdriveCore;
use super::core::GdriveFileList;
use super::error::parse_error;
use crate::raw::*;
use crate::*;

pub struct GdriveLister {
    path: String,
    core: Arc<GdriveCore>,
}

impl GdriveLister {
    pub fn new(path: String, core: Arc<GdriveCore>) -> Self {
        Self { path, core }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl oio::PageList for GdriveLister {
    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
        let file_id = self.core.path_cache.get(&self.path).await?;

        let file_id = match file_id {
            Some(file_id) => file_id,
            None => {
                ctx.done = true;
                return Ok(());
            }
        };

        let resp = self
            .core
            .gdrive_list(file_id.as_str(), 100, &ctx.token)
            .await?;

        let bytes = match resp.status() {
            StatusCode::OK => resp.into_body().bytes().await?,
            _ => return Err(parse_error(resp).await?),
        };

        // Gdrive returns empty content when this dir is not exist.
        if bytes.is_empty() {
            ctx.done = true;
            return Ok(());
        }

        let decoded_response =
            serde_json::from_slice::<GdriveFileList>(&bytes).map_err(new_json_deserialize_error)?;

        if let Some(next_page_token) = decoded_response.next_page_token {
            ctx.token = next_page_token;
        } else {
            ctx.done = true;
        }

        for mut file in decoded_response.files {
            let file_type = if file.mime_type.as_str() == "application/vnd.google-apps.folder" {
                if !file.name.ends_with('/') {
                    file.name += "/";
                }
                EntryMode::DIR
            } else {
                EntryMode::FILE
            };

            let root = &self.core.root;
            let path = format!("{}{}", &self.path, file.name);
            let normalized_path = build_rel_path(root, &path);

            // Update path cache with list result.
            self.core.path_cache.insert(&path, &file.id).await;

            let entry = oio::Entry::new(&normalized_path, Metadata::new(file_type));
            ctx.entries.push_back(entry);
        }

        Ok(())
    }
}
