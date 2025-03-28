extern crate prettytable;

use std::env;

use jirust_cli::args::commands::{
    Commands, ConfigActionValues, ConfigArgs, OutputTypes, OutputValues,
};
use jirust_cli::config::config_file::ConfigFile;
use jirust_cli::utils::{OutputType, print_data};
use jirust_cli::{manage_config, process_command};

/// Jirust CLI main function
/// Run without arguments to see the help message
#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let config_file_path = match env::var_os("HOME") {
        Some(home) => format!("{}/.jirust-cli/jirust-cli.toml", home.to_string_lossy()),
        None => ".jirust-cli/jirust-cli.toml".to_string(),
    };
    let (cfg_data, opts) = match manage_config(config_file_path.clone(), std::env::args()) {
        Ok((cfg, opts)) => (cfg, opts),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Error: Missing config file, setup mandatory!");
                let subcmd = Commands::Config(ConfigArgs {
                    cfg_act: ConfigActionValues::Setup,
                });
                (ConfigFile::default(), subcmd)
            }
            _ => {
                eprintln!("Error: Missing config file, setup mandatory!");
                let subcmd = Commands::Config(ConfigArgs {
                    cfg_act: ConfigActionValues::Setup,
                });
                (ConfigFile::default(), subcmd)
            }
        },
    };
    match process_command(opts.clone(), Some(config_file_path), cfg_data).await {
        Ok(result) => {
            let (output_format, output_type) = match opts {
                Commands::Config(_) => (OutputValues::Json, OutputType::Full),
                Commands::Issue(args) => (
                    args.output.output_format.unwrap_or(OutputValues::Json),
                    OutputType::from(args.output.output_type.unwrap_or(OutputTypes::Full)),
                ),
                Commands::Link(_) => (OutputValues::Json, OutputType::Full),
                Commands::Project(args) => (
                    args.output.output_format.unwrap_or(OutputValues::Json),
                    OutputType::from(args.output.output_type.unwrap_or(OutputTypes::Full)),
                ),
                Commands::Transition(args) => (
                    args.output.output_format.unwrap_or(OutputValues::Json),
                    OutputType::from(args.output.output_type.unwrap_or(OutputTypes::Full)),
                ),
                Commands::Version(args) => (
                    args.output.output_format.unwrap_or(OutputValues::Json),
                    OutputType::from(args.output.output_type.unwrap_or(OutputTypes::Full)),
                ),
            };
            for elem in result {
                print_data(elem, output_format, output_type.clone());
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}
