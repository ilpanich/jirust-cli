#[macro_use]
extern crate prettytable;

use std::env;

use crate::args::commands::{Commands, JirustCliArgs};

use crate::executors::version_executor::VersionExecutor;
use args::commands::ConfigActionValues;
use clap::Parser;
use config::config_file::ConfigFile;

mod args;
pub mod config;
mod executors;
pub mod runners;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let config_file_path = match env::var_os("HOME") {
        Some(home) => format!("{}/.jirust-cli/jirust-cli.conf", home.to_string_lossy()),
        None => ".jirust-cli/jirust-cli.conf".to_string(),
    };
    let mut opts = match JirustCliArgs::try_parse_from(std::env::args()) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("Error: {}", err);
            err.exit();
        }
    };
    let cfg_data = match ConfigFile::read_from_file(config_file_path.as_str()) {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!("Error: Missing config file, setup mandatory!");
            opts.subcmd = Commands::Config(args::commands::ConfigArgs {
                cfg_act: ConfigActionValues::Setup,
            });
            ConfigFile::default()
        }
    };
    if cfg_data.get_auth_key().is_empty() || cfg_data.get_jira_url().is_empty() {
        eprintln!("Error: Missing basic configuration, setup mandatory!");
        opts.subcmd = Commands::Config(args::commands::ConfigArgs {
            cfg_act: ConfigActionValues::Setup,
        });
    }
    match opts.subcmd {
        Commands::Config(args) => {
            let config_executor =
                executors::config_executor::ConfigExecutor::new(config_file_path, args.cfg_act);
            config_executor.exec_command(cfg_data).await?
        }
        Commands::Version(args) => {
            let version_executor = VersionExecutor::new(cfg_data, args.version_act, args);
            version_executor.exec_command().await?
        }
    }
    Ok(())
}
