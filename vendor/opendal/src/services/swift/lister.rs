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

use super::core::*;
use super::error::parse_error;
use crate::raw::*;
use crate::*;

pub struct SwiftLister {
    core: Arc<SwiftCore>,
    path: String,
    delimiter: &'static str,
    limit: Option<usize>,
}

impl SwiftLister {
    pub fn new(core: Arc<SwiftCore>, path: String, recursive: bool, limit: Option<usize>) -> Self {
        let delimiter = if recursive { "" } else { "/" };
        Self {
            core,
            path,
            delimiter,
            limit,
        }
    }
}

#[async_trait]
impl oio::PageList for SwiftLister {
    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
        let response = self
            .core
            .swift_list(&self.path, self.delimiter, self.limit, &ctx.token)
            .await?;

        let status_code = response.status();

        if !status_code.is_success() {
            let error = parse_error(response).await?;
            return Err(error);
        }

        let bytes = response.into_body().bytes().await?;
        let decoded_response: Vec<ListOpResponse> =
            serde_json::from_slice(&bytes).map_err(new_json_deserialize_error)?;

        // Update token and done based on resp.
        if let Some(entry) = decoded_response.last() {
            let path = match entry {
                ListOpResponse::Subdir { subdir } => subdir,
                ListOpResponse::FileInfo { name, .. } => name,
            };
            ctx.token = path.clone();
        } else {
            ctx.done = true;
        }

        for status in decoded_response {
            let entry: oio::Entry = match status {
                ListOpResponse::Subdir { subdir } => {
                    let meta = Metadata::new(EntryMode::DIR);
                    oio::Entry::new(&subdir, meta)
                }
                ListOpResponse::FileInfo {
                    bytes,
                    hash,
                    name,
                    content_type,
                    mut last_modified,
                } => {
                    // this is the pseudo directory itself; we'll skip it.
                    if name == self.path {
                        continue;
                    }

                    let mut meta = Metadata::new(EntryMode::from_path(&name));
                    meta.set_content_length(bytes);
                    meta.set_content_md5(hash.as_str());

                    // we'll change "2023-10-28T19:18:11.682610" to "2023-10-28T19:18:11.682610Z"
                    last_modified.push('Z');
                    meta.set_last_modified(parse_datetime_from_rfc3339(last_modified.as_str())?);
                    meta.set_content_type(content_type.as_str());

                    oio::Entry::with(name, meta)
                }
            };
            ctx.entries.push_back(entry);
        }
        Ok(())
    }
}
