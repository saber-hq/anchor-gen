//! Generates an Anchor crate interface from a JSON IDL.

use anchor_syn::idl::{IdlAccountItem, IdlType};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{env, fs, path::PathBuf};
use syn::{parse_macro_input, LitStr};

fn render_account_fields(name: &str, accounts: &[IdlAccountItem]) -> (TokenStream, TokenStream) {
    let mut all_structs: Vec<TokenStream> = vec![];
    let all_fields = accounts
        .iter()
        .map(|account| match account {
            anchor_syn::idl::IdlAccountItem::IdlAccount(info) => {
                let acc_name = format_ident!("{}", info.name.to_snake_case());
                let annotation = if info.is_mut {
                    quote! { #[account(mut)] }
                } else {
                    quote! {}
                };
                let ty = if info.is_signer {
                    quote! { Signer<'info> }
                } else {
                    quote! { AccountInfo<'info> }
                };
                quote! {
                   #annotation
                   pub #acc_name: #ty
                }
            }
            anchor_syn::idl::IdlAccountItem::IdlAccounts(inner) => {
                let field_name = format_ident!("{}{}", name, inner.name.to_snake_case());
                let sub_name = format!("{}{}", name, inner.name.to_pascal_case());
                let (sub_structs, sub_fields) = render_account_fields(&sub_name, &inner.accounts);
                all_structs.push(sub_structs);
                all_structs.push(quote! {
                    #[derive(Accounts)]
                    pub struct #sub_name<'info> {
                        #sub_fields
                    }
                });
                quote! {
                    pub #field_name: #sub_name<'info>
                }
            }
        })
        .collect::<Vec<_>>();
    (
        quote! {
            #(#all_structs)*
        },
        quote! {
            #(#all_fields),*
        },
    )
}

fn to_str(ty: &IdlType) -> String {
    match ty {
        IdlType::Bool => "bool".to_string(),
        IdlType::U8 => "u8".to_string(),
        IdlType::I8 => "i8".to_string(),
        IdlType::U16 => "u16".to_string(),
        IdlType::I16 => "i16".to_string(),
        IdlType::U32 => "u32".to_string(),
        IdlType::I32 => "i32".to_string(),
        IdlType::F32 => "f32".to_string(),
        IdlType::U64 => "u64".to_string(),
        IdlType::I64 => "i64".to_string(),
        IdlType::F64 => "f64".to_string(),
        IdlType::U128 => "u128".to_string(),
        IdlType::I128 => "i128".to_string(),
        IdlType::Bytes => "Vec<u8>".to_string(),
        IdlType::String => "String".to_string(),
        IdlType::PublicKey => "Pubkey".to_string(),
        IdlType::Option(inner) => format!("Option<{}>", to_str(&inner)),
        IdlType::Vec(inner) => format!("Vec<{}>", to_str(&inner)),
        IdlType::Array(ty, size) => format!("[{}; {}]", to_str(&ty), size),
        IdlType::Defined(name) => format!("{}", name),
    }
}

/// parses a string literal public key into a byte array public key
///
/// # Arguments
///
/// * `input` - A public key string
///
/// # Examples
///
/// ```
/// use static_pubkey::static_pubkey;
/// let key = static_pubkey!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
/// assert!(key.to_string() == "GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
/// ```
#[proc_macro]
pub fn generate_cpi_crate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let id_literal = parse_macro_input!(input as LitStr);
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(cargo_manifest_dir).join(id_literal.value());
    let idl_contents = fs::read_to_string(&path).unwrap();
    let idl: anchor_syn::idl::Idl = serde_json::from_str(&idl_contents).unwrap();

    let program_name: Ident = format_ident!("{}", idl.name);

    let defined = idl
        .types
        .iter()
        .map(|def| {
            let struct_name = format_ident!("{}", def.name);
            match &def.ty {
                anchor_syn::idl::IdlTypeDefinitionTy::Struct { fields } => {
                    let fields_rendered = fields
                        .iter()
                        .map(|arg| {
                            let name = format_ident!("{}", arg.name.to_snake_case());
                            let type_name = to_str(&arg.ty);
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
                anchor_syn::idl::IdlTypeDefinitionTy::Enum { variants } => {
                    let variant_idents = variants.iter().map(|v| format_ident!("{}", v.name));
                    quote! {
                        #[derive(AnchorSerialize, AnchorDeserialize)]
                        pub enum #struct_name {
                            #(#variant_idents),*
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let ixs = idl.instructions.iter().map(|ix| {
        let ix_name = format_ident!("{}", ix.name.to_snake_case());
        let accounts_name = format_ident!("{}", ix.name.to_pascal_case());

        let args = ix
            .args
            .iter()
            .map(|arg| {
                let name = format_ident!("_{}", arg.name.to_snake_case());
                let type_name = to_str(&arg.ty);
                let stream: proc_macro2::TokenStream = type_name.parse().unwrap();
                quote! {
                    #name: #stream
                }
            })
            .collect::<Vec<_>>();

        quote! {
            pub fn #ix_name(
                _ctx: Context<#accounts_name>,
                #(#args),*
            ) -> Result<()> {
                todo!()
            }
        }
    });

    let ix_structs = idl.instructions.iter().map(|ix| {
        let accounts_name = format_ident!("{}", ix.name.to_pascal_case());

        let (all_structs, all_fields) =
            render_account_fields(&ix.name.to_pascal_case(), &ix.accounts);

        quote! {
            #all_structs

            #[derive(Accounts)]
            pub struct #accounts_name<'info> {
                #all_fields
            }
        }
    });

    let output = quote! {
        use anchor_lang::prelude::*;

        #(#defined)*

        #(#ix_structs)*

        #[program]
        mod #program_name {
            use super::*;
            #(#ixs)*
        }
    };
    output.into()
}
