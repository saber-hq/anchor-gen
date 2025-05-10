use crate::{
    fields::{generate_struct_fields, get_idl_defined_fields_as_slice},
    get_field_list_properties, StructOpts,
};
use anchor_lang_idl_spec::{IdlDefinedFields, IdlEvent, IdlTypeDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::Ident;

/// Generates a struct.
pub fn generate_event(
    defs: &[IdlTypeDef],
    struct_name: &Ident,
    fields: &Option<IdlDefinedFields>,
) -> TokenStream {
    let fields_rendered = generate_struct_fields(fields);
    let props = get_field_list_properties(defs, get_idl_defined_fields_as_slice(fields));

    let derive_default = if props.can_derive_default {
        quote! {
            #[derive(Default)]
        }
    } else {
        quote! {}
    };

    quote! {
        #[event]
        #[derive(Debug)]
        #derive_default
        pub struct #struct_name {
            #fields_rendered
        }
    }
}

/// Generates event structs.
pub fn generate_events(
    events: &[IdlEvent],
    typedefs: &[IdlTypeDef],
    struct_opts: &BTreeMap<String, StructOpts>,
) -> TokenStream {
    let defined = events.iter().map(|def| {
        let struct_name = format_ident!("{}", def.name);
        let opts = struct_opts.get(&def.name).copied().unwrap_or_default();
        if opts.skip {
            quote! {}
        } else {
            let typedef = typedefs.iter().find(|d| d.name == def.name).unwrap();
            if let anchor_lang_idl_spec::IdlTypeDefTy::Struct { fields } = &typedef.ty {
                generate_event(typedefs, &struct_name, fields)
            } else {
                quote! {}
            }
        }
    });
    quote! {
        #(#defined)*
    }
}
