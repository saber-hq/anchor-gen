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
    let vault_aliases =
        ix_code_gen_config.map_or(Vec::new(), |c| c.vault_aliases.clone().unwrap_or_default());

    let mut all_structs: Vec<TokenStream> = vec![];
    let all_fields = accounts
        .iter()
        .map(|account| match account {
            anchor_syn::idl::IdlAccountItem::IdlAccount(info) => {
                // account annotation
                let mut annotation = if info.is_mut {
                    quote! { #[account(mut)] }
                } else {
                    quote! {}
                };

                // type and lifetime
                // always remove signer if it's a vault alias
                let ty = if info.is_signer {
                    quote! { Signer<'info> }
                } else if info.name.to_snake_case().eq("system_program") {
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

                let acc_name = format_ident!("{}", info.name.to_snake_case());
                // result
                if vault_aliases.contains(&info.name) {
                    None
                } else {
                    Some(quote! {
                       #annotation
                       pub #acc_name: #ty
                    })
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
                Some(quote! {
                    pub #field_name: #sub_ident<'info>
                })
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
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
