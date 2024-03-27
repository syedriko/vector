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

use async_trait::async_trait;
use dashmap::DashMap;
use serde::Deserialize;

use crate::raw::adapters::typed_kv;
use crate::raw::ConfigDeserializer;
use crate::*;

/// [dashmap](https://github.com/xacrimon/dashmap) backend support.
#[derive(Default, Deserialize, Clone, Debug)]
pub struct DashmapConfig {
    root: Option<String>,
}

/// [dashmap](https://github.com/xacrimon/dashmap) backend support.
#[doc = include_str!("docs.md")]
#[derive(Default)]
pub struct DashmapBuilder {
    config: DashmapConfig,
}

impl DashmapBuilder {
    /// Set the root for dashmap.
    pub fn root(&mut self, path: &str) -> &mut Self {
        self.config.root = Some(path.into());
        self
    }
}

impl Builder for DashmapBuilder {
    const SCHEME: Scheme = Scheme::Dashmap;
    type Accessor = DashmapBackend;

    fn from_map(map: HashMap<String, String>) -> Self {
        let config = DashmapConfig::deserialize(ConfigDeserializer::new(map))
            .expect("config deserialize must succeed");

        Self { config }
    }

    fn build(&mut self) -> Result<Self::Accessor> {
        Ok(DashmapBackend::new(Adapter {
            inner: DashMap::default(),
        })
        .with_root(self.config.root.as_deref().unwrap_or_default()))
    }
}

/// Backend is used to serve `Accessor` support in dashmap.
pub type DashmapBackend = typed_kv::Backend<Adapter>;

#[derive(Clone)]
pub struct Adapter {
    inner: DashMap<String, typed_kv::Value>,
}

impl Debug for Adapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DashmapAdapter")
            .field("size", &self.inner.len())
            .finish_non_exhaustive()
    }
}

#[async_trait]
impl typed_kv::Adapter for Adapter {
    fn info(&self) -> typed_kv::Info {
        typed_kv::Info::new(
            Scheme::Dashmap,
            &format!("{:?}", &self.inner as *const _),
            typed_kv::Capability {
                get: true,
                set: true,
                scan: true,
                delete: true,
            },
        )
    }

    async fn get(&self, path: &str) -> Result<Option<typed_kv::Value>> {
        self.blocking_get(path)
    }

    fn blocking_get(&self, path: &str) -> Result<Option<typed_kv::Value>> {
        match self.inner.get(path) {
            None => Ok(None),
            Some(bs) => Ok(Some(bs.value().to_owned())),
        }
    }

    async fn set(&self, path: &str, value: typed_kv::Value) -> Result<()> {
        self.blocking_set(path, value)
    }

    fn blocking_set(&self, path: &str, value: typed_kv::Value) -> Result<()> {
        self.inner.insert(path.to_string(), value);

        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<()> {
        self.blocking_delete(path)
    }

    fn blocking_delete(&self, path: &str) -> Result<()> {
        self.inner.remove(path);

        Ok(())
    }

    async fn scan(&self, path: &str) -> Result<Vec<String>> {
        self.blocking_scan(path)
    }

    fn blocking_scan(&self, path: &str) -> Result<Vec<String>> {
        let keys = self.inner.iter().map(|kv| kv.key().to_string());
        if path.is_empty() {
            Ok(keys.collect())
        } else {
            Ok(keys.filter(|k| k.starts_with(path) && k != path).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::*;

    #[test]
    fn test_accessor_metadata_name() {
        let b1 = DashmapBuilder::default().build().unwrap();
        assert_eq!(b1.info().name(), b1.info().name());

        let b2 = DashmapBuilder::default().build().unwrap();
        assert_ne!(b1.info().name(), b2.info().name())
    }
}
