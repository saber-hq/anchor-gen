pub use anchor_syn::idl::*;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::GlamIxCodeGenConfig;

/// Generates a list of [IdlAccountItem]s as a [TokenStream].
pub fn generate_account_fields(
    name: &str,
    accounts: &[IdlAccountItem],
) -> (TokenStream, TokenStream) {
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
                let sub_ident = format_ident!("{}", &sub_name);
                let (sub_structs, sub_fields) = generate_account_fields(&sub_name, &inner.accounts);
                all_structs.push(sub_structs);
                all_structs.push(quote! {
                    #[derive(Accounts)]
                    pub struct #sub_ident<'info> {
                        #sub_fields
                    }
                });
                quote! {
                    pub #field_name: #sub_ident<'info>
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

pub fn generate_glam_account_fields(
    name: &str,
    accounts: &[IdlAccountItem],
    ix_code_gen_config: Option<&GlamIxCodeGenConfig>,
) -> (TokenStream, TokenStream) {
    let (remove_signer, vault_aliases) = if let Some(ix_config) = ix_code_gen_config {
        (
            ix_config.remove_signer.clone().unwrap_or(Vec::new()),
            ix_config.remove_signer.clone().unwrap_or(Vec::new()),
        )
    } else {
        (Vec::new(), Vec::new())
    };

    let mut all_structs: Vec<TokenStream> = vec![];
    let all_fields = accounts
        .iter()
        .map(|account| match account {
            anchor_syn::idl::IdlAccountItem::IdlAccount(info) => {
                let acc_name = format_ident!("{}", info.name.to_snake_case());
                let mut annotation = if info.is_mut && vault_aliases.contains(&info.name) {
                    quote! { #[account(mut, address = glam_state.vault)] }
                } else if !info.is_mut && vault_aliases.contains(&info.name) {
                    quote! { #[account(address = glam_state.vault)] }
                } else if info.is_mut {
                    quote! { #[account(mut)] }
                } else {
                    quote! {}
                };

                let ty = if info.is_signer && !remove_signer.contains(&info.name) {
                    quote! { Signer<'info> }
                } else if info.name.eq("systemProgram") {
                    quote! { Program<'info, System> }
                } else if info.name == "rent" {
                    quote! { Sysvar<'info, Rent> }
                } else {
                    let mut ts = quote! {
                        /// CHECK: should be validated by target program
                    };
                    ts.extend(annotation);
                    annotation = ts;

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
                let sub_ident = format_ident!("{}", &sub_name);
                let (sub_structs, sub_fields) =
                    generate_glam_account_fields(&sub_name, &inner.accounts, ix_code_gen_config);
                all_structs.push(sub_structs);
                all_structs.push(quote! {
                    #[derive(Accounts)]
                    pub struct #sub_ident<'info> {
                        #sub_fields
                    }
                });
                quote! {
                    pub #field_name: #sub_ident<'info>
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
