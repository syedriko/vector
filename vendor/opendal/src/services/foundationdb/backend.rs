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
use foundationdb::api::NetworkAutoStop;
use foundationdb::Database;

use crate::raw::adapters::kv;
use crate::raw::normalize_root;
use crate::Builder;
use crate::Error;
use crate::ErrorKind;
use crate::Scheme;
use crate::*;

/// [foundationdb](https://www.foundationdb.org/) service support.
#[doc = include_str!("docs.md")]
#[derive(Default)]
pub struct FoundationdbBuilder {
    root: Option<String>,
    config_path: Option<String>,
}

impl FoundationdbBuilder {
    /// Set the root for Foundationdb.
    pub fn root(&mut self, path: &str) -> &mut Self {
        self.root = Some(path.into());
        self
    }

    /// Set the config path for Foundationdb. If not set, will fallback to use default
    pub fn config_path(&mut self, path: &str) -> &mut Self {
        self.config_path = Some(path.into());
        self
    }
}

impl Builder for FoundationdbBuilder {
    const SCHEME: Scheme = Scheme::Foundationdb;
    type Accessor = FoundationdbBackend;

    fn from_map(map: HashMap<String, String>) -> Self {
        let mut builder = FoundationdbBuilder::default();

        map.get("root").map(|v| builder.root(v));
        map.get("config_path").map(|v| builder.config_path(v));

        builder
    }

    fn build(&mut self) -> Result<Self::Accessor> {
        let _network = Arc::new(unsafe { foundationdb::boot() });
        let db;
        if let Some(cfg_path) = &self.config_path {
            db = Database::from_path(cfg_path).map_err(|e| {
                Error::new(ErrorKind::ConfigInvalid, "open foundation db")
                    .with_context("service", Scheme::Foundationdb)
                    .set_source(e)
            })?;
        } else {
            db = Database::default().map_err(|e| {
                Error::new(ErrorKind::ConfigInvalid, "open foundation db")
                    .with_context("service", Scheme::Foundationdb)
                    .set_source(e)
            })?
        }

        let db = Arc::new(db);

        let root = normalize_root(
            self.root
                .clone()
                .unwrap_or_else(|| "/".to_string())
                .as_str(),
        );

        Ok(FoundationdbBackend::new(Adapter { db, _network }).with_root(&root))
    }
}

/// Backend for Foundationdb services.
pub type FoundationdbBackend = kv::Backend<Adapter>;

#[derive(Clone)]
pub struct Adapter {
    db: Arc<Database>,
    _network: Arc<NetworkAutoStop>,
}

impl Debug for Adapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Adapter");
        ds.finish()
    }
}

#[async_trait]
impl kv::Adapter for Adapter {
    fn metadata(&self) -> kv::Metadata {
        kv::Metadata::new(
            Scheme::Foundationdb,
            "foundationdb",
            Capability {
                read: true,
                write: true,
                delete: true,
                ..Default::default()
            },
        )
    }

    async fn get(&self, path: &str) -> Result<Option<Vec<u8>>> {
        let transaction = self.db.create_trx().expect("Unable to create transaction");

        match transaction.get(path.as_bytes(), false).await {
            Ok(slice) => match slice {
                Some(data) => Ok(Some(data.to_vec())),
                None => Err(Error::new(
                    ErrorKind::NotFound,
                    "foundationdb: key not found",
                )),
            },
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                "foundationdb: key not found",
            )),
        }
    }

    async fn set(&self, path: &str, value: &[u8]) -> Result<()> {
        let transaction = self.db.create_trx().expect("Unable to create transaction");

        transaction.set(path.as_bytes(), value);

        match transaction.commit().await {
            Ok(_) => Ok(()),
            Err(e) => Err(parse_transaction_commit_error(e)),
        }
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let transaction = self.db.create_trx().expect("Unable to create transaction");
        transaction.clear(path.as_bytes());

        match transaction.commit().await {
            Ok(_) => Ok(()),
            Err(e) => Err(parse_transaction_commit_error(e)),
        }
    }
}

fn parse_transaction_commit_error(e: foundationdb::TransactionCommitError) -> Error {
    Error::new(ErrorKind::Unexpected, e.to_string().as_str())
        .with_context("service", Scheme::Foundationdb)
}
