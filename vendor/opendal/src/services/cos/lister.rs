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
use bytes::Buf;
use quick_xml::de;

use super::core::*;
use super::error::parse_error;
use crate::raw::*;
use crate::EntryMode;
use crate::Metadata;
use crate::Result;

pub struct CosLister {
    core: Arc<CosCore>,
    path: String,
    delimiter: &'static str,
    limit: Option<usize>,
}

impl CosLister {
    pub fn new(core: Arc<CosCore>, path: &str, recursive: bool, limit: Option<usize>) -> Self {
        let delimiter = if recursive { "" } else { "/" };
        Self {
            core,
            path: path.to_string(),
            delimiter,
            limit,
        }
    }
}

#[async_trait]
impl oio::PageList for CosLister {
    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
        let resp = self
            .core
            .cos_list_objects(&self.path, &ctx.token, self.delimiter, self.limit)
            .await?;

        if resp.status() != http::StatusCode::OK {
            return Err(parse_error(resp).await?);
        }

        let bs = resp.into_body().bytes().await?;

        let output: ListObjectsOutput =
            de::from_reader(bs.reader()).map_err(new_xml_deserialize_error)?;

        // Try our best to check whether this list is done.
        //
        // - Check `next_marker`
        ctx.done = match output.next_marker.as_ref() {
            None => true,
            Some(next_marker) => next_marker.is_empty(),
        };
        ctx.token = output.next_marker.clone().unwrap_or_default();

        for prefix in output.common_prefixes {
            let de = oio::Entry::new(
                &build_rel_path(&self.core.root, &prefix.prefix),
                Metadata::new(EntryMode::DIR),
            );

            ctx.entries.push_back(de);
        }

        for object in output.contents {
            let path = build_rel_path(&self.core.root, &object.key);

            if path == self.path || path.is_empty() {
                continue;
            }

            let meta = Metadata::new(EntryMode::from_path(&path)).with_content_length(object.size);

            let de = oio::Entry::with(path, meta);
            ctx.entries.push_back(de);
        }

        Ok(())
    }
}
