use crate::error::CLIError;
use clap::{crate_authors, crate_description, crate_version, App, SubCommand};
use cmd_lib::run_cmd;
use entity::resolver::{EntityResolverPassBox, FieldResolverSeedBox, FileResolver};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub type Command = &'static str;
pub type ModePath = &'static str;

pub struct CommandLineEntry {
    seeds: Vec<FieldResolverSeedBox>,
    passes: Vec<EntityResolverPassBox>,
    schema_file_paths: Vec<String>,
    output_file_path: String,
    after_setup: Vec<Command>,
}
impl CommandLineEntry {
    pub fn new(
        seeds: Vec<FieldResolverSeedBox>,
        passes: Vec<EntityResolverPassBox>,
        schema_file_paths: Vec<String>,
        output_file_path: String,
        after_resolve: Vec<&'static str>,
    ) -> Self {
        CommandLineEntry {
            seeds,
            passes,
            after_setup: after_resolve,
            output_file_path,
            schema_file_paths,
        }
    }

    pub fn process(self) {
        let application = App::new("Yukino CommandLine Tool")
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .subcommand(
                SubCommand::with_name("setup")
                    .about("Setup entities")
                    .version(crate_version!())
                    .author(crate_authors!()),
            );
        let matches = application.get_matches();
        if matches.subcommand_matches("setup").is_some() {
            let result = self.setup();
            Self::handle_result(result)
        }
    }

    fn handle_result<R>(result: Result<R, CLIError>) -> R {
        match result {
            Ok(result) => result,
            Err(err) => {
                eprintln!("{:?}", err);
                exit(1)
            }
        }
    }

    fn setup(self) -> Result<(), CLIError> {
        let cmd_list = self.after_setup.clone();

        let resolvers = self
            .schema_file_paths
            .iter()
            .map(|path| {
                FileResolver::new(
                    self.seeds.iter().map(|item| item.boxed()).collect(),
                    self.passes.iter().map(|item| item.boxed()).collect(),
                    path.clone(),
                )
                .map_err(|e| CLIError::ResolveError(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let result = resolvers
            .into_iter()
            .map(|resolver| {
                resolver
                    .resolve()
                    .map_err(|e| CLIError::ResolveError(e.to_string()))
                    .map(|result| result.get_result())
            })
            .fold(Ok(TokenStream::new()), |carry_result, item_result| {
                if let Ok(mut carry) = carry_result {
                    if let Ok(item) = item_result {
                        item.to_tokens(&mut carry);

                        Ok(carry)
                    } else {
                        item_result
                    }
                } else {
                    carry_result
                }
            })?
            .to_string();

        let path = Path::new(&self.output_file_path);
        if path.exists() {
            remove_file(path)?
        };

        let mut output_file = File::create(&self.output_file_path)?;

        output_file
            .write_all(result.as_bytes())
            .map_err(CLIError::IOError)?;

        for cmd in cmd_list {
            run_cmd(cmd)?
        }

        Ok(())
    }
}
