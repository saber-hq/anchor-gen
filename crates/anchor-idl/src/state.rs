use anchor_syn::idl::{IdlField, IdlTypeDefinition};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::generate_fields;

/// Generates an account state struct.
pub fn generate_account(account_name: &str, fields: &[IdlField]) -> TokenStream {
    let doc = format!(" Account: {}", account_name);
    let struct_name = format_ident!("{}", account_name);
    let fields_rendered = generate_fields(&fields);
    quote! {
        #[account]
        #[doc = #doc]
        pub struct #struct_name {
            #fields_rendered
        }
    }
}

/// Generates account state structs.
pub fn generate_accounts(typedefs: &[IdlTypeDefinition]) -> TokenStream {
    let defined = typedefs.iter().map(|def| match &def.ty {
        anchor_syn::idl::IdlTypeDefinitionTy::Struct { fields } => {
            generate_account(&def.name, &fields)
        }
        anchor_syn::idl::IdlTypeDefinitionTy::Enum { .. } => {
            panic!("unexpected enum account");
        }
    });
    quote! {
        #(#defined)*
    }
}
