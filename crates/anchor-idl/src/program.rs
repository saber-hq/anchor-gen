use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    path::PathBuf,
};

use darling::{util::PathList, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    generate_accounts, generate_ix_handlers, generate_ix_structs, generate_typedefs, GEN_VERSION,
};

#[derive(Default, FromMeta)]
pub struct GeneratorOptions {
    /// Path to the IDL.
    pub idl_path: String,
    /// List of zero copy structs.
    pub zero_copy: Option<PathList>,
    /// List of `repr(packed)` structs.
    pub packed: Option<PathList>,
    /// List of structs that should have `Default` impl skipped.
    pub skip_default: Option<PathList>,
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
        let skip_default = path_list_to_string(self.skip_default.as_ref());

        let mut all_structs: HashSet<String> = HashSet::new();
        for struct_item in [&zero_copy, &packed, &skip_default].into_iter().flatten() {
            if !all_structs.contains(struct_item) {
                all_structs.insert(struct_item.clone());
            }
        }

        let mut struct_opts: BTreeMap<String, StructOpts> = BTreeMap::new();
        all_structs.iter().for_each(|name| {
            struct_opts.insert(
                name.to_string(),
                StructOpts {
                    zero_copy: zero_copy.contains(name),
                    packed: packed.contains(name),
                    skip_default: skip_default.contains(name),
                },
            );
        });

        Generator { idl, struct_opts }
    }
}

#[derive(Clone, Copy, Default)]
pub struct StructOpts {
    pub packed: bool,
    pub zero_copy: bool,
    pub skip_default: bool,
}

pub struct Generator {
    pub idl: anchor_syn::idl::Idl,
    pub struct_opts: BTreeMap<String, StructOpts>,
}

impl Generator {
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
