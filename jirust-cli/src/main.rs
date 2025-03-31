extern crate prettytable;

use std::env;

use clap::Parser;
use jirust_cli::args::commands::{
    Commands, ConfigActionValues, ConfigArgs, JirustCliArgs, OutputTypes, OutputValues,
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
    let mut opts = match JirustCliArgs::try_parse_from(std::env::args()) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("Error: {}", err);
            err.exit();
        }
    };
    let cfg_data = match manage_config(config_file_path.clone()) {
        Ok(cfg) => cfg,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Error: Missing config file, setup mandatory!");
                opts.subcmd = Commands::Config(ConfigArgs {
                    cfg_act: ConfigActionValues::Setup,
                });
                ConfigFile::default()
            }
            _ => {
                eprintln!("Error: Missing config file, setup mandatory!");
                opts.subcmd = Commands::Config(ConfigArgs {
                    cfg_act: ConfigActionValues::Setup,
                });
                ConfigFile::default()
            }
        },
    };
    match process_command(opts.subcmd.clone(), Some(config_file_path), cfg_data).await {
        Ok(result) => {
            let (output_format, output_type) = match opts.subcmd {
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
