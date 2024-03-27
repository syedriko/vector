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

use async_trait::async_trait;
use http::StatusCode;

use super::backend::WebhdfsBackend;
use super::error::parse_error;
use super::message::*;
use crate::raw::*;
use crate::*;

pub struct WebhdfsLister {
    backend: WebhdfsBackend,
    path: String,
}

impl WebhdfsLister {
    pub fn new(backend: WebhdfsBackend, path: &str) -> Self {
        Self {
            backend,
            path: path.to_string(),
        }
    }
}

#[async_trait]
impl oio::PageList for WebhdfsLister {
    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
        let file_status = if self.backend.disable_list_batch {
            let resp = self.backend.webhdfs_list_status_request(&self.path).await?;
            match resp.status() {
                StatusCode::OK => {
                    ctx.done = true;

                    let bs = resp.into_body().bytes().await?;
                    serde_json::from_slice::<FileStatusesWrapper>(&bs)
                        .map_err(new_json_deserialize_error)?
                        .file_statuses
                        .file_status
                }
                StatusCode::NOT_FOUND => {
                    ctx.done = true;
                    return Ok(());
                }
                _ => return Err(parse_error(resp).await?),
            }
        } else {
            let resp = self
                .backend
                .webhdfs_list_status_batch_request(&self.path, &ctx.token)
                .await?;
            match resp.status() {
                StatusCode::OK => {
                    let bs = resp.into_body().bytes().await?;
                    let directory_listing = serde_json::from_slice::<DirectoryListingWrapper>(&bs)
                        .map_err(new_json_deserialize_error)?
                        .directory_listing;
                    let file_statuses = directory_listing.partial_listing.file_statuses.file_status;

                    if directory_listing.remaining_entries == 0 {
                        ctx.done = true;
                    } else if !file_statuses.is_empty() {
                        ctx.token = file_statuses.last().unwrap().path_suffix.clone();
                    }

                    file_statuses
                }
                StatusCode::NOT_FOUND => {
                    ctx.done = true;
                    return Ok(());
                }
                _ => return Err(parse_error(resp).await?),
            }
        };

        for status in file_status {
            let mut path = if self.path.is_empty() {
                status.path_suffix.to_string()
            } else {
                format!("{}/{}", self.path, status.path_suffix)
            };

            let meta = match status.ty {
                FileStatusType::Directory => Metadata::new(EntryMode::DIR),
                FileStatusType::File => Metadata::new(EntryMode::FILE)
                    .with_content_length(status.length)
                    .with_last_modified(parse_datetime_from_from_timestamp_millis(
                        status.modification_time,
                    )?),
            };

            if meta.mode().is_file() {
                path = path.trim_end_matches('/').to_string();
            }
            if meta.mode().is_dir() {
                path += "/"
            }
            let entry = oio::Entry::new(&path, meta);
            ctx.entries.push_back(entry);
        }

        Ok(())
    }
}
