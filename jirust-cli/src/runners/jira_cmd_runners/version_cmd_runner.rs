use crate::args::commands::VersionArgs;
use crate::config::config_file::{AuthData, ConfigFile};
use chrono::{DateTime, Utc};
use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::project_versions_api::*;
use jira_v3_openapi::apis::Error;
use jira_v3_openapi::models::{DeleteAndReplaceVersionBean, Version};

/// Version command runner struct
/// This struct is responsible for running the version related command
pub struct VersionCmdRunner {
    cfg: Configuration,
}

/// Version command parameters struct
/// This struct is responsible for holding the version command runner parameters
/// and it is used to pass the parameters to the version command runner
///
/// # Methods
///
/// * `new` - This method creates a new instance of the VersionCmdRunner struct
/// * `create_jira_version` - This method creates a new Jira version
/// * `get_jira_version` - This method gets a Jira version
/// * `list_jira_versions` - This method lists Jira versions
/// * `update_jira_version` - This method updates a Jira version
/// * `delete_jira_version` - This method deletes a Jira version
/// * `release_jira_version` - This method releases a Jira version
/// `archive_jira_version` - This method archives a Jira version
impl VersionCmdRunner {
    /// This method creates a new instance of the VersionCmdRunner struct
    ///
    /// # Arguments
    ///
    /// * `cfg_file` - A ConfigFile struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdRunner struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    ///
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string());
    ///
    /// let version_cmd_runner = VersionCmdRunner::new(cfg_file);
    /// ```
    pub fn new(cfg_file: ConfigFile) -> VersionCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        VersionCmdRunner { cfg: config }
    }

    /// This method creates a new Jira version with the given parameters
    /// and returns the created version
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a Version struct or a Box<dyn std::error::Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string());
    /// let version_cmd_runner = VersionCmdRunner::new(cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let version = version_cmd_runner.create_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn create_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Box<dyn std::error::Error>> {
        let version = Version {
            project: Some(params.project),
            name: Some(
                params
                    .version_name
                    .expect("VersionName is mandatory on cretion!"),
            ),
            description: params.version_description,
            start_date: params.version_start_date,
            release_date: params.version_release_date,
            archived: params.version_archived,
            released: params.version_released,
            ..Default::default()
        };
        Ok(create_version(&self.cfg, version).await?)
    }

    /// This method gets a Jira version with the given parameters
    /// and returns the version
    /// If the version is not found, it returns an error
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a Version struct or an Error<GetVersionError>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string());
    /// let version_cmd_runner = VersionCmdRunner::new(cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let version = version_cmd_runner.get_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn get_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Error<GetVersionError>> {
        Ok(get_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            None,
        )
        .await?)
    }

    /// This method lists Jira versions with the given parameters
    /// and returns the versions
    /// If there are no versions, it returns an empty vector
    /// If the version is not found, it returns an error
    /// If the version page size is given, it returns the paginated versions
    /// Otherwise, it returns all versions
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a vector of Version structs or a Box<dyn std::error::Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string());
    /// let version_cmd_runner = VersionCmdRunner::new(cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let versions = version_cmd_runner.list_jira_versions(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn list_jira_versions(
        &self,
        params: VersionCmdParams,
    ) -> Result<Vec<Version>, Box<dyn std::error::Error>> {
        if Option::is_some(&params.version_page_size) {
            match get_project_versions_paginated(
                &self.cfg,
                params.project.as_str(),
                params.version_page_offset,
                params.version_page_size,
                None,
                None,
                None,
                None,
            )
            .await?
            .values
            {
                Some(values) => Ok(values),
                None => Ok(vec![]),
            }
        } else {
            Ok(get_project_versions(&self.cfg, params.project.as_str(), None).await?)
        }
    }

    /// This method updates a Jira version with the given parameters
    /// and returns the updated version
    /// If the version is not found, it returns an error
    /// If the version ID is not given, it returns an error
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a Version struct or an Error<UpdateVersionError>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string());
    /// let version_cmd_runner = VersionCmdRunner::new(cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let version = version_cmd_runner.update_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn update_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Error<UpdateVersionError>> {
        let version = Version {
            id: Some(params.version_id.clone().expect("VersionID is mandatory!")),
            name: params.version_name,
            description: params.version_description,
            start_date: params.version_start_date,
            release_date: params.version_release_date,
            archived: params.version_archived,
            released: params.version_released,
            ..Default::default()
        };
        Ok(update_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            version,
        )
        .await?)
    }

    /// This method deletes a Jira version with the given parameters
    /// and returns the status of the deletion
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a serde_json::Value or an Error<DeleteAndReplaceVersionError>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string());
    /// let version_cmd_runner = VersionCmdRunner::new(cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let status = version_cmd_runner.delete_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn delete_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<serde_json::Value, Error<DeleteAndReplaceVersionError>> {
        match delete_and_replace_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            DeleteAndReplaceVersionBean::new(),
        )
        .await
        {
            Ok(_) => Ok(serde_json::json!({"status": "success"})),
            Err(e) => match e {
                Error::Serde(_) => Ok(
                    serde_json::json!({"status": "success", "warning": "Version was deleted, some issues in deserializing response!"}),
                ),
                _ => Err(e),
            },
        }
    }
}

/// This struct defines the parameters for the Version commands
pub struct VersionCmdParams {
    pub project: String,
    pub project_id: Option<i64>,
    pub version_name: Option<String>,
    pub version_id: Option<String>,
    pub version_description: Option<String>,
    pub version_start_date: Option<String>,
    pub version_release_date: Option<String>,
    pub version_archived: Option<bool>,
    pub version_released: Option<bool>,
    pub version_page_size: Option<i32>,
    pub version_page_offset: Option<i64>,
}

/// Implementation of the VersionCmdParams struct
///
/// # Methods
///
/// * `new` - returns a new VersionCmdParams struct
/// * `merge_args` - merges the current version with the optional arguments
/// * `mark_released` - marks the version as released
/// * `mark_archived` - marks the version as archived
impl VersionCmdParams {
    /// This method returns a new VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let params = VersionCmdParams::new();
    /// ```
    pub fn new() -> VersionCmdParams {
        VersionCmdParams {
            project: "".to_string(),
            project_id: None,
            version_name: None,
            version_id: None,
            version_description: None,
            version_start_date: None,
            version_release_date: None,
            version_archived: None,
            version_released: None,
            version_page_size: None,
            version_page_offset: None,
        }
    }

    /// This method merges the current version with the optional arguments
    /// and returns a VersionCmdParams struct
    /// If the optional arguments are not given, it uses the current version values
    ///
    /// # Arguments
    ///
    /// * `current_version` - A Version struct
    /// * `opt_args` - An Option<&VersionArgs> struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::args::commands::{VersionArgs, VersionActionValues};
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let mut current_version: Version = Version::new();
    /// current_version.id = Some("12345".to_string());
    /// current_version.project_id = Some(9876);
    /// current_version.project = Some("TEST_PROJECT".to_string());
    /// current_version.name = Some("v1.0".to_string());
    /// current_version.description = Some("This is the first version".to_string());
    ///
    /// let opt_args = VersionArgs {
    ///   version_act: VersionActionValues::List,
    ///   project: "project_key".to_string(),
    ///   project_id: None,
    ///   version_id: Some("97531".to_string()),
    ///   version_name: Some("version_name".to_string()),
    ///   version_description: Some("version_description".to_string()),
    ///   version_start_date: None,
    ///   version_release_date: None,
    ///   version_archived: None,
    ///   version_released: Some(true),
    ///   version_page_size: None,
    ///   version_page_offset: None,
    /// };
    ///
    /// let params = VersionCmdParams::merge_args(current_version, Some(&opt_args));
    ///
    /// assert_eq!(params.project, "TEST_PROJECT".to_string());
    /// assert_eq!(params.project_id, Some(9876));
    /// assert_eq!(params.version_id, Some("12345".to_string()));
    /// assert_eq!(params.version_name, Some("version_name".to_string()));
    /// assert_eq!(params.version_description, Some("version_description".to_string()));
    /// assert_eq!(params.version_released, Some(true));
    /// ```
    pub fn merge_args(
        current_version: Version,
        opt_args: Option<&VersionArgs>,
    ) -> VersionCmdParams {
        match opt_args {
            Some(args) => VersionCmdParams {
                project: current_version.project.clone().unwrap_or("".to_string()),
                project_id: current_version.project_id.clone(),
                version_id: current_version.id,
                version_name: if Option::is_some(&args.version_name) {
                    args.version_name.clone()
                } else {
                    current_version.name
                },
                version_description: if Option::is_some(&args.version_description) {
                    args.version_description.clone()
                } else {
                    current_version.description
                },
                version_start_date: if Option::is_some(&args.version_start_date) {
                    args.version_start_date.clone()
                } else {
                    current_version.start_date
                },
                version_release_date: if Option::is_some(&args.version_release_date) {
                    args.version_release_date.clone()
                } else {
                    current_version.release_date
                },
                version_archived: if Option::is_some(&args.version_archived) {
                    args.version_archived
                } else {
                    current_version.archived
                },
                version_released: if Option::is_some(&args.version_released) {
                    args.version_released
                } else {
                    current_version.released
                },
                version_page_size: None,
                version_page_offset: None,
            },
            None => VersionCmdParams {
                project: current_version.project.clone().unwrap_or("".to_string()),
                project_id: current_version.project_id.clone(),
                version_id: current_version.id,
                version_name: current_version.name,
                version_description: current_version.description,
                version_start_date: current_version.start_date,
                version_release_date: current_version.release_date,
                version_archived: current_version.archived,
                version_released: current_version.released,
                version_page_size: None,
                version_page_offset: None,
            },
        }
    }

    /// This method marks the version as released
    /// and returns a VersionCmdParams struct
    /// It sets the version_released and version_release_date fields
    /// with the current date
    ///
    /// # Arguments
    ///
    /// * `version` - A Version struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let mut version: Version = Version::new();
    /// version.id = Some("12345".to_string());
    /// version.project_id = Some(9876);
    /// version.project = Some("TEST_PROJECT".to_string());
    /// version.name = Some("v1.0".to_string());
    /// version.description = Some("This is the first version".to_string());
    ///
    /// assert_eq!(version.released, None);
    ///
    /// let params = VersionCmdParams::mark_released(version);
    ///
    /// assert_eq!(params.version_released, Some(true));
    /// ```
    pub fn mark_released(version: Version) -> VersionCmdParams {
        let mut version_to_release = Self::merge_args(version, None);
        version_to_release.version_released = Some(true);
        version_to_release.version_release_date = Some(Utc::now().format("%Y-%m-%d").to_string());
        version_to_release
    }

    /// This method marks the version as archived
    /// and returns a VersionCmdParams struct
    ///
    /// # Arguments
    ///
    /// * `version` - A Version struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let mut version: Version = Version::new();
    /// version.id = Some("12345".to_string());
    /// version.project_id = Some(9876);
    /// version.project = Some("TEST_PROJECT".to_string());
    /// version.name = Some("v1.0".to_string());
    /// version.description = Some("This is the first version".to_string());
    ///
    /// assert_eq!(version.archived, None);
    ///
    /// let params = VersionCmdParams::mark_archived(version);
    ///
    /// assert_eq!(params.version_archived, Some(true));
    /// ```
    ///
    pub fn mark_archived(version: Version) -> VersionCmdParams {
        let mut version_to_archive = Self::merge_args(version, None);
        version_to_archive.version_archived = Some(true);
        version_to_archive
    }
}

/// This trait defines the methods from for the Version command to VersionCmdParams struct
impl From<&VersionArgs> for VersionCmdParams {
    /// This method converts the VersionArgs struct to a VersionCmdParams struct
    /// and returns a VersionCmdParams struct
    ///
    /// # Arguments
    ///
    /// * `args` - A VersionArgs struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::args::commands::{VersionActionValues, VersionArgs};
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let version_args = VersionArgs {
    ///   version_act: VersionActionValues::List,
    ///   project: "project_key".to_string(),
    ///   project_id: None,
    ///   version_id: None,
    ///   version_name: Some("version_name".to_string()),
    ///   version_description: Some("version_description".to_string()),
    ///   version_start_date: None,
    ///   version_release_date: None,
    ///   version_archived: None,
    ///   version_released: None,
    ///   version_page_size: None,
    ///   version_page_offset: None,
    /// };
    ///
    /// let params = VersionCmdParams::from(&version_args);
    ///
    /// assert_eq!(params.project, "project_key".to_string());
    /// assert_eq!(params.version_name, Some("version_name".to_string()));
    /// assert_eq!(params.version_description, Some("version_description".to_string()));
    /// ```
    fn from(args: &VersionArgs) -> Self {
        let now: DateTime<Utc> = Utc::now();
        VersionCmdParams {
            project: args.project.clone(),
            project_id: args.project_id.clone(),
            version_name: args.version_name.clone(),
            version_id: args.version_id.clone(),
            version_description: args.version_description.clone(),
            version_start_date: Some(
                args.version_start_date
                    .clone()
                    .unwrap_or(now.format("%Y-%m-%d").to_string()),
            ),
            version_release_date: args.version_release_date.clone(),
            version_archived: args.version_archived.clone(),
            version_released: args.version_released.clone(),
            version_page_size: args.version_page_size.clone(),
            version_page_offset: args.version_page_offset.clone(),
        }
    }
}

/// This function allows to print the version details in a pretty way (full data)
/// It uses the prettytable library to print the version details
///
/// # Arguments
///
/// * `versions` - A Vector of Version structs
///
/// # Examples
///
/// ```
/// use jira_v3_openapi::models::Version;
/// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::print_table_full;
///
/// let versions: Vec<Version> = vec![Version::new()];
///
/// print_table_full(versions);
/// ```
///
pub fn print_table_full(versions: Vec<Version>) {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        bFC->"Project ID",
        bFc->"ID",
        bFm->"Name",
        bFw->"Description",
        bFy->"Start Date",
        bFr->"Release Date",
        bFb->"Archived",
        bFg->"Released"
    ]);
    for version in versions {
        table.add_row(row![
            FC->version.project_id.unwrap_or_default(),
            Fc->version.id.unwrap_or_default(),
            Fm->version.name.unwrap_or_default(),
            Fw->version.description.unwrap_or_default(),
            Fy->version.start_date.unwrap_or_default(),
            Fr->version.release_date.unwrap_or_default(),
            Fb->version.archived.unwrap_or_default(),
            Fg->version.released.unwrap_or_default()
        ]);
    }
    table.printstd();
}

/// This function allows to print the version details in a pretty way (basic data)
/// It uses the prettytable library to print the version details
///
/// # Arguments
///
/// * `versions` - A Vector of Version structs
///
/// # Examples
///
/// ```
/// use jira_v3_openapi::models::Version;
/// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::print_table_basic;
///
/// let versions: Vec<Version> = vec![Version::new()];
/// print_table_basic(versions);
/// ```
pub fn print_table_basic(versions: Vec<Version>) {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        bFC->"Project ID",
        bFc->"ID",
        bFm->"Name",
        bFy->"Start Date",
        bFr->"Release Date",
        bFb->"Archived",
        bFg->"Released"
    ]);
    for version in versions {
        table.add_row(row![
            FC->version.project_id.unwrap_or_default(),
            Fc->version.id.unwrap_or_default(),
            Fm->version.name.unwrap_or_default(),
            Fy->version.start_date.unwrap_or_default(),
            Fr->version.release_date.unwrap_or_default(),
            Fb->version.archived.unwrap_or_default(),
            Fg->version.released.unwrap_or_default()
        ]);
    }
    table.printstd();
}

/// This function allows to print the version details in a pretty way (single data)
/// It uses the prettytable library to print the version details
///
/// # Arguments
///
/// * `version` - A Version struct
///
/// # Examples
///
/// ```
/// use jira_v3_openapi::models::Version;
/// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::print_table_single;
///
/// let version: Version = Version::new();
/// print_table_single(version);
/// ```
pub fn print_table_single(version: Version) {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        FC->version.project_id.unwrap_or_default(),
        bFC->"Project ID",
        bFc->"ID",
        bFm->"Name",
        bFw->"Description",
        bFy->"Start Date",
        bFr->"Release Date",
        bFb->"Archived",
        bFg->"Released"
    ]);

    table.add_row(row![
        Fc->version.id.unwrap_or_default(),
        Fm->version.name.unwrap_or_default(),
        Fw->version.description.unwrap_or_default(),
        Fy->version.start_date.unwrap_or_default(),
        Fr->version.release_date.unwrap_or_default(),
        Fb->version.archived.unwrap_or_default(),
        Fg->version.released.unwrap_or_default()
    ]);

    table.printstd();
}
