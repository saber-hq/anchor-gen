use std::collections::BTreeMap;

use anchor_lang_idl_spec::{IdlAccount, IdlField, IdlTypeDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    fields::{generate_struct_fields_from_slice, get_idl_defined_fields_as_slice},
    get_field_list_properties, StructOpts,
};

/// Generates an account state struct.
pub fn generate_account(
    defs: &[IdlTypeDef],
    account_name: &str,
    fields: &[IdlField],
    opts: StructOpts,
) -> TokenStream {
    let props = get_field_list_properties(defs, fields);

    let derive_copy = if props.can_copy && !opts.zero_copy {
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
    let derive_account = if opts.zero_copy {
        let repr = if opts.packed {
            quote! {
                #[repr(packed)]
            }
        } else {
            quote! {
                #[repr(C)]
            }
        };
        quote! {
            #[account(zero_copy(unsafe))]
            #repr
        }
    } else {
        quote! {
            #[account]
        }
    };

    let doc = format!(" Account: {}", account_name);
    let struct_name = format_ident!("{}", account_name);
    let fields_rendered = generate_struct_fields_from_slice(fields);
    quote! {
        #derive_account
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
    typedefs: &[IdlTypeDef],
    account_defs: &[IdlAccount],
    struct_opts: &BTreeMap<String, StructOpts>,
) -> TokenStream {
    let defined = account_defs
        .iter()
        .map(|account| {
            typedefs
                .iter()
                .find(|type_def| type_def.name == account.name)
                .unwrap()
        })
        .map(|def| match &def.ty {
            anchor_lang_idl_spec::IdlTypeDefTy::Struct { fields } => {
                let opts = struct_opts.get(&def.name).copied().unwrap_or_default();
                generate_account(
                    typedefs,
                    &def.name,
                    get_idl_defined_fields_as_slice(fields),
                    opts,
                )
            }
            anchor_lang_idl_spec::IdlTypeDefTy::Enum { .. } => {
                panic!("unexpected enum account");
            }
            anchor_lang_idl_spec::IdlTypeDefTy::Type { alias: _ } => {
                panic!("unexpected type account")
            }
        });
    quote! {
        #(#defined)*
    }
}
