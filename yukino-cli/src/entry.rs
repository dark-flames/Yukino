use crate::error::YukinoCLIError;
use crate::resolver::Resolver;
use clap::{crate_authors, crate_description, crate_version, App, SubCommand};
use std::collections::HashMap;
use std::process::exit;
use yukino_core::mapping::resolver::FieldResolveCell;

#[allow(dead_code)]
pub struct CommandLineEntry {
    resolver: Resolver, // db connection
}

#[allow(dead_code)]
impl CommandLineEntry {
    pub fn new(
        seeds: Vec<Box<dyn FieldResolveCell>>,
        model_files_path: HashMap<&'static str, String>,
        output_file_path: String,
    ) -> Self {
        let resolver = Self::handle_result(
            Resolver::new(seeds, model_files_path, output_file_path).map_err(YukinoCLIError::from),
        );

        CommandLineEntry { resolver }
    }
    pub fn process(&mut self) {
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

    fn handle_result<R>(result: Result<R, YukinoCLIError>) -> R {
        match result {
            Ok(result) => result,
            Err(err) => {
                eprintln!("error: {:?}", err);
                exit(1)
            }
        }
    }

    fn setup(&mut self) -> Result<(), YukinoCLIError> {
        self.resolver.resolve()?;

        self.resolver.write_implements()?;

        Ok(())
    }
}
