use anchor_idl::GeneratorOptions;
use clap::{Parser, Subcommand};
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, File};

#[derive(Parser)]
#[command(name = "glam-cpi-gen")]
#[command(about = "Generates CPI interface from an IDL file", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate CPI interface for external programs
    Cpi {
        #[arg(required = true)]
        idl_path: String,

        #[arg(short, long, required = true)]
        program_id: String,

        #[arg(long)]
        output: Option<String>,
    },
    /// Generate GLAM CPI wrapper implementation
    Glam {
        #[arg(required = true)]
        idl_path: String,

        #[arg(long)]
        idl_name_alias: Option<String>,

        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        skip_imports: bool,

        #[arg(long)]
        output: Option<String>,

        #[arg(long)]
        config: Option<String>,

        #[arg(long)]
        ixs: Option<Vec<String>>,
    },
}

fn prettify(tokens: TokenStream) -> String {
    let syntax_tree: File = parse2(tokens).expect("Failed to parse TokenStream");
    let pretty_code = unparse(&syntax_tree);

    pretty_code
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cpi {
            idl_path,
            program_id,
            output,
        } => {
            let opts = GeneratorOptions {
                idl_path,
                ..Default::default()
            };
            let generator = opts.to_generator();

            let mut token_stream = TokenStream::new();
            token_stream.extend(quote! {
                use anchor_lang::declare_id;
                declare_id!(#program_id);
            });
            token_stream.extend(generator.generate_cpi_interface());
            let pretty_code = prettify(token_stream);

            if let Some(output_file) = output {
                std::fs::write(output_file, pretty_code).unwrap();
            } else {
                println!("{}", pretty_code);
            }
        }
        Commands::Glam {
            idl_path,
            idl_name_alias,
            skip_imports,
            output,
            config,
            ixs,
        } => {
            let opts = GeneratorOptions {
                idl_path,
                idl_name_alias: idl_name_alias.clone(),
                glam_codegen_config: config,
                ..Default::default()
            };
            let generator = opts.to_generator();

            let glam_code = generator.generate_glam_code(&ixs.unwrap_or_default(), skip_imports, idl_name_alias);
            let pretty_code = prettify(glam_code);

            if let Some(output_file) = output {
                std::fs::write(output_file, pretty_code).unwrap();
            } else {
                print!("{}", pretty_code);
            }
        }
    }
}
