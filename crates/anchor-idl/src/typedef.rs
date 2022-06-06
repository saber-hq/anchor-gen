use anchor_syn::idl::{IdlEnumVariant, IdlField, IdlTypeDefinition};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

/// Generates a struct.
pub fn generate_struct(struct_name: &Ident, fields: &[IdlField]) -> TokenStream {
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
        #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
        pub struct #struct_name {
            #(#fields_rendered),*
        }
    }
}

/// Generates an enum.
pub fn generate_enum(enum_name: &Ident, variants: &[IdlEnumVariant]) -> TokenStream {
    let variant_idents = variants.iter().map(|v| format_ident!("{}", v.name));
    quote! {
        #[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug)]
        pub enum #enum_name {
            #(#variant_idents),*
        }
    }
}

/// Generates structs and enums.
pub fn generate_typedefs(typedefs: &[IdlTypeDefinition]) -> TokenStream {
    let defined = typedefs.iter().map(|def| {
        let struct_name = format_ident!("{}", def.name);
        match &def.ty {
            anchor_syn::idl::IdlTypeDefinitionTy::Struct { fields } => {
                generate_struct(&struct_name, &fields)
            }
            anchor_syn::idl::IdlTypeDefinitionTy::Enum { variants } => {
                generate_enum(&struct_name, &variants)
            }
        }
    });
    quote! {
        #(#defined)*
    }
}
