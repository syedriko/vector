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

use super::core::*;
use super::error::parse_error;
use crate::raw::oio::Entry;
use crate::raw::*;
use crate::*;

pub struct PcloudLister {
    core: Arc<PcloudCore>,

    path: String,
}

impl PcloudLister {
    pub(super) fn new(core: Arc<PcloudCore>, path: &str) -> Self {
        PcloudLister {
            core,
            path: path.to_string(),
        }
    }
}

#[async_trait]
impl oio::PageList for PcloudLister {
    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
        let resp = self.core.list_folder(&self.path).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
                let bs = resp.into_body().bytes().await?;

                let resp: ListFolderResponse =
                    serde_json::from_slice(&bs).map_err(new_json_deserialize_error)?;
                let result = resp.result;

                if result == 2005 {
                    ctx.done = true;
                    return Ok(());
                }

                if result != 0 {
                    return Err(Error::new(ErrorKind::Unexpected, &format!("{resp:?}")));
                }

                if let Some(metadata) = resp.metadata {
                    if let Some(contents) = metadata.contents {
                        for content in contents {
                            let path = if content.isfolder {
                                format!("{}/", content.path.clone())
                            } else {
                                content.path.clone()
                            };

                            let md = parse_list_metadata(content)?;
                            let path = build_rel_path(&self.core.root, &path);

                            ctx.entries.push_back(Entry::new(&path, md))
                        }
                    }

                    ctx.done = true;
                    return Ok(());
                }

                return Err(Error::new(
                    ErrorKind::Unexpected,
                    &String::from_utf8_lossy(&bs),
                ));
            }
            _ => Err(parse_error(resp).await?),
        }
    }
}
