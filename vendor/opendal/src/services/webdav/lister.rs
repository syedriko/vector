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
use std::sync::Arc;

use super::core::*;
use super::error::*;
use crate::raw::*;
use crate::*;

pub struct WebdavLister {
    core: Arc<WebdavCore>,

    path: String,
    args: OpList,
}

impl WebdavLister {
    pub fn new(core: Arc<WebdavCore>, path: &str, args: OpList) -> Self {
        Self {
            core,
            path: path.to_string(),
            args,
        }
    }
}

#[async_trait]
impl oio::PageList for WebdavLister {
    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
        let resp = self.core.webdav_list(&self.path, &self.args).await?;

        // jfrog artifactory's webdav services have some strange behavior.
        // We add this flag to check if the server is jfrog artifactory.
        //
        // Example: `"x-jfrog-version": "Artifactory/7.77.5 77705900"`
        let is_jfrog_artifactory = if let Some(v) = resp.headers().get("x-jfrog-version") {
            v.to_str().unwrap_or_default().starts_with("Artifactory")
        } else {
            false
        };

        let bs = if resp.status().is_success() {
            resp.into_body().bytes().await?
        } else if resp.status() == StatusCode::NOT_FOUND && self.path.ends_with('/') {
            ctx.done = true;
            return Ok(());
        } else {
            return Err(parse_error(resp).await?);
        };

        let result: Multistatus = deserialize_multistatus(&bs)?;

        for res in result.response {
            let mut path = res
                .href
                .strip_prefix(&self.core.server_path)
                .unwrap_or(&res.href)
                .to_string();

            let meta = parse_propstat(&res.propstat)?;

            // Append `/` to path if it's a dir
            if !path.ends_with('/') && meta.is_dir() {
                path += "/"
            }

            // Ignore the root path itself.
            if self.core.root == path {
                continue;
            }

            let normalized_path = build_rel_path(&self.core.root, &path);
            let decoded_path = percent_decode_path(&normalized_path);

            if normalized_path == self.path || decoded_path == self.path {
                // WebDAV server may return the current path as an entry.
                continue;
            }

            // HACKS! HACKS! HACKS!
            //
            // jfrog artifactory will generate a virtual checksum file for each file.
            // The checksum file can't be stated, but can be listed and read.
            // We ignore the checksum files to avoid listing unexpected files.
            if is_jfrog_artifactory && meta.content_type() == Some("application/x-checksum") {
                continue;
            }

            ctx.entries.push_back(oio::Entry::new(&decoded_path, meta))
        }
        ctx.done = true;

        Ok(())
    }
}
