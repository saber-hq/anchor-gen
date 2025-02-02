use anchor_syn::idl::IdlInstruction;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

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
    signers_to_remove: &std::collections::HashMap<String, Vec<String>>,
) -> TokenStream {
    let defs = ixs
        .iter()
        .filter(|ix| ixs_to_generate.is_empty() || ixs_to_generate.contains(&ix.name.to_string()))
        .map(|ix| {
            let accounts_name = format_ident!("{}{}", program_name, ix.name.to_pascal_case());

            let (_all_structs, all_fields) = crate::generate_account_fields(
                &ix.name.to_pascal_case(),
                &ix.accounts,
                signers_to_remove
                    .get(&ix.name.to_string())
                    .unwrap_or(&Vec::new()),
            );

            let mut glam_accounts_ts = TokenStream::new();
            glam_accounts_ts.extend(quote! {
                #[account(mut)]
                pub glam_state: Box<Account<'info, StateAccount>>,

                #[account(mut, seeds = [SEED_VAULT.as_bytes(), glam_state.key().as_ref()], bump)]
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
        });
    quote! {
        #(#defs)*
    }
}

pub fn generate_ix_structs(ixs: &[IdlInstruction]) -> TokenStream {
    let defs = ixs.iter().map(|ix| {
        let accounts_name = format_ident!("{}", ix.name.to_pascal_case());

        let (all_structs, all_fields) =
            crate::generate_account_fields(&ix.name.to_pascal_case(), &ix.accounts, &[]);

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
    permission: &Option<String>,
    integration: &Option<String>,
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

    let access_control_permission = if let Some(permission) = permission {
        let permission = format_ident!("{}", permission);
        quote! {
            #[access_control(acl::check_access(&ctx.accounts.glam_state, &ctx.accounts.glam_signer.key, Permission::#permission))]
        }
    } else {
        quote! {}
    };

    let access_control_integration = if let Some(integration) = integration {
        let integration = format_ident!("{}", integration);
        quote! {
            #[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::#integration))]
        }
    } else {
        quote! {}
    };

    quote! {
        #access_control_permission
        #access_control_integration
        pub fn #glam_ix_name(
            ctx: Context<#glam_ix_accounts_name>,
            #(#args),*
        ) -> Result<()> {
            let state_key = ctx.accounts.glam_state.key();
            let seeds = [
                "vault".as_ref(),
                state_key.as_ref(),
                &[ctx.bumps.glam_vault],
            ];
            let vault_signer_seeds = &[&seeds[..]];

            #program_name_snake_case::cpi::#cpi_ix_name(CpiContext::new_with_signer(
                ctx.accounts.cpi_program.to_account_info(),
                #program_name_snake_case::cpi::accounts::#cpi_ix_accounts_name {
                    #(#account_infos),*
                },
                vault_signer_seeds
            ),#(#cpi_ix_args),*)
        }
    }
}

pub fn generate_glam_ix_handlers(
    ixs: &[IdlInstruction],
    program_name: &Ident,
    ixs_to_generate: &[String],
    permissions: &std::collections::HashMap<String, Option<String>>,
    integrations: &std::collections::HashMap<String, Option<String>>,
) -> TokenStream {
    let streams = ixs
        .iter()
        .filter(|ix| ixs_to_generate.is_empty() || ixs_to_generate.contains(&ix.name.to_string()))
        .map(|ix| {
            generate_glam_ix_handler(
                ix,
                program_name,
                permissions.get(ix.name.as_str()).unwrap(),
                integrations.get(ix.name.as_str()).unwrap(),
            )
        });
    quote! {
        #(#streams)*
    }
}
