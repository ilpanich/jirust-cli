use crate::args::commands::ProjectArgs;
use crate::config::config_file::{AuthData, ConfigFile};
use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::issues_api::{
    get_create_issue_meta_issue_type_id, get_create_issue_meta_issue_types,
};
use jira_v3_openapi::apis::projects_api::search_projects;
use jira_v3_openapi::models::{
    project::Project, FieldCreateMetadata, IssueTypeIssueCreateMetadata,
};

/// Project command runner struct.
///
/// This struct is responsible for holding the project commands parameters
/// and it is used to pass the parameters to the project commands runner.
pub struct ProjectCmdRunner {
    cfg: Configuration,
}

/// Project command runner implementation.
///
/// # Methods
///
/// * `new` - Creates a new instance of the ProjectCmdRunner struct.
/// * `list_jira_projects` - Lists Jira projects.
/// * `get_jira_project_issue_types` - Gets Jira project issue types.
/// * `get_jira_project_issue_type_id` - Gets Jira project issue fields by issue type ID.
impl ProjectCmdRunner {
    /// Creates a new instance of the ProjectCmdRunner struct.
    ///
    /// # Arguments
    ///
    /// * `cfg_file` - A ConfigFile struct.
    ///
    /// # Returns
    ///
    /// * A new instance of the ProjectCmdRunner struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::project_cmd_runner::ProjectCmdRunner;
    /// use toml::Table;
    ///
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    ///
    /// let project_cmd_runner = ProjectCmdRunner::new(cfg_file);
    /// ```
    pub fn new(cfg_file: ConfigFile) -> ProjectCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        ProjectCmdRunner { cfg: config }
    }

    /// Lists Jira projects.
    ///
    /// # Arguments
    ///
    /// * `params` - A ProjectCmdParams struct.
    ///
    /// # Returns
    ///
    /// * A Result with a vector of Project structs or an error message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::project_cmd_runner::{ProjectCmdRunner, ProjectCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let project_cmd_runner = ProjectCmdRunner::new(cfg_file);
    /// let params = ProjectCmdParams::new();
    ///
    /// let projects = project_cmd_runner.list_jira_projects(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn list_jira_projects(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let page_size = Some(params.projects_page_size.unwrap_or(10));
        let page_offset = Some(i64::from(params.projects_page_offset.unwrap_or(0)));
        match search_projects(
            &self.cfg,
            page_offset,
            page_size,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
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
    }

    /// Gets Jira project issue types.
    ///
    /// # Arguments
    ///
    /// * `params` - A ProjectCmdParams struct.
    ///
    /// # Returns
    /// * A Result with a vector of IssueTypeIssueCreateMetadata structs or an error message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::project_cmd_runner::{ProjectCmdRunner, ProjectCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let project_cmd_runner = ProjectCmdRunner::new(cfg_file);
    /// let params = ProjectCmdParams::new();
    ///
    /// let issue_types = project_cmd_runner.get_jira_project_issue_types(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn get_jira_project_issue_types(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<IssueTypeIssueCreateMetadata>, Box<dyn std::error::Error>> {
        let page_size = Some(params.projects_page_size.unwrap_or(10));
        let page_offset = Some(params.projects_page_offset.unwrap_or(0));
        match get_create_issue_meta_issue_types(
            &self.cfg,
            &params.project_key.expect("Project Key is required!"),
            page_offset,
            page_size,
        )
        .await?
        .issue_types
        {
            Some(issue_types) => Ok(issue_types),
            None => Ok(vec![]),
        }
    }

    /// Gets Jira project issue fields by issue type id.
    ///
    /// # Arguments
    ///
    /// * `params` - A ProjectCmdParams struct.
    ///
    /// # Returns
    ///
    /// * A Result with a vector of FieldCreateMetadata structs or an error message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::project_cmd_runner::{ProjectCmdRunner, ProjectCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("auth_token".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let project_cmd_runner = ProjectCmdRunner::new(cfg_file);
    /// let params = ProjectCmdParams::new();
    ///
    /// let issue_fields = project_cmd_runner.get_jira_project_issue_type_id(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn get_jira_project_issue_type_id(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<FieldCreateMetadata>, Box<dyn std::error::Error>> {
        let page_size = Some(params.projects_page_size.unwrap_or(10));
        let page_offset = Some(params.projects_page_offset.unwrap_or(0));
        match get_create_issue_meta_issue_type_id(
            &self.cfg,
            &params.project_key.expect("Project Key is required!"),
            &params.project_issue_type.expect("Issue Type is required!"),
            page_offset,
            page_size,
        )
        .await?
        .fields
        {
            Some(id) => Ok(id),
            None => Ok(vec![]),
        }
    }
}

/// This struct defines the parameters for the Project commands
///
/// # Fields
///
/// * `project_key` - The project key, **required** for get project issue types and issue fields commands.
/// * `project_issue_type` - The project issue type, **required** for get issue fields command.
/// * `projects_page_size` - The page size for the project command, optional.
/// * `projects_page_offset` - The page offset for the project command, optional.
pub struct ProjectCmdParams {
    pub project_key: Option<String>,
    pub project_issue_type: Option<String>,
    pub projects_page_size: Option<i32>,
    pub projects_page_offset: Option<i32>,
}

/// Implementation of the ProjectCmdParams struct
///
/// # Methods
///
/// * `new` - Creates a new ProjectCmdParams struct.
///
impl ProjectCmdParams {
    /// Creates a new ProjectCmdParams struct instance.
    ///
    /// # Returns
    ///
    /// * A ProjectCmdParams struct instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::project_cmd_runner::ProjectCmdParams;
    ///
    /// let params = ProjectCmdParams::new();
    /// ```
    pub fn new() -> ProjectCmdParams {
        ProjectCmdParams {
            project_key: None,
            project_issue_type: None,
            projects_page_size: None,
            projects_page_offset: None,
        }
    }
}

/// Implementation of the From trait for the ProjectCmdParams struct
/// to convert from ProjectArgs to ProjectCmdParams.
impl From<&ProjectArgs> for ProjectCmdParams {
    /// Converts from ProjectArgs to ProjectCmdParams.
    ///
    /// # Arguments
    ///
    /// * `value` - A ProjectArgs struct.
    ///
    /// # Returns
    ///
    /// * A ProjectCmdParams struct instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::project_cmd_runner::ProjectCmdParams;
    /// use jirust_cli::args::commands::{ProjectArgs, ProjectActionValues, PaginationArgs, OutputArgs};
    ///
    /// let project_args = ProjectArgs {
    ///     project_act: ProjectActionValues::List,
    ///     project_key: Some("project_key".to_string()),
    ///     project_issue_type: Some("project_issue_type".to_string()),
    ///     pagination: PaginationArgs { page_size: Some(10), page_offset: None },
    ///     output: OutputArgs { output: None },
    /// };
    ///
    /// let params = ProjectCmdParams::from(&project_args);
    ///
    /// assert_eq!(params.project_key, Some("project_key".to_string()));
    /// assert_eq!(params.project_issue_type, Some("project_issue_type".to_string()));
    /// assert_eq!(params.projects_page_size, Some(10));
    /// assert_eq!(params.projects_page_offset, Some(0));
    /// ```
    fn from(value: &ProjectArgs) -> Self {
        ProjectCmdParams {
            project_key: value.project_key.clone(),
            project_issue_type: value.project_issue_type.clone(),
            projects_page_size: value.pagination.page_size,
            projects_page_offset: Some(
                i32::try_from(value.pagination.page_offset.unwrap_or(0))
                    .expect("Invalid page offset, should fit an i32!"),
            ),
        }
    }
}

/// Implementation of the Default trait for the ProjectCmdParams struct
impl Default for ProjectCmdParams {
    /// Creates a default ProjectCmdParams struct instance.
    ///
    /// # Returns
    ///
    /// * A ProjectCmdParams struct instance with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::project_cmd_runner::ProjectCmdParams;
    ///
    /// let params = ProjectCmdParams::default();
    ///
    /// assert_eq!(params.project_key, None);
    /// assert_eq!(params.project_issue_type, None);
    /// assert_eq!(params.projects_page_size, None);
    /// assert_eq!(params.projects_page_offset, None);
    /// ```
    fn default() -> Self {
        ProjectCmdParams::new()
    }
}
