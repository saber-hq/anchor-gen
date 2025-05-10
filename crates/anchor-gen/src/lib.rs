//! Generates a crate for cross-program invocations to an Anchor program from a JSON IDL.
//!
//! [![Crates.io](https://img.shields.io/crates/v/anchor-gen)](https://crates.io/crates/anchor-gen) [![License](https://img.shields.io/crates/l/anchor-gen)](https://github.com/saber-hq/anchor-gen/blob/master/LICENSE.txt) [![Build Status](https://img.shields.io/github/workflow/status/saber-hq/anchor-gen/Rust/master)](https://github.com/saber-hq/anchor-gen/actions/workflows/rust.yml?query=branch%3Amaster) [![Contributors](https://img.shields.io/github/contributors/saber-hq/anchor-gen)](https://github.com/saber-hq/anchor-gen/graphs/contributors) [![Code Coverage](https://img.shields.io/codecov/c/github/saber-hq/anchor-gen)](https://app.codecov.io/gh/saber-hq/anchor-gen)
//!
//! Now updated for Anchor 0.31.1!
//!
//! **Warning: this code has not been audited. Please use it at your own risk.**
//!
//! # Usage
//!
//! First, add the following to a `Cargo.toml` file in a new crate:
//!
//! ```toml
//! [dependencies]
//! anchor-gen = "0.31.1"
//! ```
//!
//! Then, in `lib.rs`, write:
//!
//! ```skip
//! anchor_gen::generate_cpi_crate!("../../examples/govern-cpi/idl.json");
//! ```
//!
//! This will generate a fully functional Rust CPI client for your IDL.
//!
//! Usage examples can be found in the [examples/](https://github.com/saber-hq/anchor-gen/tree/master/examples) directory.
//!
//! Note: This does not work on legacy IDLs. To migrate a legacy IDL, use `anchor idl convert idl.json`.

pub use anchor_generate_cpi_crate::generate_cpi_crate;
pub use anchor_generate_cpi_interface::generate_cpi_interface;
