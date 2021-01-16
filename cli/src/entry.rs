use crate::error::CLIError;
use crate::resolver::Resolver;
use clap::{crate_authors, crate_description, crate_version, App, SubCommand};
use cmd_lib::run_cmd;
use entity::resolver::FieldResolverSeedBox;
use std::collections::HashMap;
use std::process::exit;

pub type Command = &'static str;
pub type ModePath = &'static str;

pub struct CommandLineEntry {
    resolver: Resolver,
    after_setup: Vec<Command>,
}
impl CommandLineEntry {
    pub fn new(
        seeds: Vec<FieldResolverSeedBox>,
        model_files_path: HashMap<ModePath, String>,
        output_file_path: String,
        after_resolve: Vec<&'static str>,
    ) -> Self {
        CommandLineEntry {
            resolver: Self::handle_result(Resolver::new(seeds, model_files_path, output_file_path)),
            after_setup: after_resolve,
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

        self.resolver.resolve()?.write_result()?;

        for cmd in cmd_list {
            run_cmd(cmd)?
        }

        Ok(())
    }
}
