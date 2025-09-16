use crate::config::config_file::{AuthData, ConfigFile};

use rpassword::read_password;
use std::{fs, io::BufRead, path::Path};

/// ConfigCmdRunner is a struct that holds the configuration file path
/// and provides methods to initialize, set, and show the configuration file.
pub struct ConfigCmdRunner {
    cfg_file: String,
}

/// Implementation of ConfigCmdRunner
///
/// # Methods
///
/// * `new(cfg_file: String) -> ConfigCmdRunner` - creates a new instance of ConfigCmdRunner
/// * `init_file() -> Result<(), std::io::Error>` - initializes the configuration file
/// * `set_cfg_auth(cfg: ConfigFile) -> Result<ConfigFile, std::io::Error>` - sets the authentication data in the configuration file
/// * `set_cfg_jira(cfg: ConfigFile) -> Result<ConfigFile, std::io::Error>` - sets the Jira URL in the configuration file
/// * `setup_cfg(cfg: ConfigFile) -> Result<(), std::io::Error>` - sets up the configuration file
/// * `show_cfg(cfg: ConfigFile)` - shows the configuration file
impl ConfigCmdRunner {
    /// Creates a new instance of ConfigCmdRunner
    ///
    /// # Arguments
    ///
    /// * `cfg_file` - a String that holds the path to the configuration file
    ///
    /// # Returns
    ///
    /// * `ConfigCmdRunner` - a new instance of ConfigCmdRunner
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::cfg_cmd_runner::ConfigCmdRunner;
    ///
    /// let cfg_runner = ConfigCmdRunner::new("test_path/to/config/file".to_string());
    /// ```
    pub fn new(cfg_file: String) -> ConfigCmdRunner {
        ConfigCmdRunner { cfg_file }
    }

    /// Initializes the configuration file
    ///
    /// # Returns
    ///
    /// * `Result<(), std::io::Error>` - a Result that returns an empty tuple or an error
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::cfg_cmd_runner::ConfigCmdRunner;
    ///
    /// let cfg_runner = ConfigCmdRunner::new("test_path/to/config/file".to_string());
    /// cfg_runner.init_file();
    /// ```
    pub fn init_file(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.cfg_file);
        fs::create_dir_all(path.parent().unwrap())?;
        fs::File::create(path)?;
        Ok(())
    }

    /// Sets the authentication data in the configuration file
    ///
    /// # Arguments
    ///
    /// * `cfg` - a ConfigFile that holds the configuration data
    ///
    /// # Returns
    ///
    /// * `Result<ConfigFile, std::io::Error>` - a Result that returns the updated ConfigFile or an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::cfg_cmd_runner::ConfigCmdRunner;
    ///
    /// let cfg_runner = ConfigCmdRunner::new("test_path/to/config/file".to_string());
    /// let cfg = ConfigFile::default();
    ///
    /// cfg_runner.set_cfg_auth(cfg);
    /// ```
    pub fn set_cfg_auth(&self, cfg: ConfigFile) -> Result<ConfigFile, std::io::Error> {
        println!("Your username: ");
        let stdin = std::io::stdin();
        let mut reader = stdin.lock();
        self.read_auth_from_sources(cfg, &mut reader, || read_password())
    }

    fn read_auth_from_sources<R, P>(
        &self,
        mut cfg: ConfigFile,
        reader: &mut R,
        mut password_reader: P,
    ) -> Result<ConfigFile, std::io::Error>
    where
        R: BufRead,
        P: FnMut() -> Result<String, std::io::Error>,
    {
        let mut user = String::new();
        reader.read_line(&mut user)?;
        println!("Your apikey: ");
        let apikey = password_reader()?;
        let config_data = AuthData::new(user, apikey);
        cfg.set_auth_key(config_data.to_base64());
        cfg.write_to_file(self.cfg_file.as_str())?;
        Ok(cfg)
    }

    /// Sets the Jira URL in the configuration file
    ///
    /// # Arguments
    ///
    /// * `cfg` - a ConfigFile that holds the configuration data
    ///
    /// # Returns
    ///
    /// * `Result<ConfigFile, std::io::Error>` - a Result that returns the updated ConfigFile or an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::cfg_cmd_runner::ConfigCmdRunner;
    ///
    /// let cfg_runner = ConfigCmdRunner::new("test_path/to/config/file".to_string());
    /// let cfg = ConfigFile::default();
    /// cfg_runner.set_cfg_jira(cfg);
    /// ```
    pub fn set_cfg_jira(&self, cfg: ConfigFile) -> Result<ConfigFile, std::io::Error> {
        println!("Your Jira instance URL: ");
        let stdin = std::io::stdin();
        let mut reader = stdin.lock();
        self.read_jira_from_reader(cfg, &mut reader)
    }

    fn read_jira_from_reader<R>(
        &self,
        mut cfg: ConfigFile,
        reader: &mut R,
    ) -> Result<ConfigFile, std::io::Error>
    where
        R: BufRead,
    {
        let mut read_data = String::new();
        reader.read_line(&mut read_data)?;
        cfg.set_jira_url(read_data.clone());
        read_data.clear();
        println!("Default Jira issue resolution JSON Value: ");
        reader.read_line(&mut read_data)?;
        cfg.set_standard_resolution(read_data.clone());
        read_data.clear();
        println!("Default Jira issue resolution comment JSON: ");
        reader.read_line(&mut read_data)?;
        cfg.set_standard_resolution_comment(read_data);
        cfg.write_to_file(self.cfg_file.as_str())?;
        Ok(cfg)
    }

    /// Sets up the configuration file
    ///
    /// # Arguments
    ///
    /// * `cfg` - a ConfigFile that holds the configuration data
    ///
    /// # Returns
    ///
    /// * `Result<(), std::io::Error>` - a Result that returns an empty tuple or an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::cfg_cmd_runner::ConfigCmdRunner;
    ///
    /// let cfg_runner = ConfigCmdRunner::new("test_path/to/config/file".to_string());
    /// let cfg = ConfigFile::default();
    /// cfg_runner.setup_cfg(cfg);
    /// ```
    pub fn setup_cfg(&self, mut cfg: ConfigFile) -> Result<(), std::io::Error> {
        self.init_file()?;
        cfg = self.set_cfg_jira(cfg)?;
        self.set_cfg_auth(cfg)?;
        Ok(())
    }

    /// Shows the configuration file data
    ///
    /// # Arguments
    ///
    /// * `cfg` - a ConfigFile that holds the configuration data
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::cfg_cmd_runner::ConfigCmdRunner;
    ///
    /// let cfg = ConfigFile::default();
    ///
    /// let cfg_runner = ConfigCmdRunner::new("test_path/to/config/file".to_string());
    /// cfg_runner.show_cfg(cfg);
    /// ```
    pub fn show_cfg(&self, cfg: ConfigFile) {
        println!("Auth token: {}", cfg.get_auth_key());
        println!("Jira URL: {}", cfg.get_jira_url());
        println!(
            "Jira default resolution: {:?}",
            cfg.get_standard_resolution()
        );
        println!(
            "Jira default resolution comment: {:?}",
            cfg.get_standard_resolution_comment()
        );
    }

    #[cfg(test)]
    pub(crate) fn set_cfg_auth_with_reader<R, P>(
        &self,
        cfg: ConfigFile,
        reader: &mut R,
        password_reader: P,
    ) -> Result<ConfigFile, std::io::Error>
    where
        R: BufRead,
        P: FnMut() -> Result<String, std::io::Error>,
    {
        self.read_auth_from_sources(cfg, reader, password_reader)
    }

    #[cfg(test)]
    pub(crate) fn set_cfg_jira_with_reader<R>(
        &self,
        cfg: ConfigFile,
        reader: &mut R,
    ) -> Result<ConfigFile, std::io::Error>
    where
        R: BufRead,
    {
        self.read_jira_from_reader(cfg, reader)
    }
}
