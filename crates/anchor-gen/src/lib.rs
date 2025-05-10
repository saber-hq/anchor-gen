//! Generates a crate for cross-program invocations to an Anchor program from a JSON IDL.
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
