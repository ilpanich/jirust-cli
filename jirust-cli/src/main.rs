#[macro_use]
extern crate prettytable;

use std::env;

use crate::args::commands::{Commands, JirustCliArgs};
use crate::runners::{
    cfg_cmd_runner::ConfigCmdRunner,
    jira_cmd_runners::version_cmd_runner::{
        print_table_full, print_table_single, VersionCmdParams, VersionCmdRunner,
    },
};

use args::commands::ConfigActionValues;
use clap::Parser;
use config::config_file::ConfigFile;

pub mod args;
pub mod config;
pub mod runners;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let config_file_path = match env::var_os("HOME") {
        Some(home) => format!("{}/.jirust-cli/jirust-cli.conf", home.to_string_lossy()),
        None => ".jirust-cli/jirust-cli.conf".to_string(),
    };
    let config_cmd_runner = ConfigCmdRunner::new(config_file_path.clone());
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
        Commands::Config(arg) => match arg.cfg_act {
            cfg_arg => match cfg_arg {
                ConfigActionValues::Auth => {
                    let _cfg_res = match config_cmd_runner.set_cfg_auth(cfg_data) {
                        Ok(_) => println!("Authentication configuration stored successfully"),
                        Err(err) => {
                            eprintln!("Error storing authentication configuration: {}", err);
                        }
                    };
                }
                ConfigActionValues::Jira => {
                    let _cfg_res = match config_cmd_runner.set_cfg_jira(cfg_data) {
                        Ok(_) => println!("Initialization configuration stored successfully"),
                        Err(err) => {
                            eprintln!("Error storing initialization configuration: {}", err);
                        }
                    };
                }
                ConfigActionValues::Setup => {
                    let _cfg_res = match config_cmd_runner.setup_cfg(cfg_data) {
                        Ok(_) => println!("Configuration setup successfully"),
                        Err(err) => {
                            eprintln!("Error setting up configuration: {}", err);
                        }
                    };
                }
                ConfigActionValues::Show => {
                    config_cmd_runner.show_cfg(cfg_data);
                }
            },
        },
        Commands::Version(arg) => match arg {
            version_arg => match version_arg.version_act {
                args::commands::VersionActionValues::Create => {
                    let version_cmd_runner = VersionCmdRunner::new(cfg_data);
                    let res = version_cmd_runner
                        .create_jira_version(VersionCmdParams::from(&version_arg))
                        .await?;
                    print_table_single(res);
                }
                args::commands::VersionActionValues::List => {
                    let version_cmd_runner = VersionCmdRunner::new(cfg_data);
                    let res = version_cmd_runner
                        .list_jira_versions(VersionCmdParams::from(&version_arg))
                        .await?;
                    print_table_full(res);
                }
                args::commands::VersionActionValues::Update => {
                    println!("Update version");
                }
                args::commands::VersionActionValues::Delete => {
                    let version_cmd_runner = VersionCmdRunner::new(cfg_data);
                    let res = version_cmd_runner
                        .delete_jira_version(VersionCmdParams::from(&version_arg))
                        .await;
                    match res {
                        Ok(_) => println!("Version deleted successfully"),
                        Err(err) => eprintln!("Error deleting version: {}", err),
                    }
                }
                args::commands::VersionActionValues::Release => {
                    println!("Release version");
                }
            },
        },
    }
    Ok(())
}
