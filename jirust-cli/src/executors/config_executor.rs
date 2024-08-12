use crate::args::commands::ConfigActionValues;
use crate::config::config_file::ConfigFile;
use crate::runners::cfg_cmd_runner::ConfigCmdRunner;

pub struct ConfigExecutor {
    config_cmd_runner: ConfigCmdRunner,
    config_action: ConfigActionValues,
}

impl ConfigExecutor {
    pub fn new(cfg_file: String, config_action: ConfigActionValues) -> Self {
        let config_cmd_runner = ConfigCmdRunner::new(cfg_file.clone());
        Self {
            config_cmd_runner,
            config_action,
        }
    }

    pub async fn exec_command(
        &self,
        cfg_data: ConfigFile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.config_action {
            ConfigActionValues::Auth => {
                let _cfg_res = match self.config_cmd_runner.set_cfg_auth(cfg_data) {
                    Ok(_) => println!("Authentication configuration stored successfully"),
                    Err(err) => {
                        eprintln!("Error storing authentication configuration: {}", err);
                    }
                };
            }
            ConfigActionValues::Jira => {
                let _cfg_res = match self.config_cmd_runner.set_cfg_jira(cfg_data) {
                    Ok(_) => println!("Initialization configuration stored successfully"),
                    Err(err) => {
                        eprintln!("Error storing initialization configuration: {}", err);
                    }
                };
            }
            ConfigActionValues::Setup => {
                let _cfg_res = match self.config_cmd_runner.setup_cfg(cfg_data) {
                    Ok(_) => println!("Configuration setup successfully"),
                    Err(err) => {
                        eprintln!("Error setting up configuration: {}", err);
                    }
                };
            }
            ConfigActionValues::Show => {
                self.config_cmd_runner.show_cfg(cfg_data);
            }
        }
        Ok(())
    }
}
