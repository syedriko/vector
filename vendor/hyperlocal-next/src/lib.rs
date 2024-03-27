#![deny(
    missing_debug_implementations,
    unreachable_pub,
    rust_2018_idioms,
    missing_docs
)]
#![warn(clippy::all, clippy::pedantic)]

//! `hyperlocal` provides [Hyper](http://github.com/hyperium/hyper) bindings
//! for [Unix domain sockets](https://github.com/tokio-rs/tokio/tree/master/tokio-net/src/uds/).
//!
//! See the examples for how to configure a client or a server.
//!
//! # Features
//!
//! - Client- enables the client extension trait and connector. *Enabled by
//!   default*.

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{UnixClientExt, UnixConnector};

mod uri;

pub use uri::Uri;
