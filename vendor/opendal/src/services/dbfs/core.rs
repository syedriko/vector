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

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bytes::Bytes;
use http::header;
use http::Request;
use http::Response;
use http::StatusCode;
use serde_json::json;

use super::error::parse_error;
use crate::raw::*;
use crate::*;

pub struct DbfsCore {
    pub root: String,
    pub endpoint: String,
    pub token: String,
    pub client: HttpClient,
}

impl Debug for DbfsCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbfsCore")
            .field("root", &self.root)
            .field("endpoint", &self.endpoint)
            .field("token", &self.token)
            .finish_non_exhaustive()
    }
}

impl DbfsCore {
    pub async fn dbfs_create_dir(&self, path: &str) -> Result<Response<IncomingAsyncBody>> {
        let url = format!("{}/api/2.0/dbfs/mkdirs", self.endpoint);
        let mut req = Request::post(&url);

        let auth_header_content = format!("Bearer {}", self.token);
        req = req.header(header::AUTHORIZATION, auth_header_content);

        let p = build_rooted_abs_path(&self.root, path)
            .trim_end_matches('/')
            .to_string();

        let req_body = &json!({
            "path": percent_encode_path(&p),
        });
        let body = AsyncBody::Bytes(Bytes::from(req_body.to_string()));

        let req = req.body(body).map_err(new_request_build_error)?;

        self.client.send(req).await
    }

    pub async fn dbfs_delete(&self, path: &str) -> Result<Response<IncomingAsyncBody>> {
        let url = format!("{}/api/2.0/dbfs/delete", self.endpoint);
        let mut req = Request::post(&url);

        let auth_header_content = format!("Bearer {}", self.token);
        req = req.header(header::AUTHORIZATION, auth_header_content);

        let p = build_rooted_abs_path(&self.root, path)
            .trim_end_matches('/')
            .to_string();

        let request_body = &json!({
            "path": percent_encode_path(&p),
            // TODO: support recursive toggle, should we add a new field in OpDelete?
            "recursive": true,
        });

        let body = AsyncBody::Bytes(Bytes::from(request_body.to_string()));

        let req = req.body(body).map_err(new_request_build_error)?;

        self.client.send(req).await
    }

    pub async fn dbfs_rename(&self, from: &str, to: &str) -> Result<Response<IncomingAsyncBody>> {
        let source = build_rooted_abs_path(&self.root, from);
        let target = build_rooted_abs_path(&self.root, to);

        let url = format!("{}/api/2.0/dbfs/move", self.endpoint);
        let mut req = Request::post(&url);

        let auth_header_content = format!("Bearer {}", self.token);
        req = req.header(header::AUTHORIZATION, auth_header_content);

        let req_body = &json!({
            "source_path": percent_encode_path(&source),
            "destination_path": percent_encode_path(&target),
        });

        let body = AsyncBody::Bytes(Bytes::from(req_body.to_string()));

        let req = req.body(body).map_err(new_request_build_error)?;

        self.client.send(req).await
    }

    pub async fn dbfs_list(&self, path: &str) -> Result<Response<IncomingAsyncBody>> {
        let p = build_rooted_abs_path(&self.root, path)
            .trim_end_matches('/')
            .to_string();

        let url = format!(
            "{}/api/2.0/dbfs/list?path={}",
            self.endpoint,
            percent_encode_path(&p)
        );
        let mut req = Request::get(&url);

        let auth_header_content = format!("Bearer {}", self.token);
        req = req.header(header::AUTHORIZATION, auth_header_content);

        let req = req
            .body(AsyncBody::Empty)
            .map_err(new_request_build_error)?;

        self.client.send(req).await
    }

    pub fn dbfs_create_file_request(&self, path: &str, body: Bytes) -> Result<Request<AsyncBody>> {
        let url = format!("{}/api/2.0/dbfs/put", self.endpoint);

        let contents = BASE64_STANDARD.encode(body);
        let mut req = Request::post(&url);

        let auth_header_content = format!("Bearer {}", self.token);
        req = req.header(header::AUTHORIZATION, auth_header_content);

        let req_body = &json!({
            "path": path,
            "contents": contents,
            "overwrite": true,
        });

        let body = AsyncBody::Bytes(Bytes::from(req_body.to_string()));

        req.body(body).map_err(new_request_build_error)
    }

    pub async fn dbfs_read(
        &self,
        path: &str,
        offset: u64,
        length: u64,
    ) -> Result<Response<IncomingAsyncBody>> {
        let p = build_rooted_abs_path(&self.root, path)
            .trim_end_matches('/')
            .to_string();

        let mut url = format!(
            "{}/api/2.0/dbfs/read?path={}",
            self.endpoint,
            percent_encode_path(&p)
        );

        if offset > 0 {
            url.push_str(&format!("&offset={}", offset));
        }

        if length > 0 {
            url.push_str(&format!("&length={}", length));
        }

        let mut req = Request::get(&url);

        let auth_header_content = format!("Bearer {}", self.token);
        req = req.header(header::AUTHORIZATION, auth_header_content);

        let req = req
            .body(AsyncBody::Empty)
            .map_err(new_request_build_error)?;

        let resp = self.client.send(req).await?;

        let status = resp.status();

        match status {
            StatusCode::OK => Ok(resp),
            _ => Err(parse_error(resp).await?),
        }
    }

    pub async fn dbfs_get_status(&self, path: &str) -> Result<Response<IncomingAsyncBody>> {
        let p = build_rooted_abs_path(&self.root, path)
            .trim_end_matches('/')
            .to_string();

        let url = format!(
            "{}/api/2.0/dbfs/get-status?path={}",
            &self.endpoint,
            percent_encode_path(&p)
        );

        let mut req = Request::get(&url);

        let auth_header_content = format!("Bearer {}", self.token);
        req = req.header(header::AUTHORIZATION, auth_header_content);

        let req = req
            .body(AsyncBody::Empty)
            .map_err(new_request_build_error)?;

        self.client.send(req).await
    }

    pub async fn dbfs_ensure_parent_path(&self, path: &str) -> Result<()> {
        let resp = self.dbfs_get_status(path).await?;

        match resp.status() {
            StatusCode::OK => return Ok(()),
            StatusCode::NOT_FOUND => {
                self.dbfs_create_dir(path).await?;
            }
            _ => return Err(parse_error(resp).await?),
        }
        Ok(())
    }
}
