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
use std::time::Duration;

use async_trait::async_trait;
use log::debug;
use moka::sync::CacheBuilder;
use moka::sync::SegmentedCache;
use serde::Deserialize;

use crate::raw::adapters::typed_kv;
use crate::raw::ConfigDeserializer;
use crate::*;

/// Config for Mokaservices support.
#[derive(Default, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct MokaConfig {
    /// Name for this cache instance.
    pub name: Option<String>,
    /// Sets the max capacity of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::max_capacity`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.max_capacity)
    pub max_capacity: Option<u64>,
    /// Sets the time to live of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::time_to_live`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.time_to_live)
    pub time_to_live: Option<Duration>,
    /// Sets the time to idle of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::time_to_idle`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.time_to_idle)
    pub time_to_idle: Option<Duration>,
    /// Sets the segments number of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::segments`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.segments)
    pub num_segments: Option<usize>,
}

impl Debug for MokaConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MokaConfig")
            .field("name", &self.name)
            .field("max_capacity", &self.max_capacity)
            .field("time_to_live", &self.time_to_live)
            .field("time_to_idle", &self.time_to_idle)
            .field("num_segments", &self.num_segments)
            .finish_non_exhaustive()
    }
}

/// [moka](https://github.com/moka-rs/moka) backend support.
#[doc = include_str!("docs.md")]
#[derive(Default, Debug)]
pub struct MokaBuilder {
    config: MokaConfig,
}

impl MokaBuilder {
    /// Name for this cache instance.
    pub fn name(&mut self, v: &str) -> &mut Self {
        if !v.is_empty() {
            self.config.name = Some(v.to_owned());
        }
        self
    }

    /// Sets the max capacity of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::max_capacity`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.max_capacity)
    pub fn max_capacity(&mut self, v: u64) -> &mut Self {
        if v != 0 {
            self.config.max_capacity = Some(v);
        }
        self
    }

    /// Sets the time to live of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::time_to_live`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.time_to_live)
    pub fn time_to_live(&mut self, v: Duration) -> &mut Self {
        if !v.is_zero() {
            self.config.time_to_live = Some(v);
        }
        self
    }

    /// Sets the time to idle of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::time_to_idle`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.time_to_idle)
    pub fn time_to_idle(&mut self, v: Duration) -> &mut Self {
        if !v.is_zero() {
            self.config.time_to_idle = Some(v);
        }
        self
    }

    /// Sets the segments number of the cache.
    ///
    /// Refer to [`moka::sync::CacheBuilder::segments`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.segments)
    pub fn segments(&mut self, v: usize) -> &mut Self {
        assert!(v != 0);
        self.config.num_segments = Some(v);
        self
    }
}

impl Builder for MokaBuilder {
    const SCHEME: Scheme = Scheme::Moka;
    type Accessor = MokaBackend;

    fn from_map(map: HashMap<String, String>) -> Self {
        MokaBuilder {
            config: MokaConfig::deserialize(ConfigDeserializer::new(map))
                .expect("config deserialize must succeed"),
        }
    }

    fn build(&mut self) -> Result<Self::Accessor> {
        debug!("backend build started: {:?}", &self);

        let mut builder: CacheBuilder<String, typed_kv::Value, _> =
            SegmentedCache::builder(self.config.num_segments.unwrap_or(1));
        // Use entries' bytes as capacity weigher.
        builder = builder.weigher(|k, v| (k.len() + v.size()) as u32);
        if let Some(v) = &self.config.name {
            builder = builder.name(v);
        }
        if let Some(v) = self.config.max_capacity {
            builder = builder.max_capacity(v)
        }
        if let Some(v) = self.config.time_to_live {
            builder = builder.time_to_live(v)
        }
        if let Some(v) = self.config.time_to_idle {
            builder = builder.time_to_idle(v)
        }

        debug!("backend build finished: {:?}", &self);
        Ok(MokaBackend::new(Adapter {
            inner: builder.build(),
        }))
    }
}

/// Backend is used to serve `Accessor` support in moka.
pub type MokaBackend = typed_kv::Backend<Adapter>;

#[derive(Clone)]
pub struct Adapter {
    inner: SegmentedCache<String, typed_kv::Value>,
}

impl Debug for Adapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Adapter")
            .field("size", &self.inner.weighted_size())
            .field("count", &self.inner.entry_count())
            .finish()
    }
}

#[async_trait]
impl typed_kv::Adapter for Adapter {
    fn info(&self) -> typed_kv::Info {
        typed_kv::Info::new(
            Scheme::Moka,
            self.inner.name().unwrap_or("moka"),
            typed_kv::Capability {
                get: true,
                set: true,
                delete: true,
                scan: true,
            },
        )
    }

    async fn get(&self, path: &str) -> Result<Option<typed_kv::Value>> {
        self.blocking_get(path)
    }

    fn blocking_get(&self, path: &str) -> Result<Option<typed_kv::Value>> {
        match self.inner.get(path) {
            None => Ok(None),
            Some(bs) => Ok(Some(bs)),
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
        self.inner.invalidate(path);

        Ok(())
    }

    async fn scan(&self, path: &str) -> Result<Vec<String>> {
        self.blocking_scan(path)
    }

    fn blocking_scan(&self, path: &str) -> Result<Vec<String>> {
        let keys = self.inner.iter().map(|kv| kv.0.to_string());
        if path.is_empty() {
            Ok(keys.collect())
        } else {
            Ok(keys.filter(|k| k.starts_with(path) && k != path).collect())
        }
    }
}
