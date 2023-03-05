use std::collections::BTreeMap;

use anchor_syn::idl::{IdlEvent, IdlField, IdlTypeDefinition};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{generate_struct, StructOpts};

/// Generates event structs.
pub fn generate_events(
    events: Option<&[IdlEvent]>,
    typedefs: &[IdlTypeDefinition],
    struct_opts: &BTreeMap<String, StructOpts>,
) -> TokenStream {
    match events {
        Some(events) => {
            let defined = events.iter().map(|def| {
                let struct_name = format_ident!("{}", def.name);
                let opts = struct_opts.get(&def.name).copied().unwrap_or_default();

                let fields = def
                    .fields
                    .iter()
                    .map(|f| IdlField {
                        name: f.name.clone(),
                        ty: f.ty.clone(),
                    })
                    .collect::<Vec<_>>();

                generate_struct(&typedefs, &struct_name, &fields, opts)
            });
            quote! {
                #(#defined)*
            }
        }
        None => quote!(),
    }
}
