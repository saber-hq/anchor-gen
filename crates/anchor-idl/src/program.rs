use crate::*;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

/// Generates all CPI helpers.
pub fn generate_cpi_helpers(idl: &anchor_syn::idl::Idl) -> TokenStream {
    let program_name: Ident = format_ident!("{}", idl.name);

    let accounts = generate_accounts(&idl.accounts);
    let typedefs = generate_typedefs(&idl.types);
    let ix_handlers = generate_ix_handlers(&idl.instructions);
    let ix_structs = generate_ix_structs(&idl.instructions);

    let docs = format!(
        " Anchor CPI crate generated from {} v{} using [anchor-gen](https://crates.io/crates/anchor-gen) v{}.",
        &idl.name,
        &idl.version, 
        &GEN_VERSION.unwrap_or("unknown")
    );

    quote! {
        use anchor_lang::prelude::*;

        pub mod typedefs {
            //! User-defined types.
            use super::*;
            #typedefs
        }

        pub mod state {
            //! Accounts which hold state.
            use super::*;
            #accounts
        }

        pub mod instructions {
            //! Accounts used in instructions.
            use super::*;
            #ix_structs
        }

        use instructions::*;
        use state::*;
        use typedefs::*;


        #[program]
        pub mod #program_name {
            #![doc = #docs]

            use super::*;
            #ix_handlers
        }
    }
}