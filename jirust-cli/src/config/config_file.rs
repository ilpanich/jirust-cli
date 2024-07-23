use base64::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug)]
pub struct ConfigData {
    username: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    auth: AuthSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSection {
    auth_token: String,
}

impl ConfigData {
    pub fn new(username: String, api_key: String) -> ConfigData {
        ConfigData { username, api_key }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }

    /// Encode the username and api_key to base64 to be used in the Authorization header of the request.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(format!("{}:{}", self.username, self.api_key))
    }
}

impl ConfigFile {
    pub fn new(auth_token: String) -> ConfigFile {
        ConfigFile {
            auth: AuthSection { auth_token },
        }
    }

    pub fn set_auth_key(&mut self, auth_token: String) {
        self.auth.auth_token = auth_token;
    }

    pub fn get_auth_key(&self) -> &str {
        &self.auth.auth_token
    }

    /// Stores the configuration to a file.
    pub fn write_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open file");
        serde_yaml::to_writer(file, self).expect("Failed to write to file");
        Ok(())
    }

    /// Loads the configuration from a file.
    pub fn read_from_file(file_path: &str) -> Result<ConfigFile, std::io::Error> {
        let file = std::fs::File::open(file_path).expect("Failed to open file");
        let config_file: ConfigFile =
            serde_yaml::from_reader(file).expect("Failed to read from file");
        Ok(config_file)
    }
}
