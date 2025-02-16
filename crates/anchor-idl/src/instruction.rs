use anchor_syn::idl::IdlInstruction;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::GlamIxCodeGenConfig;

/// Generates a single instruction handler.
pub fn generate_ix_handler(ix: &IdlInstruction) -> TokenStream {
    let ix_name = format_ident!("{}", ix.name.to_snake_case());
    let accounts_name = format_ident!("{}", ix.name.to_pascal_case());

    let args = ix
        .args
        .iter()
        .map(|arg| {
            let name = format_ident!("_{}", arg.name.to_snake_case());
            let type_name = crate::ty_to_rust_type(&arg.ty);
            let stream: proc_macro2::TokenStream = type_name.parse().unwrap();
            quote! {
                #name: #stream
            }
        })
        .collect::<Vec<_>>();

    if cfg!(feature = "compat-program-result") {
        quote! {
            pub fn #ix_name(
                _ctx: Context<#accounts_name>,
                #(#args),*
            ) -> ProgramResult {
                unimplemented!("This program is a wrapper for CPI.")
            }
        }
    } else {
        quote! {
            pub fn #ix_name(
                _ctx: Context<#accounts_name>,
                #(#args),*
            ) -> Result<()> {
                unimplemented!("This program is a wrapper for CPI.")
            }
        }
    }
}

pub fn generate_glam_ix_structs(
    ixs: &[IdlInstruction],
    program_name: &Ident,
    ixs_to_generate: &[String],
    ix_code_gen_configs: &std::collections::HashMap<String, GlamIxCodeGenConfig>,
) -> TokenStream {
    //  ixs_to_generate &&  ix_code_gen_configs: generate only the intersecting instructions
    // !ixs_to_generate && !ix_code_gen_configs: generate all instructions
    // !ixs_to_generate &&  ix_code_gen_configs: generate only the instructions specified in the config
    //  ixs_to_generate && !ix_code_gen_configs: generate only the specified instructions

    let defs = ixs
        .iter()
        .filter(|ix| ixs_to_generate.is_empty() || ixs_to_generate.contains(&ix.name.to_string()))
        .map(|ix| {
            let accounts_name = format_ident!("{}{}", program_name, ix.name.to_pascal_case());
            let ix_code_gen_config = ix_code_gen_configs.get(ix.name.as_str());

            let (_all_structs, all_fields) = crate::generate_glam_account_fields(
                &ix.name.to_pascal_case(),
                &ix.accounts,
                ix_code_gen_config,
            );

            let glam_state_annotation = ix_code_gen_config
                .map(|config| {
                    if config.mutable_state {
                        quote! { #[account(mut)] }
                    } else {
                        quote! {}
                    }
                })
                .unwrap_or(quote! {});

            let seeds =
                quote! { [crate::constants::SEED_VAULT.as_bytes(), glam_state.key().as_ref()] };
            let glam_vault_annotation = if let Some(config) = ix_code_gen_config {
                if config.mutable_vault {
                    quote! { #[account(mut, seeds = #seeds, bump)] }
                } else {
                    quote! { #[account(seeds = #seeds, bump)] }
                }
            } else {
                quote! { #[account(seeds = #seeds, bump)] }
            };

            if let Some(type_alias) = ix_code_gen_config
                .unwrap_or(&GlamIxCodeGenConfig::default())
                .accounts_type_alias
                .clone()
            {
                let type_alias = format_ident!("{}{}", program_name, type_alias.to_pascal_case());
                quote! {
                    pub type #accounts_name<'info> = #type_alias<'info>;
                }
            } else {
                let mut glam_accounts_ts = TokenStream::new();
                glam_accounts_ts.extend(quote! {
                    #glam_state_annotation
                    pub glam_state: Box<Account<'info, StateAccount>>,

                    #glam_vault_annotation
                    pub glam_vault: SystemAccount<'info>,

                    #[account(mut)]
                    pub glam_signer: Signer<'info>,

                    pub cpi_program: Program<'info, #program_name>,
                });

                quote! {
                    #[derive(Accounts)]
                    pub struct #accounts_name<'info> {
                        #glam_accounts_ts

                        #all_fields
                    }
                }
            }
        });
    quote! {
        #(#defs)*
    }
}

pub fn generate_ix_structs(ixs: &[IdlInstruction]) -> TokenStream {
    let defs = ixs.iter().map(|ix| {
        let accounts_name = format_ident!("{}", ix.name.to_pascal_case());

        let (all_structs, all_fields) =
            crate::generate_account_fields(&ix.name.to_pascal_case(), &ix.accounts);

        quote! {
            #all_structs

            #[derive(Accounts)]
            pub struct #accounts_name<'info> {

                #all_fields
            }
        }
    });
    quote! {
        #(#defs)*
    }
}

/// Generates all instruction handlers.
pub fn generate_ix_handlers(ixs: &[IdlInstruction]) -> TokenStream {
    let streams = ixs.iter().map(generate_ix_handler);
    quote! {
        #(#streams)*
    }
}

pub fn generate_glam_ix_handler(
    ix: &IdlInstruction,
    program_name: &Ident,
    ix_code_gen_config: &GlamIxCodeGenConfig,
) -> TokenStream {
    let program_name_snake_case = format_ident!("{}", program_name.to_string().to_snake_case());
    let program_name_pascal_case = format_ident!("{}", program_name.to_string().to_pascal_case());

    let glam_ix_name = format_ident!("{}_{}", program_name_snake_case, ix.name.to_snake_case());
    let cpi_ix_name = format_ident!("{}", ix.name.to_snake_case());

    let glam_ix_accounts_name =
        format_ident!("{}{}", program_name_pascal_case, ix.name.to_pascal_case());
    let cpi_ix_accounts_name = format_ident!("{}", ix.name.to_pascal_case());

    let args = ix
        .args
        .iter()
        .map(|arg| {
            let name = format_ident!("{}", arg.name.to_snake_case());
            let type_name = crate::ty_to_rust_type(&arg.ty);
            let stream: proc_macro2::TokenStream = type_name.parse().unwrap();
            quote! {
                #name: #stream
            }
        })
        .collect::<Vec<_>>();

    let cpi_ix_args = ix
        .args
        .iter()
        .map(|arg| {
            let name = format_ident!("{}", arg.name.to_snake_case());
            quote! {
                #name
            }
        })
        .collect::<Vec<_>>();

    let account_infos = ix
        .accounts
        .iter()
        .map(|account| match account {
            anchor_syn::idl::IdlAccountItem::IdlAccount(info) => {
                let name = format_ident!("{}", info.name.to_snake_case());
                quote! {
                    #name: ctx.accounts.#name.to_account_info()
                }
            }
            anchor_syn::idl::IdlAccountItem::IdlAccounts(_info) => quote! {},
        })
        .collect::<Vec<_>>();

    let access_control_permission = if let Some(permission) = &ix_code_gen_config.permission {
        let permission = format_ident!("{}", permission);
        quote! {
            #[access_control(acl::check_access(&ctx.accounts.glam_state, &ctx.accounts.glam_signer.key, Permission::#permission))]
        }
    } else {
        quote! {}
    };

    let access_control_integration = if let Some(integration) = &ix_code_gen_config.integration {
        let integration = format_ident!("{}", integration);
        quote! {
            #[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::#integration))]
        }
    } else {
        quote! {}
    };

    let (lt0, lt1, lt2, lt3) = if ix_code_gen_config.with_remaining_accounts {
        (
            quote! { <'c: 'info, 'info> },
            quote! { '_, '_, 'c, 'info, },
            quote! { <'info> },
            quote! { .with_remaining_accounts(ctx.remaining_accounts.to_vec())},
        )
    } else {
        (quote! {}, quote! {}, quote! {}, quote! {})
    };

    if ix_code_gen_config.signed_by_vault {
        quote! {
            #access_control_permission
            #access_control_integration
            #[glam_macros::glam_vault_signer_seeds]
            pub fn #glam_ix_name #lt0(
                ctx: Context<#lt1 #glam_ix_accounts_name #lt2>,
                #(#args),*
            ) -> Result<()> {
                #program_name_snake_case::cpi::#cpi_ix_name(CpiContext::new_with_signer(
                    ctx.accounts.cpi_program.to_account_info(),
                    #program_name_snake_case::cpi::accounts::#cpi_ix_accounts_name {
                        #(#account_infos),*
                    },
                    glam_vault_signer_seeds
                )#lt3,#(#cpi_ix_args),*)
            }
        }
    } else {
        quote! {
            #access_control_permission
            #access_control_integration
            pub fn #glam_ix_name(
                ctx: Context<#glam_ix_accounts_name>,
                #(#args),*
            ) -> Result<()> {
                #program_name_snake_case::cpi::#cpi_ix_name(CpiContext::new(
                    ctx.accounts.cpi_program.to_account_info(),
                    #program_name_snake_case::cpi::accounts::#cpi_ix_accounts_name {
                        #(#account_infos),*
                    },
                ),#(#cpi_ix_args),*)
            }
        }
    }
}

pub fn generate_glam_ix_handlers(
    ixs: &[IdlInstruction],
    program_name: &Ident,
    ixs_to_generate: &[String],
    ix_code_gen_configs: &std::collections::HashMap<String, GlamIxCodeGenConfig>,
) -> TokenStream {
    let streams = ixs
        .iter()
        .filter(|ix| ixs_to_generate.is_empty() || ixs_to_generate.contains(&ix.name.to_string()))
        .map(|ix| {
            let ix_code_gen_config = ix_code_gen_configs
                .get(ix.name.as_str())
                .cloned()
                .unwrap_or_default();

            generate_glam_ix_handler(ix, program_name, &ix_code_gen_config)
        });
    quote! {
        #(#streams)*
    }
}
