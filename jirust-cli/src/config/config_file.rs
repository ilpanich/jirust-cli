use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};
use toml;

#[derive(Debug)]
pub struct AuthData {
    username: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    auth: AuthSection,
    jira: JiraSection,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthSection {
    auth_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JiraSection {
    jira_url: String,
}

impl AuthData {
    pub fn new(username: String, api_key: String) -> AuthData {
        AuthData { username, api_key }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username.replace("\n", "");
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key.replace("\n", "");
    }

    /// Encode the username and api_key to base64 to be used in the Authorization header of the request.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(format!("{}:{}", self.username, self.api_key).replace("\n", ""))
    }
}

impl ConfigFile {
    pub fn new(auth_token: String, jira_url: String) -> ConfigFile {
        ConfigFile {
            auth: AuthSection { auth_token },
            jira: JiraSection { jira_url },
        }
    }

    pub fn default() -> ConfigFile {
        ConfigFile {
            auth: AuthSection {
                auth_token: String::from(""),
            },
            jira: JiraSection {
                jira_url: String::from(""),
            },
        }
    }

    pub fn set_auth_key(&mut self, auth_token: String) {
        self.auth.auth_token = auth_token.replace("\n", "");
    }

    pub fn get_auth_key(&self) -> &str {
        &self.auth.auth_token
    }

    pub fn set_jira_url(&mut self, jira_url: String) {
        self.jira.jira_url = jira_url.replace("\n", "");
    }

    pub fn get_jira_url(&self) -> &str {
        &self.jira.jira_url
    }

    /// Stores the configuration to a file.
    pub fn write_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Failed to open file");
        let toml_str = toml::to_string(self).expect("Failed to serialize toml");
        file.write_all(toml_str.as_bytes())
    }

    /// Loads the configuration from a file.
    pub fn read_from_file(file_path: &str) -> Result<ConfigFile, toml::de::Error> {
        let config_file_str = fs::read_to_string(file_path)
            .unwrap_or(toml::to_string(&ConfigFile::default()).unwrap_or("".to_string()));
        toml::from_str(&config_file_str)
    }
}
