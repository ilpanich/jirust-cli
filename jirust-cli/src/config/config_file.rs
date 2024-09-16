use base64::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, io::Write};
use toml;

/// This struct holds the username and api_key for the Jira API.
#[derive(Debug)]
pub struct AuthData {
    username: String,
    api_key: String,
}

/// This struct holds the configuration data to use the Jira API (authentication info and Jira base_url).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    auth: AuthSection,
    jira: JiraSection,
}

/// This struct holds the authentication token to be used with the Jira API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthSection {
    auth_token: String,
}

/// This struct holds the Jira base_url.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JiraSection {
    jira_url: String,
    standard_resolution: String,
    standard_resolution_comment: String,
}

/// Implementation of AuthData
///
/// # Methods
/// * `new(username: String, api_key: String) -> AuthData` - creates a new instance of AuthData
/// * `set_username(username: String)` - sets the username
/// * `set_api_key(api_key: String)` - sets the api_key
/// * `get_username() -> String` - gets the username
/// * `get_api_key() -> String` - gets the api_key
/// * `to_base64() -> String` - converts the AuthData to a base64 string
/// * `from_base64(base64_str: String) -> AuthData` - converts a base64 string to an AuthData
/// * `write_to_file(file: &str) -> Result<(), std::io::Error>` - writes the AuthData to a file
/// * `read_from_file(file: &str) -> Result<AuthData, std::io::Error>` - reads the AuthData from a file
impl AuthData {
    /// Create a new AuthData struct.
    ///
    /// # Arguments
    /// * username - The username to be used for authentication.
    /// * api_key - The api_key to be used for authentication.
    ///
    /// # Returns
    /// * A new AuthData struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::AuthData;
    ///
    /// let auth_data = AuthData::new("username".to_string(), "api_key".to_string());
    /// ```
    pub fn new(username: String, api_key: String) -> AuthData {
        AuthData { username, api_key }
    }

    /// Set the username for the AuthData struct.
    ///
    /// # Arguments
    /// * username - The username to be used for authentication.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::AuthData;
    ///
    /// let mut auth_data = AuthData::new("username".to_string(), "api_key".to_string());
    /// auth_data.set_username("new_username".to_string());
    /// ```
    pub fn set_username(&mut self, username: String) {
        self.username = username.replace("\n", "");
    }

    /// Set the api_key for the AuthData struct.
    ///
    /// # Arguments
    /// * api_key - The api_key to be used for authentication.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::AuthData;
    ///
    /// let mut auth_data = AuthData::new("username".to_string(), "api_key".to_string());
    /// auth_data.set_api_key("new_api_key".to_string());
    /// ```
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key.replace("\n", "");
    }

    /// Encode the username and api_key to base64 to be used in the Authorization header of the request.
    ///
    /// # Returns
    /// * A base64 encoded string of the username and api_key.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::AuthData;
    ///
    /// let auth_data = AuthData::new("username".to_string(), "api_key".to_string());
    /// let base64_encoded = auth_data.to_base64();
    ///
    /// assert_eq!(base64_encoded, "dXNlcm5hbWU6YXBpX2tleQ==");
    /// ```
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(format!("{}:{}", self.username, self.api_key).replace("\n", ""))
    }

    /// Decode a base64 encoded string to get the username and api_key.
    ///
    /// # Arguments
    /// * encoded - The base64 encoded string to be decoded.
    ///
    /// # Returns
    /// * A tuple containing the username and api_key.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::AuthData;
    ///
    /// let (username, api_key) = AuthData::from_base64("dXNlcm5hbWU6YXBpX2tleQ==");
    ///
    /// assert_eq!(username, "username");
    /// assert_eq!(api_key, "api_key");
    /// ```
    pub fn from_base64(encoded: &str) -> (String, String) {
        let decoded = BASE64_STANDARD
            .decode(encoded)
            .expect("Failed to decode base64");
        let decoded_str = String::from_utf8(decoded).expect("Failed to convert to string");
        let parts: Vec<&str> = decoded_str.split(':').collect();
        (parts[0].to_string(), parts[1].to_string())
    }
}

/// Implementation of ConfigFile
///
/// # Methods
/// * `new(auth_token: String, jira_url: String) -> ConfigFile` - creates a new instance of ConfigFile
/// * default() -> ConfigFile - creates a new instance of ConfigFile with default values
/// * `write_to_file(file: &str) -> Result<(), std::io::Error>` - writes the ConfigFile to a file
/// * `read_from_file(file: &str) -> Result<ConfigFile, std::io::Error>` - reads the ConfigFile from a file
/// * `get_auth() -> AuthSection` - gets the AuthSection from the ConfigFile
/// * `get_jira() -> JiraSection` - gets the JiraSection from the ConfigFile
/// * `set_auth(auth: AuthSection)` - sets the AuthSection in the ConfigFile
/// * `set_jira(jira: JiraSection)` - sets the JiraSection in the ConfigFile
impl ConfigFile {
    /// Create a new ConfigFile struct.
    ///
    /// # Arguments
    /// * auth_token - The authentication token to be used with the Jira API.
    /// * jira_url - The base_url for the Jira API.
    ///
    /// # Returns
    /// * A new ConfigFile struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let config = ConfigFile::new("auth_token".to_string(), "jira_url".to_string());
    ///
    /// assert_eq!(config.get_auth_key(), "auth_token");
    /// assert_eq!(config.get_jira_url(), "jira_url");
    /// ```
    pub fn new(
        auth_token: String,
        jira_url: String,
        standard_resolution: String,
        standard_resolution_comment: String,
    ) -> ConfigFile {
        ConfigFile {
            auth: AuthSection { auth_token },
            jira: JiraSection {
                jira_url,
                standard_resolution,
                standard_resolution_comment,
            },
        }
    }

    /// Create a new ConfigFile struct with default values.
    /// This is useful for creating a new configuration file.
    /// The default values can be set using the set methods.
    /// The default values are:
    /// - auth_token: ""
    /// - jira_url: ""
    ///
    /// # Returns
    /// * A new ConfigFile struct with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let config = ConfigFile::default();
    ///
    /// assert_eq!(config.get_auth_key(), "");
    /// assert_eq!(config.get_jira_url(), "");
    /// ```
    pub fn default() -> ConfigFile {
        ConfigFile {
            auth: AuthSection {
                auth_token: String::from(""),
            },
            jira: JiraSection {
                jira_url: String::from(""),
                standard_resolution: String::from(""),
                standard_resolution_comment: String::from(""),
            },
        }
    }

    /// Set the authentication token for the ConfigFile struct.
    /// This is the token that will be used to authenticate with the Jira API.
    ///
    /// # Arguments
    /// * auth_token - The authentication token to be used with the Jira API.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let mut config = ConfigFile::default();
    /// config.set_auth_key("auth_key".to_string());
    ///
    /// assert_eq!(config.get_auth_key(), "auth_key");
    /// ```
    pub fn set_auth_key(&mut self, auth_token: String) {
        self.auth.auth_token = auth_token.replace("\n", "");
    }

    /// Get the authentication token for the ConfigFile struct.
    /// This is the token that will be used to authenticate with the Jira API.
    /// This is useful for getting the current value of the authentication token.
    ///
    /// # Returns
    /// * The authentication token to be used with the Jira API.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let config = ConfigFile::new("auth_key".to_string(), "jira_url".to_string());
    /// let auth_key = config.get_auth_key();
    ///
    /// assert_eq!(auth_key, "auth_key");
    /// ```
    pub fn get_auth_key(&self) -> &str {
        &self.auth.auth_token
    }

    /// Set the Jira URL for the ConfigFile struct.
    /// This is the base URL for the Jira API.
    ///
    /// # Arguments
    /// * jira_url - The base URL for the Jira API.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let mut config = ConfigFile::default();
    /// config.set_jira_url("jira_url".to_string());
    ///
    /// assert_eq!(config.get_jira_url(), "jira_url");
    /// ```
    pub fn set_jira_url(&mut self, jira_url: String) {
        self.jira.jira_url = jira_url.replace("\n", "");
    }

    /// Get the Jira URL for the ConfigFile struct.
    /// This is the base URL for the Jira API.
    ///
    /// # Returns
    /// * The base URL for the Jira API.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let config = ConfigFile::new("auth_key".to_string(), "jira_url".to_string());
    /// let jira_url = config.get_jira_url();
    ///
    /// assert_eq!(jira_url, "jira_url");
    /// ```
    pub fn get_jira_url(&self) -> &str {
        &self.jira.jira_url
    }

    pub fn set_standard_resolution(&mut self, standard_resolution: String) {
        self.jira.standard_resolution = standard_resolution;
    }

    pub fn get_standard_resolution(&self) -> &String {
        &self.jira.standard_resolution
    }

    pub fn set_standard_resolution_comment(&mut self, standard_resolution_comment: String) {
        self.jira.standard_resolution_comment = standard_resolution_comment;
    }

    pub fn get_standard_resolution_comment(&self) -> &String {
        &self.jira.standard_resolution_comment
    }

    /// Stores the configuration to a file.
    /// This will overwrite the file if it already exists.
    ///
    /// # Arguments
    /// * file_path - The path to the file to write the configuration to.
    ///
    /// # Returns
    /// * A Result containing either an empty Ok or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let config = ConfigFile::new("auth_key".to_string(), "jira_url".to_string());
    /// let result = config.write_to_file("config.toml");
    ///
    /// assert!(result.is_ok());
    /// ```
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
    /// If the file does not exist, it will return a ConfigFile with default values.
    /// If the file is not valid toml, it will return an error.
    /// If the file is valid toml, it will return the ConfigFile.
    ///
    /// # Arguments
    /// * file_path - The path to the file to read the configuration from.
    ///
    /// # Returns
    /// * A Result containing either the ConfigFile or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let config = ConfigFile::read_from_file("config_example.toml");
    ///
    /// assert!(config.clone().is_ok());
    /// assert_eq!(config.clone().unwrap().get_auth_key(), "auth_key");
    /// assert_eq!(config.clone().unwrap().get_jira_url(), "jira_url");
    /// ```
    pub fn read_from_file(file_path: &str) -> Result<ConfigFile, toml::de::Error> {
        let config_file_str = fs::read_to_string(file_path)
            .unwrap_or(toml::to_string(&ConfigFile::default()).unwrap_or("".to_string()));
        toml::from_str(&config_file_str)
    }
}
