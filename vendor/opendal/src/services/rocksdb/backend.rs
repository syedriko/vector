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
use std::sync::Arc;

use async_trait::async_trait;
use rocksdb::DB;
use serde::Deserialize;
use tokio::task;

use crate::raw::adapters::kv;
use crate::raw::*;
use crate::Result;
use crate::*;

#[derive(Default, Deserialize, Clone)]
#[serde(default)]
#[non_exhaustive]
/// Config for Rocksdb Service.
pub struct RocksdbConfig {
    /// The path to the rocksdb data directory.
    datadir: Option<String>,
    /// the working directory of the service. Can be "/path/to/dir"
    ///
    /// default is "/"
    root: Option<String>,
}

/// RocksDB service support.
#[doc = include_str!("docs.md")]
#[derive(Clone, Default)]
pub struct RocksdbBuilder {
    config: RocksdbConfig,
}

impl RocksdbBuilder {
    /// Set the path to the rocksdb data directory. Will create if not exists.
    pub fn datadir(&mut self, path: &str) -> &mut Self {
        self.config.datadir = Some(path.into());
        self
    }

    /// set the working directory, all operations will be performed under it.
    ///
    /// default: "/"
    pub fn root(&mut self, root: &str) -> &mut Self {
        if !root.is_empty() {
            self.config.root = Some(root.to_owned());
        }
        self
    }
}

impl Builder for RocksdbBuilder {
    const SCHEME: Scheme = Scheme::Rocksdb;
    type Accessor = RocksdbBackend;

    fn from_map(map: HashMap<String, String>) -> Self {
        let config = RocksdbConfig::deserialize(ConfigDeserializer::new(map))
            .expect("config deserialize must succeed");
        RocksdbBuilder { config }
    }

    fn build(&mut self) -> Result<Self::Accessor> {
        let path = self.config.datadir.take().ok_or_else(|| {
            Error::new(ErrorKind::ConfigInvalid, "datadir is required but not set")
                .with_context("service", Scheme::Rocksdb)
        })?;
        let db = DB::open_default(&path).map_err(|e| {
            Error::new(ErrorKind::ConfigInvalid, "open default transaction db")
                .with_context("service", Scheme::Rocksdb)
                .with_context("datadir", path)
                .set_source(e)
        })?;

        Ok(RocksdbBackend::new(Adapter { db: Arc::new(db) }))
    }
}

/// Backend for rocksdb services.
pub type RocksdbBackend = kv::Backend<Adapter>;

#[derive(Clone)]
pub struct Adapter {
    db: Arc<DB>,
}

impl Debug for Adapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Adapter");
        ds.field("path", &self.db.path());
        ds.finish()
    }
}

#[async_trait]
impl kv::Adapter for Adapter {
    fn metadata(&self) -> kv::Metadata {
        kv::Metadata::new(
            Scheme::Rocksdb,
            &self.db.path().to_string_lossy(),
            Capability {
                read: true,
                write: true,
                list: true,
                blocking: true,
                ..Default::default()
            },
        )
    }

    async fn get(&self, path: &str) -> Result<Option<Vec<u8>>> {
        let cloned_self = self.clone();
        let cloned_path = path.to_string();

        task::spawn_blocking(move || cloned_self.blocking_get(cloned_path.as_str()))
            .await
            .map_err(new_task_join_error)?
    }

    fn blocking_get(&self, path: &str) -> Result<Option<Vec<u8>>> {
        self.db.get(path).map_err(parse_rocksdb_error)
    }

    async fn set(&self, path: &str, value: &[u8]) -> Result<()> {
        let cloned_self = self.clone();
        let cloned_path = path.to_string();
        let cloned_value = value.to_vec();

        task::spawn_blocking(move || cloned_self.blocking_set(cloned_path.as_str(), &cloned_value))
            .await
            .map_err(new_task_join_error)?
    }

    fn blocking_set(&self, path: &str, value: &[u8]) -> Result<()> {
        self.db.put(path, value).map_err(parse_rocksdb_error)
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let cloned_self = self.clone();
        let cloned_path = path.to_string();

        task::spawn_blocking(move || cloned_self.blocking_delete(cloned_path.as_str()))
            .await
            .map_err(new_task_join_error)?
    }

    fn blocking_delete(&self, path: &str) -> Result<()> {
        self.db.delete(path).map_err(parse_rocksdb_error)
    }

    async fn scan(&self, path: &str) -> Result<Vec<String>> {
        let cloned_self = self.clone();
        let cloned_path = path.to_string();

        task::spawn_blocking(move || cloned_self.blocking_scan(cloned_path.as_str()))
            .await
            .map_err(new_task_join_error)?
    }

    /// TODO: we only need key here.
    fn blocking_scan(&self, path: &str) -> Result<Vec<String>> {
        let it = self.db.prefix_iterator(path).map(|r| r.map(|(k, _)| k));
        let mut res = Vec::default();

        for key in it {
            let key = key.map_err(parse_rocksdb_error)?;
            let key = String::from_utf8_lossy(&key);
            // FIXME: it's must a bug that rocksdb returns key that not start with path.
            if !key.starts_with(path) {
                continue;
            }
            // List should skip the path itself.
            if key == path {
                continue;
            }
            res.push(key.to_string());
        }

        Ok(res)
    }
}

fn parse_rocksdb_error(e: rocksdb::Error) -> Error {
    Error::new(ErrorKind::Unexpected, "got rocksdb error").set_source(e)
}
