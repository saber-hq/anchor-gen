//! Generates a crate for cross-program invocations to an Anchor program from a JSON IDL.
//!
//! # Usage
//!
//! In a new crate, write:
//!
//! ```skip
//! anchor_gen::generate_cpi_crate!("../../examples/govern-cpi/idl.json");
//!
//! declare_id!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
//! ```
//!
//! This will generate a fully functional Rust CPI client for your IDL.
//!
//! More examples can be found in the [examples/](https://github.com/saber-hq/anchor-gen/tree/master/examples) directory.

use anchor_idl::GeneratorOptions;
use darling::FromMeta;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Generates an Anchor CPI crate from a JSON file.
///
/// # Arguments
///
/// * `idl_path` - Path to a JSON IDL relative to the crate's the Cargo.toml.
///
/// # Examples
///
/// ```
/// anchor_generate_cpi_interface::generate_cpi_interface!(idl_path = "../../examples/govern-cpi/idl.json");
/// declare_id!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
/// # fn main() -> Result<()> {
/// let _my_governor = GovernanceParameters {
///     quorum_votes: 0,
///     timelock_delay_seconds: 0,
///     voting_period: 0,
///     voting_delay: 0,
/// };
/// #   Ok(())
/// # }
/// ```
#[proc_macro]
pub fn generate_cpi_interface(input: proc_macro::TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(input as syn::AttributeArgs);
    let parsed = match GeneratorOptions::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    parsed.to_generator().generate_cpi_interface().into()
}
