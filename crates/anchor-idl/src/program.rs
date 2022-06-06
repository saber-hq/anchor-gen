use proc_macro2::{Ident, TokenStream};
use quote::{format_ident,quote};

use crate::GEN_VERSION;

/// Generates all CPI helpers.
pub fn generate_cpi_helpers(idl: &anchor_syn::idl::Idl) -> TokenStream {
    let program_name: Ident = format_ident!("{}", idl.name);

    let typedefs = crate::generate_typedefs(&idl.types);

    let ix_handlers = crate::generate_ix_handlers(&idl.instructions);

    let ix_structs = crate::generate_ix_structs(&idl.instructions);

    let docs = format!(
        " Anchor CPI crate generated from {} v{} using [anchor-gen](https://crates.io/crates/anchor-gen) v{}.",
        &idl.name,
        &idl.version, 
        &GEN_VERSION.unwrap_or("unknown")
    );

    quote! {
        use anchor_lang::prelude::*;

        #typedefs

        #ix_structs

        #[program]
        pub mod #program_name {
            #![doc = #docs]

            use super::*;
            #ix_handlers
        }
    }
}