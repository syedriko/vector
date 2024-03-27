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

use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use async_trait::async_trait;
use openssh_sftp_client::file::File;
use openssh_sftp_client::file::TokioCompatFile;
use tokio::io::AsyncWrite;

use crate::raw::oio;
use crate::*;

pub struct SftpWriter {
    file: Pin<Box<TokioCompatFile>>,
}

impl SftpWriter {
    pub fn new(file: File) -> Self {
        SftpWriter {
            file: Box::pin(TokioCompatFile::new(file)),
        }
    }
}

#[async_trait]
impl oio::Write for SftpWriter {
    fn poll_write(&mut self, cx: &mut Context<'_>, bs: &dyn oio::WriteBuf) -> Poll<Result<usize>> {
        self.file
            .as_mut()
            .poll_write(cx, bs.chunk())
            .map_err(new_std_io_error)
    }

    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.file
            .as_mut()
            .poll_shutdown(cx)
            .map_err(new_std_io_error)
    }

    fn poll_abort(&mut self, _: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Err(Error::new(
            ErrorKind::Unsupported,
            "SftpWriter doesn't support abort",
        )))
    }
}

fn new_std_io_error(err: std::io::Error) -> Error {
    Error::new(ErrorKind::Unexpected, "read from sftp").set_source(err)
}
