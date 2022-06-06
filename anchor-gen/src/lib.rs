//! Generates an Anchor crate interface from a JSON IDL.

use std::{env, fs, path::PathBuf};
use syn::{parse_macro_input, LitStr};

/// Generates an Anchor CPI crate from a JSON file.
///
/// # Arguments
///
/// * `input` - Path to a JSON IDL relative to the crate's the Cargo.toml.
///
/// # Examples
///
/// ```
/// anchor_gen::generate_cpi_crate!("../examples/govern-cpi/idl.json");
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
pub fn generate_cpi_crate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let id_literal = parse_macro_input!(input as LitStr);
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(cargo_manifest_dir).join(id_literal.value());
    let idl_contents = fs::read_to_string(&path).unwrap();
    let idl: anchor_syn::idl::Idl = serde_json::from_str(&idl_contents).unwrap();
    let output = anchor_idl::generate_cpi_helpers(&idl);
    output.into()
}
