//! [![github]](https://github.com/dtolnay/target-tuple)&ensp;[![crates-io]](https://crates.io/crates/target-tuple)&ensp;[![docs-rs]](https://docs.rs/target-tuple)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

#![no_std]
#![doc(html_root_url = "https://docs.rs/target-tuple/1.0.2")]

#[cfg(not(host_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/macros.rs"));

#[cfg(host_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\macros.rs"));

/// The target tuple that is being compiled for.
pub const TARGET: &str = target!();

/// The host tuple of the Rust compiler.
pub const HOST: &str = host!();
