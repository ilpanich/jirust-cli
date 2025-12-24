use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};
use toml::{self, Table, Value};

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
    #[serde(default)]
    yara: YaraSection,
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
    transitions_names: Table,
}

/// This struct holds the YARA scanner configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YaraSection {
    rules_source: String,
    rules_directory: String,
    cache_file: String,
    cache_version_file: String,
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
/// * `set_standard_resolution(standard_resolution: String)` - sets the standard_resolution in the ConfigFile
/// * `get_standard_resolution() -> String` - gets the standard_resolution from the ConfigFile
/// * `set_standard_resolution_comment(standard_resolution_comment: String)` - sets the standard_resolution_comment in the ConfigFile
/// * `get_standard_resolution_comment() -> String` - gets the standard_resolution_comment from the Config
/// * `add_transition_name(key: String, value: String)` - adds a transition_name to the ConfigFile
/// * `get_transition_name(key: &str) -> Option<String>` - gets a transition_name from the ConfigFile
impl ConfigFile {
    /// Create a new ConfigFile struct.
    ///
    /// # Arguments
    /// * auth_token - The authentication token to be used with the Jira API.
    /// * jira_url - The base_url for the Jira API.
    /// * standard_resolution - The standard resolution to be used when resolving an issue.
    /// * standard_resolution_comment - The standard comment to be used when resolving an issue.
    /// * transitions_names - The transitions names to be used when transitioning an issue.
    /// * yara - The YARA scanner configuration section.
    ///
    /// # Returns
    /// * A new ConfigFile struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::{ConfigFile, YaraSection};
    /// use toml::Table;
    ///
    /// let config = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new(), YaraSection::default());
    ///
    /// assert_eq!(config.get_auth_key(), "auth_token");
    /// assert_eq!(config.get_jira_url(), "jira_url");
    /// assert_eq!(config.get_standard_resolution(), "standard_resolution");
    /// assert_eq!(config.get_standard_resolution_comment(), "standard_resolution_comment");
    /// ```
    pub fn new(
        auth_token: String,
        jira_url: String,
        standard_resolution: String,
        standard_resolution_comment: String,
        transitions_names: Table,
        yara: YaraSection,
    ) -> ConfigFile {
        ConfigFile {
            auth: AuthSection { auth_token },
            jira: JiraSection {
                jira_url,
                standard_resolution,
                standard_resolution_comment,
                transitions_names,
            },
            yara,
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
    /// use toml::Table;
    ///
    /// let config = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new(), YaraSection::default());
    /// let auth_key = config.get_auth_key();
    ///
    /// assert_eq!(auth_key, "auth_token");
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
    /// use toml::Table;
    ///
    /// let config = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new(), YaraSection::default());
    /// let jira_url = config.get_jira_url();
    ///
    /// assert_eq!(jira_url, "jira_url");
    /// ```
    pub fn get_jira_url(&self) -> &str {
        &self.jira.jira_url
    }

    /// Set the standard resolution for the ConfigFile struct.
    /// This is the standard resolution that will be used when resolving an issue.
    ///
    /// # Arguments
    /// * standard_resolution - The standard resolution to be used when resolving an issue.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let mut config = ConfigFile::default();
    /// config.set_standard_resolution("standard_resolution".to_string());
    ///
    /// assert_eq!(config.get_standard_resolution(), "standard_resolution");
    /// ```
    pub fn set_standard_resolution(&mut self, standard_resolution: String) {
        self.jira.standard_resolution = standard_resolution;
    }

    /// Get the standard resolution for the ConfigFile struct.
    /// This is the standard resolution that will be used when resolving an issue.
    ///
    /// # Returns
    /// * The standard resolution to be used when resolving an issue.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// let config = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new(), YaraSection::default());
    /// let standard_resolution = config.get_standard_resolution();
    ///
    /// assert_eq!(config.get_standard_resolution(), "standard_resolution");
    /// ```
    pub fn get_standard_resolution(&self) -> &String {
        &self.jira.standard_resolution
    }

    /// Set the standard resolution comment for the ConfigFile struct.
    /// This is the standard resolution comment that will be used when resolving an issue.
    ///
    /// # Arguments
    /// * standard_resolution_comment - The standard resolution comment to be used when resolving an issue.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let mut config = ConfigFile::default();
    /// config.set_standard_resolution_comment("standard_resolution_comment".to_string());
    ///
    /// assert_eq!(config.get_standard_resolution_comment(), "standard_resolution_comment");
    /// ```
    pub fn set_standard_resolution_comment(&mut self, standard_resolution_comment: String) {
        self.jira.standard_resolution_comment = standard_resolution_comment;
    }

    /// Get the standard resolution comment for the ConfigFile struct.
    /// This is the standard resolution comment that will be used when resolving an issue.
    ///
    /// # Returns
    /// * The standard resolution comment to be used when resolving an issue.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// let config = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new(), YaraSection::default());
    /// let standard_resolution_comment = config.get_standard_resolution_comment();
    ///
    /// assert_eq!(standard_resolution_comment, "standard_resolution_comment");
    /// ```
    pub fn get_standard_resolution_comment(&self) -> &String {
        &self.jira.standard_resolution_comment
    }

    /// Add a transition name to the ConfigFile struct.
    /// This is used to store the transition name for a specific transition.
    /// This is used to transition an issue to a specific state.
    /// The key is the transition internal identifier and the value is the transition name.
    ///
    /// # Arguments
    /// * key - The transition internal identifier.
    /// * value - The transition name.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let mut config = ConfigFile::default();
    /// config.add_transition_name("transition_key".to_string(), "Transition name".to_string());
    ///
    /// assert_eq!(config.get_transition_name("transition_key"), Some(vec!["Transition name".to_string()]));
    /// ```
    pub fn add_transition_name(&mut self, key: String, value: String) {
        let mut existing_value: Vec<Value> = self
            .jira
            .transitions_names
            .get(&key)
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|v| Value::String(v.as_str().unwrap().to_string()))
            .collect();
        existing_value.push(Value::String(value));
        self.jira
            .transitions_names
            .insert(key, Value::Array(existing_value));
    }

    /// Get the transition name for a specific transition internal identifier.
    /// This is used to transition an issue to a specific state.
    /// The key is the transition internal identifier and the value is the transition name.
    /// If the transition internal identifier does not exist, None is returned.
    ///
    /// # Arguments
    /// * key - The transition internal identifier.
    ///
    /// # Returns
    /// * The transition name for the specific transition internal identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    ///
    /// let mut config = ConfigFile::default();
    /// config.add_transition_name("transition_key".to_string(), "Transition name".to_string());
    ///
    /// assert_eq!(config.get_transition_name("transition_key"), Some(vec!["Transition name".to_string()]));
    /// ```
    pub fn get_transition_name(&self, key: &str) -> Option<Vec<String>> {
        let tranisitons_names = self
            .jira
            .transitions_names
            .get(key)
            .and_then(|v| v.as_array());
        Some(
            tranisitons_names
                .unwrap_or(&vec![])
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
        )
    }

    /// Get the YARA section from the ConfigFile struct.
    ///
    /// # Returns
    /// * The YARA section from the ConfigFile.
    pub fn get_yara_section(&self) -> &YaraSection {
        &self.yara
    }

    /// Get the YARA rules source URL from the ConfigFile struct.
    ///
    /// # Returns
    /// * The YARA rules source URL.
    pub fn get_yara_rules_source(&self) -> &str {
        &self.yara.rules_source
    }

    /// Get the YARA rules directory from the ConfigFile struct.
    ///
    /// # Returns
    /// * The YARA rules directory name (relative to ~/.jirust-cli/).
    pub fn get_yara_rules_directory(&self) -> &str {
        &self.yara.rules_directory
    }

    /// Get the YARA cache file from the ConfigFile struct.
    ///
    /// # Returns
    /// * The YARA cache file name (relative to ~/.jirust-cli/).
    pub fn get_yara_cache_file(&self) -> &str {
        &self.yara.cache_file
    }

    /// Get the YARA cache version file from the ConfigFile struct.
    ///
    /// # Returns
    /// * The YARA cache version file name (relative to ~/.jirust-cli/).
    pub fn get_yara_cache_version_file(&self) -> &str {
        &self.yara.cache_version_file
    }

    /// Set the YARA rules source URL for the ConfigFile struct.
    ///
    /// # Arguments
    /// * source - The YARA rules source URL (git repo or zip file).
    pub fn set_yara_rules_source(&mut self, source: String) {
        self.yara.rules_source = source;
    }

    /// Set the YARA rules directory for the ConfigFile struct.
    ///
    /// # Arguments
    /// * directory - The YARA rules directory name (relative to ~/.jirust-cli/).
    pub fn set_yara_rules_directory(&mut self, directory: String) {
        self.yara.rules_directory = directory;
    }

    /// Set the YARA cache file for the ConfigFile struct.
    ///
    /// # Arguments
    /// * file - The YARA cache file name (relative to ~/.jirust-cli/).
    pub fn set_yara_cache_file(&mut self, file: String) {
        self.yara.cache_file = file;
    }

    /// Set the YARA cache version file for the ConfigFile struct.
    ///
    /// # Arguments
    /// * file - The YARA cache version file name (relative to ~/.jirust-cli/).
    pub fn set_yara_cache_version_file(&mut self, file: String) {
        self.yara.cache_version_file = file;
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
    /// use toml::Table;
    ///
    /// let config = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new(), YaraSection::default());
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
    /// use std::path::Path;
    ///
    /// // Try both possible paths to handle different working directories
    /// let config_path = if Path::new("config_example.toml").exists() {
    ///     "config_example.toml"
    /// } else {
    ///     "jirust-cli/config_example.toml"
    /// };
    ///
    /// let config = ConfigFile::read_from_file(config_path);
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

impl Default for YaraSection {
    /// Create a new YaraSection struct with default values.
    /// The default values are:
    /// - rules_source: "https://github.com/YARAHQ/yara-forge/releases/latest/download/yara-forge-rules-core.zip"
    /// - rules_directory: "yara-rules"
    /// - cache_file: "yara_rules.cache"
    /// - cache_version_file: "yara_rules.cache.version"
    ///
    /// # Returns
    /// * A new YaraSection struct with default values.
    fn default() -> YaraSection {
        YaraSection {
            rules_source: String::from(
                "https://github.com/YARAHQ/yara-forge/releases/latest/download/yara-forge-rules-core.zip",
            ),
            rules_directory: String::from("yara-rules"),
            cache_file: String::from("yara_rules.cache"),
            cache_version_file: String::from("yara_rules.cache.version"),
        }
    }
}

impl Default for ConfigFile {
    /// Create a new ConfigFile struct with default values.
    /// This is useful for creating a new configuration file.
    /// The default values can be set using the set methods.
    /// The default values are:
    /// - auth_token: ""
    /// - jira_url: ""
    /// - standard_resolution: ""
    /// - standard_resolution_comment: ""
    /// - transitions_names: Table::new()
    /// - yara: YaraSection::default()
    ///
    /// # Returns
    /// * A new ConfigFile struct with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// let config = ConfigFile::default();
    ///
    /// assert_eq!(config.get_auth_key(), "");
    /// assert_eq!(config.get_jira_url(), "");
    /// assert_eq!(config.get_standard_resolution(), "");
    /// assert_eq!(config.get_standard_resolution_comment(), "");
    /// ```
    fn default() -> ConfigFile {
        ConfigFile {
            auth: AuthSection {
                auth_token: String::from(""),
            },
            jira: JiraSection {
                jira_url: String::from(""),
                standard_resolution: String::from(""),
                standard_resolution_comment: String::from(""),
                transitions_names: Table::new(),
            },
            yara: YaraSection::default(),
        }
    }
}

impl YaraSection {
    pub fn new(
        rules_source: String,
        rules_directory: String,
        cache_file: String,
        cache_version_file: String,
    ) -> YaraSection {
        YaraSection {
            rules_source,
            rules_directory,
            cache_file,
            cache_version_file,
        }
    }

    pub fn get_rules_source(&self) -> &str {
        &self.rules_source
    }

    pub fn get_rules_directory(&self) -> &str {
        &self.rules_directory
    }

    pub fn get_cache_file(&self) -> &str {
        &self.cache_file
    }

    pub fn get_cache_version_file(&self) -> &str {
        &self.cache_version_file
    }
}
