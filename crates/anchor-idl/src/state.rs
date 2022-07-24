use anchor_syn::idl::{IdlField, IdlTypeDefinition};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{generate_fields, get_field_list_properties};

/// Generates an account state struct.
pub fn generate_account(
    defs: &[IdlTypeDefinition],
    account_name: &str,
    fields: &[IdlField],
) -> TokenStream {
    let props = get_field_list_properties(defs, fields);

    let derive_copy = if props.can_copy {
        quote! {
            #[derive(Copy)]
        }
    } else {
        quote! {}
    };
    let derive_default = if props.can_derive_default {
        quote! {
            #[derive(Default)]
        }
    } else {
        quote! {}
    };

    let doc = format!(" Account: {}", account_name);
    let struct_name = format_ident!("{}", account_name);
    let fields_rendered = generate_fields(&fields);
    quote! {
        #[account]
        #[doc = #doc]
        #derive_copy
        #derive_default
        pub struct #struct_name {
            #fields_rendered
        }
    }
}

/// Generates account state structs.
pub fn generate_accounts(
    typedefs: &[IdlTypeDefinition],
    account_defs: &[IdlTypeDefinition],
) -> TokenStream {
    let defined = account_defs.iter().map(|def| match &def.ty {
        anchor_syn::idl::IdlTypeDefinitionTy::Struct { fields } => {
            generate_account(typedefs, &def.name, &fields)
        }
        anchor_syn::idl::IdlTypeDefinitionTy::Enum { .. } => {
            panic!("unexpected enum account");
        }
    });
    quote! {
        #(#defined)*
    }
}
