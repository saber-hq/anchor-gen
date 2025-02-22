use heck::{ToPascalCase, ToSnakeCase};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env, fs,
    path::PathBuf,
};

use darling::{util::PathList, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{
    generate_accounts, generate_glam_ix_handlers, generate_glam_ix_structs, generate_ix_handlers,
    generate_ix_structs, generate_typedefs, GEN_VERSION,
};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GlamIxCodeGenConfig {
    pub ix_name: String,
    pub permission: Option<String>,
    pub integration: Option<String>,
    pub remove_signer: Option<Vec<String>>, // TODO: not being used, consider removing
    pub vault_aliases: Option<Vec<String>>,
    pub accounts_struct: Option<String>, // by default accounts struct name is `<ProgramName><IxName>`, this overwrites it with `<ProgramName><AccountsStruct>`
    pub with_remaining_accounts: bool,
    pub signed_by_vault: bool,
    pub mutable_vault: bool,
    pub mutable_state: bool,
}

#[derive(Default, FromMeta)]
pub struct GeneratorOptions {
    /// Path to the IDL.
    pub idl_path: String,
    /// GLAM autogen config json.
    pub glam_codegen_config: Option<String>,
    /// List of zero copy structs.
    pub zero_copy: Option<PathList>,
    /// List of `repr(packed)` structs.
    pub packed: Option<PathList>,
}

fn path_list_to_string(list: Option<&PathList>) -> HashSet<String> {
    list.map(|el| {
        el.iter()
            .map(|el| el.get_ident().unwrap().to_string())
            .collect()
    })
    .unwrap_or_default()
}

impl GeneratorOptions {
    pub fn to_generator(&self) -> Generator {
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = PathBuf::from(cargo_manifest_dir).join(&self.idl_path);
        let idl_contents = fs::read_to_string(&path).unwrap();
        let idl: anchor_syn::idl::Idl = serde_json::from_str(&idl_contents).unwrap();

        let zero_copy = path_list_to_string(self.zero_copy.as_ref());
        let packed = path_list_to_string(self.packed.as_ref());

        let mut struct_opts: BTreeMap<String, StructOpts> = BTreeMap::new();
        let all_structs: HashSet<&String> = zero_copy.union(&packed).collect::<HashSet<_>>();
        all_structs.into_iter().for_each(|name| {
            struct_opts.insert(
                name.to_string(),
                StructOpts {
                    zero_copy: zero_copy.contains(name),
                    packed: packed.contains(name),
                },
            );
        });

        let mut ix_code_gen_configs = HashMap::new();

        if let Some(glam_codegen_config) = &self.glam_codegen_config {
            let glam_autogen_config_contents = fs::read_to_string(glam_codegen_config).unwrap();
            let config: serde_json::Value =
                serde_json::from_str(&glam_autogen_config_contents).unwrap();

            ix_code_gen_configs = config
                .get(idl.name.as_str())
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|el| serde_json::from_value(el.clone()).unwrap())
                .collect::<Vec<GlamIxCodeGenConfig>>()
                .into_iter()
                .map(|c| (c.ix_name.clone(), c))
                .collect();
        }

        Generator {
            idl,
            struct_opts,
            ix_code_gen_configs,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct StructOpts {
    pub packed: bool,
    pub zero_copy: bool,
}

pub struct Generator {
    pub idl: anchor_syn::idl::Idl,
    pub struct_opts: BTreeMap<String, StructOpts>,
    pub ix_code_gen_configs: HashMap<String, GlamIxCodeGenConfig>,
}

impl Generator {
    pub fn generate_glam_code(&self, ixs: &[String], skip_imports: bool) -> TokenStream {
        let idl = &self.idl;
        let program_name_pascal_case = format_ident!("{}", idl.name.to_pascal_case());
        let program_name_snake_case = format_ident!("{}", idl.name.to_snake_case());

        let ix_structs = generate_glam_ix_structs(
            &idl.instructions,
            &program_name_pascal_case,
            ixs,
            &self.ix_code_gen_configs,
        );
        let ix_handlers = generate_glam_ix_handlers(
            &idl.instructions,
            &program_name_pascal_case,
            ixs,
            &self.ix_code_gen_configs,
        );

        let imports = if skip_imports {
            quote! {}
        } else {
            quote! {
                use anchor_lang::prelude::*;
                use crate::state::{acl::{self, *}, StateAccount};

                use #program_name_snake_case::program::#program_name_pascal_case;
                use #program_name_snake_case::typedefs::*;
            }
        };

        quote! {
            #imports

            #ix_structs

            #ix_handlers
        }
    }

    pub fn generate_cpi_interface(&self) -> TokenStream {
        let idl = &self.idl;
        let program_name: Ident = format_ident!("{}", idl.name);

        let accounts = generate_accounts(&idl.types, &idl.accounts, &self.struct_opts);
        let typedefs = generate_typedefs(&idl.types, &self.struct_opts);
        let ix_handlers = generate_ix_handlers(&idl.instructions);
        let ix_structs = generate_ix_structs(&idl.instructions);

        let docs = format!(
        " Anchor CPI crate generated from {} v{} using [anchor-gen](https://crates.io/crates/anchor-gen) v{}.",
        &idl.name,
        &idl.version,
        &GEN_VERSION.unwrap_or("unknown")
    );

        quote! {
            use anchor_lang::prelude::*;

            pub mod typedefs {
                //! User-defined types.
                use super::*;
                #typedefs
            }

            pub mod state {
                //! Structs of accounts which hold state.
                use super::*;
                #accounts
            }

            pub mod ix_accounts {
                //! Accounts used in instructions.
                use super::*;
                #ix_structs
            }

            use ix_accounts::*;
            pub use state::*;
            pub use typedefs::*;

            #[program]
            pub mod #program_name {
                #![doc = #docs]

                use super::*;
                #ix_handlers
            }
        }
    }
}
