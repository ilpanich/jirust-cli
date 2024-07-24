use crate::config::config_file::{ConfigData, ConfigFile};

use rpassword::read_password;
use std::{fs, path::Path};

pub struct ConfigCmdRunner {
    cfg_file: String,
}

impl ConfigCmdRunner {
    pub fn new(cfg_file: String) -> ConfigCmdRunner {
        ConfigCmdRunner { cfg_file }
    }

    pub fn init_file(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.cfg_file);
        fs::create_dir_all(path.parent().unwrap())?;
        fs::File::create(path)?;
        Ok(())
    }

    pub fn set_cfg_auth(&self, mut cfg: ConfigFile) -> Result<(), std::io::Error> {
        let input = std::io::stdin();
        let mut user = String::new();
        let mut apikey = String::new();
        println!("Your username: ");
        input.read_line(&mut user)?;
        println!("Your apikey: ");
        apikey = read_password()?;
        let config_data = ConfigData::new(user, apikey);
        cfg.set_auth_key(config_data.to_base64());
        cfg.write_to_file(&self.cfg_file.as_str())
    }

    pub fn set_cfg_init(&self, mut cfg: ConfigFile) -> Result<(), std::io::Error> {
        let input = std::io::stdin();
        let mut url = String::new();
        println!("Your Jira instance URL: ");
        input.read_line(&mut url)?;
        cfg.set_jira_url(url);
        cfg.write_to_file(&self.cfg_file.as_str())
    }
}
