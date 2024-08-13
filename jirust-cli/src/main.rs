extern crate prettytable;

use std::env;

use jirust_cli::args::commands::{Commands, ConfigActionValues, ConfigArgs};
use jirust_cli::config::config_file::ConfigFile;
use jirust_cli::{manage_config, process_command};

/// Jirust CLI main function
/// Run without arguments to see the help message
#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let config_file_path = match env::var_os("HOME") {
        Some(home) => format!("{}/.jirust-cli/jirust-cli.conf", home.to_string_lossy()),
        None => ".jirust-cli/jirust-cli.conf".to_string(),
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
    let _res = process_command(opts, config_file_path, cfg_data).await;
    Ok(())
}
