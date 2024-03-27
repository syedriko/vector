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

use super::core::constants::X_UPYUN_MULTI_UUID;
use super::core::UpyunCore;
use super::error::parse_error;
use crate::raw::*;
use crate::*;

pub type UpyunWriters = oio::MultipartWriter<UpyunWriter>;

pub struct UpyunWriter {
    core: Arc<UpyunCore>,
    op: OpWrite,
    path: String,
}

impl UpyunWriter {
    pub fn new(core: Arc<UpyunCore>, op: OpWrite, path: String) -> Self {
        UpyunWriter { core, op, path }
    }
}

#[async_trait]
impl oio::MultipartWrite for UpyunWriter {
    async fn write_once(&self, size: u64, body: AsyncBody) -> Result<()> {
        let req = self
            .core
            .upload(&self.path, Some(size), &self.op, body)
            .await?;

        let resp = self.core.send(req).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
                resp.into_body().consume().await?;
                Ok(())
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn initiate_part(&self) -> Result<String> {
        let resp = self
            .core
            .initiate_multipart_upload(&self.path, &self.op)
            .await?;

        let status = resp.status();

        match status {
            StatusCode::NO_CONTENT => {
                let id =
                    parse_header_to_str(resp.headers(), X_UPYUN_MULTI_UUID)?.ok_or(Error::new(
                        ErrorKind::Unexpected,
                        &format!("{} header is missing", X_UPYUN_MULTI_UUID),
                    ))?;

                Ok(id.to_string())
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn write_part(
        &self,
        upload_id: &str,
        part_number: usize,
        size: u64,
        body: AsyncBody,
    ) -> Result<oio::MultipartPart> {
        let req = self
            .core
            .upload_part(&self.path, upload_id, part_number, size, body)
            .await?;

        let resp = self.core.send(req).await?;

        let status = resp.status();

        match status {
            StatusCode::NO_CONTENT | StatusCode::CREATED => {
                resp.into_body().consume().await?;

                Ok(oio::MultipartPart {
                    part_number,
                    etag: "".to_string(),
                })
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn complete_part(&self, upload_id: &str, _parts: &[oio::MultipartPart]) -> Result<()> {
        let resp = self
            .core
            .complete_multipart_upload(&self.path, upload_id)
            .await?;

        let status = resp.status();

        match status {
            StatusCode::NO_CONTENT => {
                resp.into_body().consume().await?;

                Ok(())
            }
            _ => Err(parse_error(resp).await?),
        }
    }

    async fn abort_part(&self, _upload_id: &str) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Upyun does not support abort multipart upload",
        ))
    }
}
