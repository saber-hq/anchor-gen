use anchor_syn::idl::{IdlField, IdlTypeDefinition};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

/// Generates an account state struct.
pub fn generate_account(struct_name: &Ident, fields: &[IdlField]) -> TokenStream {
    let fields_rendered = fields
        .iter()
        .map(|arg| {
            let name = format_ident!("{}", arg.name.to_snake_case());
            let type_name = crate::ty_to_rust_type(&arg.ty);
            let stream: proc_macro2::TokenStream = type_name.parse().unwrap();
            quote! {
                pub #name: #stream
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #[account]
        pub struct #struct_name {
            #(#fields_rendered),*
        }
    }
}

/// Generates account state structs.
pub fn generate_accounts(typedefs: &[IdlTypeDefinition]) -> TokenStream {
    let defined = typedefs.iter().map(|def| {
        let struct_name = format_ident!("{}", def.name);
        match &def.ty {
            anchor_syn::idl::IdlTypeDefinitionTy::Struct { fields } => {
                generate_account(&struct_name, &fields)
            }
            anchor_syn::idl::IdlTypeDefinitionTy::Enum { .. } => {
                panic!("unexpected enum account");
            }
        }
    });
    quote! {
        #(#defined)*
    }
}
