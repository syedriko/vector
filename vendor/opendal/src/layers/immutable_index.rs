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

use std::collections::HashSet;
use std::fmt::Debug;
use std::task::Context;
use std::task::Poll;
use std::vec::IntoIter;

use async_trait::async_trait;

use crate::raw::*;
use crate::*;

/// Add an immutable in-memory index for underlying storage services.
///
/// Especially useful for services without list capability like HTTP.
///
/// # Examples
///
/// ```rust, no_run
/// use std::collections::HashMap;
///
/// use opendal::layers::ImmutableIndexLayer;
/// use opendal::services;
/// use opendal::Operator;
///
/// let mut iil = ImmutableIndexLayer::default();
///
/// for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
///     iil.insert(i.to_string())
/// }
///
/// let op = Operator::from_map::<services::Http>(HashMap::default())
///     .unwrap()
///     .layer(iil)
///     .finish();
/// ```
#[derive(Default, Debug, Clone)]
pub struct ImmutableIndexLayer {
    vec: Vec<String>,
}

impl ImmutableIndexLayer {
    /// Insert a key into index.
    pub fn insert(&mut self, key: String) {
        self.vec.push(key);
    }

    /// Insert keys from iter.
    pub fn extend_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.vec.extend(iter);
    }
}

impl<A: Accessor> Layer<A> for ImmutableIndexLayer {
    type LayeredAccessor = ImmutableIndexAccessor<A>;

    fn layer(&self, inner: A) -> Self::LayeredAccessor {
        ImmutableIndexAccessor {
            vec: self.vec.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImmutableIndexAccessor<A: Accessor> {
    inner: A,
    vec: Vec<String>,
}

impl<A: Accessor> ImmutableIndexAccessor<A> {
    fn children_flat(&self, path: &str) -> Vec<String> {
        self.vec
            .iter()
            .filter(|v| v.starts_with(path) && v.as_str() != path)
            .cloned()
            .collect()
    }

    fn children_hierarchy(&self, path: &str) -> Vec<String> {
        let mut res = HashSet::new();

        for i in self.vec.iter() {
            // `/xyz` should not belong to `/abc`
            if !i.starts_with(path) {
                continue;
            }

            // remove `/abc` if self
            if i == path {
                continue;
            }

            match i[path.len()..].find('/') {
                // File `/abc/def.csv` must belong to `/abc`
                None => {
                    res.insert(i.to_string());
                }
                Some(idx) => {
                    // The index of first `/` after `/abc`.
                    let dir_idx = idx + 1 + path.len();

                    if dir_idx == i.len() {
                        // Dir `/abc/def/` belongs to `/abc/`
                        res.insert(i.to_string());
                    } else {
                        // File/Dir `/abc/def/xyz` doesn't belong to `/abc`.
                        // But we need to list `/abc/def` out so that we can walk down.
                        res.insert(i[..dir_idx].to_string());
                    }
                }
            }
        }

        res.into_iter().collect()
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A: Accessor> LayeredAccessor for ImmutableIndexAccessor<A> {
    type Inner = A;
    type Reader = A::Reader;
    type BlockingReader = A::BlockingReader;
    type Writer = A::Writer;
    type BlockingWriter = A::BlockingWriter;
    type Lister = ImmutableDir;
    type BlockingLister = ImmutableDir;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    /// Add list capabilities for underlying storage services.
    fn metadata(&self) -> AccessorInfo {
        let mut meta = self.inner.info();

        let cap = meta.full_capability_mut();
        cap.list = true;
        cap.list_with_recursive = true;

        meta
    }

    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
        self.inner.read(path, args).await
    }

    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
        let mut path = path;
        if path == "/" {
            path = ""
        }

        let idx = if args.recursive() {
            self.children_flat(path)
        } else {
            self.children_hierarchy(path)
        };

        Ok((RpList::default(), ImmutableDir::new(idx)))
    }

    fn blocking_read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::BlockingReader)> {
        self.inner.blocking_read(path, args)
    }

    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        self.inner.write(path, args).await
    }

    fn blocking_write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::BlockingWriter)> {
        self.inner.blocking_write(path, args)
    }

    fn blocking_list(&self, path: &str, args: OpList) -> Result<(RpList, Self::BlockingLister)> {
        let mut path = path;
        if path == "/" {
            path = ""
        }

        let idx = if args.recursive() {
            self.children_flat(path)
        } else {
            self.children_hierarchy(path)
        };

        Ok((RpList::default(), ImmutableDir::new(idx)))
    }
}

pub struct ImmutableDir {
    idx: IntoIter<String>,
}

impl ImmutableDir {
    fn new(idx: Vec<String>) -> Self {
        Self {
            idx: idx.into_iter(),
        }
    }

    fn inner_next(&mut self) -> Option<oio::Entry> {
        self.idx.next().map(|v| {
            let mode = if v.ends_with('/') {
                EntryMode::DIR
            } else {
                EntryMode::FILE
            };
            let meta = Metadata::new(mode);
            oio::Entry::with(v, meta)
        })
    }
}

impl oio::List for ImmutableDir {
    fn poll_next(&mut self, _: &mut Context<'_>) -> Poll<Result<Option<oio::Entry>>> {
        Poll::Ready(Ok(self.inner_next()))
    }
}

impl oio::BlockingList for ImmutableDir {
    fn next(&mut self) -> Result<Option<oio::Entry>> {
        Ok(self.inner_next())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use anyhow::Result;
    use futures::TryStreamExt;
    use log::debug;

    use super::*;
    use crate::layers::LoggingLayer;
    use crate::services::Http;
    use crate::EntryMode;
    use crate::Operator;

    #[tokio::test]
    async fn test_list() -> Result<()> {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();

        let mut iil = ImmutableIndexLayer::default();
        for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
            iil.insert(i.to_string())
        }

        let op = Operator::new(Http::from_map({
            let mut map = HashMap::new();
            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());

            map
        }))?
        .layer(LoggingLayer::default())
        .layer(iil)
        .finish();

        let mut map = HashMap::new();
        let mut set = HashSet::new();
        let mut ds = op.lister("").await?;
        while let Some(entry) = ds.try_next().await? {
            debug!("got entry: {}", entry.path());
            assert!(
                set.insert(entry.path().to_string()),
                "duplicated value: {}",
                entry.path()
            );
            map.insert(entry.path().to_string(), entry.metadata().mode());
        }

        assert_eq!(map["file"], EntryMode::FILE);
        assert_eq!(map["dir/"], EntryMode::DIR);
        assert_eq!(map["dir_without_prefix/"], EntryMode::DIR);
        Ok(())
    }

    #[tokio::test]
    async fn test_scan() -> Result<()> {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();

        let mut iil = ImmutableIndexLayer::default();
        for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
            iil.insert(i.to_string())
        }

        let op = Operator::new(Http::from_map({
            let mut map = HashMap::new();
            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());

            map
        }))?
        .layer(LoggingLayer::default())
        .layer(iil)
        .finish();

        let mut ds = op.lister_with("/").recursive(true).await?;
        let mut set = HashSet::new();
        let mut map = HashMap::new();
        while let Some(entry) = ds.try_next().await? {
            debug!("got entry: {}", entry.path());
            assert!(
                set.insert(entry.path().to_string()),
                "duplicated value: {}",
                entry.path()
            );
            map.insert(entry.path().to_string(), entry.metadata().mode());
        }

        debug!("current files: {:?}", map);

        assert_eq!(map["file"], EntryMode::FILE);
        assert_eq!(map["dir/"], EntryMode::DIR);
        assert_eq!(map["dir_without_prefix/file"], EntryMode::FILE);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_dir() -> Result<()> {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();

        let mut iil = ImmutableIndexLayer::default();
        for i in [
            "dataset/stateful/ontime_2007_200.csv",
            "dataset/stateful/ontime_2008_200.csv",
            "dataset/stateful/ontime_2009_200.csv",
        ] {
            iil.insert(i.to_string())
        }

        let op = Operator::new(Http::from_map({
            let mut map = HashMap::new();
            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());

            map
        }))?
        .layer(LoggingLayer::default())
        .layer(iil)
        .finish();

        //  List /
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        let mut ds = op.lister("/").await?;
        while let Some(entry) = ds.try_next().await? {
            assert!(
                set.insert(entry.path().to_string()),
                "duplicated value: {}",
                entry.path()
            );
            map.insert(entry.path().to_string(), entry.metadata().mode());
        }

        assert_eq!(map.len(), 1);
        assert_eq!(map["dataset/"], EntryMode::DIR);

        //  List dataset/stateful/
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        let mut ds = op.lister("dataset/stateful/").await?;
        while let Some(entry) = ds.try_next().await? {
            assert!(
                set.insert(entry.path().to_string()),
                "duplicated value: {}",
                entry.path()
            );
            map.insert(entry.path().to_string(), entry.metadata().mode());
        }

        assert_eq!(map["dataset/stateful/ontime_2007_200.csv"], EntryMode::FILE);
        assert_eq!(map["dataset/stateful/ontime_2008_200.csv"], EntryMode::FILE);
        assert_eq!(map["dataset/stateful/ontime_2009_200.csv"], EntryMode::FILE);
        Ok(())
    }

    #[tokio::test]
    async fn test_walk_top_down_dir() -> Result<()> {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();

        let mut iil = ImmutableIndexLayer::default();
        for i in [
            "dataset/stateful/ontime_2007_200.csv",
            "dataset/stateful/ontime_2008_200.csv",
            "dataset/stateful/ontime_2009_200.csv",
        ] {
            iil.insert(i.to_string())
        }

        let op = Operator::new(Http::from_map({
            let mut map = HashMap::new();
            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());

            map
        }))?
        .layer(LoggingLayer::default())
        .layer(iil)
        .finish();

        let mut ds = op.lister_with("/").recursive(true).await?;

        let mut map = HashMap::new();
        let mut set = HashSet::new();
        while let Some(entry) = ds.try_next().await? {
            assert!(
                set.insert(entry.path().to_string()),
                "duplicated value: {}",
                entry.path()
            );
            map.insert(entry.path().to_string(), entry.metadata().mode());
        }

        debug!("current files: {:?}", map);

        assert_eq!(map["dataset/stateful/ontime_2007_200.csv"], EntryMode::FILE);
        assert_eq!(map["dataset/stateful/ontime_2008_200.csv"], EntryMode::FILE);
        assert_eq!(map["dataset/stateful/ontime_2009_200.csv"], EntryMode::FILE);
        Ok(())
    }
}
