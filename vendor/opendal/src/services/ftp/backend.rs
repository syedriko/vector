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

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;

use async_tls::TlsConnector;
use async_trait::async_trait;
use bb8::PooledConnection;
use bb8::RunError;
use futures::AsyncRead;
use futures::AsyncReadExt;
use http::Uri;
use log::debug;
use serde::Deserialize;
use suppaftp::list::File;
use suppaftp::types::FileType;
use suppaftp::types::Response;
use suppaftp::AsyncRustlsConnector;
use suppaftp::AsyncRustlsFtpStream;
use suppaftp::FtpError;
use suppaftp::ImplAsyncFtpStream;
use suppaftp::Status;
use tokio::sync::OnceCell;

use super::err::parse_error;
use super::lister::FtpLister;
use super::util::FtpReader;
use super::writer::FtpWriter;
use crate::raw::*;
use crate::services::ftp::writer::FtpWriters;
use crate::*;

/// Config for Ftpservices support.
#[derive(Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct FtpConfig {
    /// endpoint of this backend
    pub endpoint: Option<String>,
    /// root of this backend
    pub root: Option<String>,
    /// user of this backend
    pub user: Option<String>,
    /// password of this backend
    pub password: Option<String>,
}

impl Debug for FtpConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FtpConfig")
            .field("endpoint", &self.endpoint)
            .field("root", &self.root)
            .finish_non_exhaustive()
    }
}

/// FTP and FTPS services support.
#[doc = include_str!("docs.md")]
#[derive(Default)]
pub struct FtpBuilder {
    config: FtpConfig,
}

impl Debug for FtpBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FtpBuilder")
            .field("config", &self.config)
            .finish()
    }
}

impl FtpBuilder {
    /// set endpoint for ftp backend.
    pub fn endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.config.endpoint = if endpoint.is_empty() {
            None
        } else {
            Some(endpoint.to_string())
        };

        self
    }

    /// set root path for ftp backend.
    pub fn root(&mut self, root: &str) -> &mut Self {
        self.config.root = if root.is_empty() {
            None
        } else {
            Some(root.to_string())
        };

        self
    }

    /// set user for ftp backend.
    pub fn user(&mut self, user: &str) -> &mut Self {
        self.config.user = if user.is_empty() {
            None
        } else {
            Some(user.to_string())
        };

        self
    }

    /// set password for ftp backend.
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.config.password = if password.is_empty() {
            None
        } else {
            Some(password.to_string())
        };

        self
    }
}

impl Builder for FtpBuilder {
    const SCHEME: Scheme = Scheme::Ftp;
    type Accessor = FtpBackend;

    fn build(&mut self) -> Result<Self::Accessor> {
        debug!("ftp backend build started: {:?}", &self);
        let endpoint = match &self.config.endpoint {
            None => return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")),
            Some(v) => v,
        };

        let endpoint_uri = match endpoint.parse::<Uri>() {
            Err(e) => {
                return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
                    .with_context("endpoint", endpoint)
                    .set_source(e));
            }
            Ok(uri) => uri,
        };

        let host = endpoint_uri.host().unwrap_or("127.0.0.1");
        let port = endpoint_uri.port_u16().unwrap_or(21);

        let endpoint = format!("{host}:{port}");

        let enable_secure = match endpoint_uri.scheme_str() {
            Some("ftp") => false,
            // if the user forgot to add a scheme prefix
            // treat it as using secured scheme
            Some("ftps") | None => true,

            Some(s) => {
                return Err(Error::new(
                    ErrorKind::ConfigInvalid,
                    "endpoint is unsupported or invalid",
                )
                .with_context("endpoint", s));
            }
        };

        let root = normalize_root(&self.config.root.take().unwrap_or_default());

        let user = match &self.config.user {
            None => "".to_string(),
            Some(v) => v.clone(),
        };

        let password = match &self.config.password {
            None => "".to_string(),
            Some(v) => v.clone(),
        };

        debug!("ftp backend finished: {:?}", &self);

        Ok(FtpBackend {
            endpoint,
            root,
            user,
            password,
            enable_secure,
            pool: OnceCell::new(),
        })
    }

    fn from_map(map: HashMap<String, String>) -> Self {
        FtpBuilder {
            config: FtpConfig::deserialize(ConfigDeserializer::new(map))
                .expect("config deserialize must succeed"),
        }
    }
}

pub struct Manager {
    endpoint: String,
    root: String,
    user: String,
    password: String,
    enable_secure: bool,
}

#[async_trait]
impl bb8::ManageConnection for Manager {
    type Connection = AsyncRustlsFtpStream;
    type Error = FtpError;

    async fn connect(&self) -> std::result::Result<Self::Connection, Self::Error> {
        let stream = ImplAsyncFtpStream::connect(&self.endpoint).await?;
        // switch to secure mode if ssl/tls is on.
        let mut ftp_stream = if self.enable_secure {
            stream
                .into_secure(
                    AsyncRustlsConnector::from(TlsConnector::default()),
                    &self.endpoint,
                )
                .await?
        } else {
            stream
        };

        // login if needed
        if !self.user.is_empty() {
            ftp_stream.login(&self.user, &self.password).await?;
        }

        // change to the root path
        match ftp_stream.cwd(&self.root).await {
            Err(FtpError::UnexpectedResponse(e)) if e.status == Status::FileUnavailable => {
                ftp_stream.mkdir(&self.root).await?;
                // Then change to root path
                ftp_stream.cwd(&self.root).await?;
            }
            // Other errors, return.
            Err(e) => return Err(e),
            // Do nothing if success.
            Ok(_) => (),
        }

        ftp_stream.transfer_type(FileType::Binary).await?;

        Ok(ftp_stream)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> std::result::Result<(), Self::Error> {
        conn.noop().await
    }

    /// Don't allow reuse conn.
    ///
    /// We need to investigate why reuse conn will cause error.
    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        true
    }
}

/// Backend is used to serve `Accessor` support for ftp.
#[derive(Clone)]
pub struct FtpBackend {
    endpoint: String,
    root: String,
    user: String,
    password: String,
    enable_secure: bool,
    pool: OnceCell<bb8::Pool<Manager>>,
}

impl Debug for FtpBackend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Backend").finish()
    }
}

#[async_trait]
impl Accessor for FtpBackend {
    type Reader = FtpReader;
    type Writer = FtpWriters;
    type Lister = FtpLister;
    type BlockingReader = ();
    type BlockingWriter = ();
    type BlockingLister = ();

    fn info(&self) -> AccessorInfo {
        let mut am = AccessorInfo::default();
        am.set_scheme(Scheme::Ftp)
            .set_root(&self.root)
            .set_native_capability(Capability {
                stat: true,

                read: true,
                read_with_range: true,

                write: true,

                delete: true,
                create_dir: true,

                list: true,

                ..Default::default()
            });

        am
    }

    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
        let mut ftp_stream = self.ftp_connect(Operation::CreateDir).await?;

        let paths: Vec<&str> = path.split_inclusive('/').collect();

        let mut curr_path = String::new();

        for path in paths {
            curr_path.push_str(path);
            match ftp_stream.mkdir(&curr_path).await {
                // Do nothing if status is FileUnavailable or OK(()) is return.
                Err(FtpError::UnexpectedResponse(Response {
                    status: Status::FileUnavailable,
                    ..
                }))
                | Ok(()) => (),
                Err(e) => {
                    return Err(parse_error(e));
                }
            }
        }

        return Ok(RpCreateDir::default());
    }

    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
        let file = self.ftp_stat(path).await?;

        let mode = if file.is_file() {
            EntryMode::FILE
        } else if file.is_directory() {
            EntryMode::DIR
        } else {
            EntryMode::Unknown
        };

        let mut meta = Metadata::new(mode);
        meta.set_content_length(file.size() as u64);
        meta.set_last_modified(file.modified().into());

        Ok(RpStat::new(meta))
    }

    /// TODO: migrate to FileReader maybe?
    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
        let mut ftp_stream = self.ftp_connect(Operation::Read).await?;

        let meta = self.ftp_stat(path).await?;

        let br = args.range();
        let r: Box<dyn AsyncRead + Send + Unpin> = match (br.offset(), br.size()) {
            (Some(offset), Some(size)) => {
                ftp_stream
                    .resume_transfer(offset as usize)
                    .await
                    .map_err(parse_error)?;
                let ds = ftp_stream
                    .retr_as_stream(path)
                    .await
                    .map_err(parse_error)?
                    .take(size);
                Box::new(ds)
            }
            (Some(offset), None) => {
                ftp_stream
                    .resume_transfer(offset as usize)
                    .await
                    .map_err(parse_error)?;
                let ds = ftp_stream.retr_as_stream(path).await.map_err(parse_error)?;
                Box::new(ds)
            }
            (None, Some(size)) => {
                ftp_stream
                    .resume_transfer((meta.size() as u64 - size) as usize)
                    .await
                    .map_err(parse_error)?;
                let ds = ftp_stream.retr_as_stream(path).await.map_err(parse_error)?;
                Box::new(ds)
            }
            (None, None) => {
                let ds = ftp_stream.retr_as_stream(path).await.map_err(parse_error)?;
                Box::new(ds)
            }
        };

        Ok((RpRead::new(), FtpReader::new(r, ftp_stream)))
    }

    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        // Ensure the parent dir exists.
        let parent = get_parent(path);
        let paths: Vec<&str> = parent.split('/').collect();

        // TODO: we can optimize this by checking dir existence first.
        let mut ftp_stream = self.ftp_connect(Operation::Write).await?;
        let mut curr_path = String::new();

        for path in paths {
            curr_path.push_str(path);
            match ftp_stream.mkdir(&curr_path).await {
                // Do nothing if status is FileUnavailable or OK(()) is return.
                Err(FtpError::UnexpectedResponse(Response {
                    status: Status::FileUnavailable,
                    ..
                }))
                | Ok(()) => (),
                Err(e) => {
                    return Err(parse_error(e));
                }
            }
        }

        let w = FtpWriter::new(self.clone(), path.to_string());
        let w = oio::OneShotWriter::new(w);

        Ok((RpWrite::new(), w))
    }

    async fn delete(&self, path: &str, _: OpDelete) -> Result<RpDelete> {
        let mut ftp_stream = self.ftp_connect(Operation::Delete).await?;

        let result = if path.ends_with('/') {
            ftp_stream.rmdir(&path).await
        } else {
            ftp_stream.rm(&path).await
        };

        match result {
            Err(FtpError::UnexpectedResponse(Response {
                status: Status::FileUnavailable,
                ..
            }))
            | Ok(_) => (),
            Err(e) => {
                return Err(parse_error(e));
            }
        }

        Ok(RpDelete::default())
    }

    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
        let mut ftp_stream = self.ftp_connect(Operation::List).await?;

        let pathname = if path == "/" { None } else { Some(path) };
        let files = ftp_stream.list(pathname).await.map_err(parse_error)?;

        Ok((
            RpList::default(),
            FtpLister::new(if path == "/" { "" } else { path }, files),
        ))
    }
}

impl FtpBackend {
    pub async fn ftp_connect(&self, _: Operation) -> Result<PooledConnection<'static, Manager>> {
        let pool = self
            .pool
            .get_or_try_init(|| async {
                bb8::Pool::builder()
                    .max_size(64)
                    .build(Manager {
                        endpoint: self.endpoint.to_string(),
                        root: self.root.to_string(),
                        user: self.user.to_string(),
                        password: self.password.to_string(),
                        enable_secure: self.enable_secure,
                    })
                    .await
            })
            .await
            .map_err(parse_error)?;

        pool.get_owned().await.map_err(|err| match err {
            RunError::User(err) => parse_error(err),
            RunError::TimedOut => {
                Error::new(ErrorKind::Unexpected, "connection request: timeout").set_temporary()
            }
        })
    }

    async fn ftp_stat(&self, path: &str) -> Result<File> {
        let mut ftp_stream = self.ftp_connect(Operation::Stat).await?;

        let (parent, basename) = (get_parent(path), get_basename(path));

        let pathname = if parent == "/" { None } else { Some(parent) };

        let resp = ftp_stream.list(pathname).await.map_err(parse_error)?;

        // Get stat of file.
        let mut files = resp
            .into_iter()
            .filter_map(|file| File::from_str(file.as_str()).ok())
            .filter(|f| f.name() == basename.trim_end_matches('/'))
            .collect::<Vec<File>>();

        if files.is_empty() {
            Err(Error::new(
                ErrorKind::NotFound,
                "file is not found during list",
            ))
        } else {
            Ok(files.remove(0))
        }
    }
}

#[cfg(test)]
mod build_test {
    use super::FtpBuilder;
    use crate::*;

    #[test]
    fn test_build() {
        // ftps scheme, should suffix with default port 21
        let mut builder = FtpBuilder::default();
        builder.endpoint("ftps://ftp_server.local");
        let b = builder.build();
        assert!(b.is_ok());

        // ftp scheme
        let mut builder = FtpBuilder::default();
        builder.endpoint("ftp://ftp_server.local:1234");
        let b = builder.build();
        assert!(b.is_ok());

        // no scheme
        let mut builder = FtpBuilder::default();
        builder.endpoint("ftp_server.local:8765");
        let b = builder.build();
        assert!(b.is_ok());

        // invalid scheme
        let mut builder = FtpBuilder::default();
        builder.endpoint("invalidscheme://ftp_server.local:8765");
        let b = builder.build();
        assert!(b.is_err());
        let e = b.unwrap_err();
        assert_eq!(e.kind(), ErrorKind::ConfigInvalid);
    }
}
