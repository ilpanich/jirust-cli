use std::io::Error;

use crate::args::commands::ProjectArgs;
use crate::config::config_file::{AuthData, ConfigFile};
use async_trait::async_trait;
use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::issues_api::{
    get_create_issue_meta_issue_type_id, get_create_issue_meta_issue_types,
};
use jira_v3_openapi::apis::projects_api::{create_project, search_projects};
use jira_v3_openapi::models::create_project_details::{AssigneeType, ProjectTypeKey};
use jira_v3_openapi::models::{CreateProjectDetails, ProjectIdentifiers};
use jira_v3_openapi::models::{
    FieldCreateMetadata, IssueTypeIssueCreateMetadata, project::Project,
};

#[cfg(test)]
use mockall::automock;

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
    /// let project_cmd_runner = ProjectCmdRunner::new(&cfg_file);
    /// ```
    pub fn new(cfg_file: &ConfigFile) -> ProjectCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        ProjectCmdRunner { cfg: config }
    }

    /// Create a new Jira project using the provided parameters.
    ///
    /// # Arguments
    /// * `params` - The parameters for creating the project.
    ///
    /// # Returns
    /// A `Result` containing the project identifiers if successful, or an error if failed.
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
    /// let project_cmd_runner = ProjectCmdRunner::new(&cfg_file);
    ///
    /// let mut params = ProjectCmdParams::new();
    /// params.project_key = Some("TEST".to_string());
    /// params.project_name = Some("Test Project".to_string());
    /// params.project_description = Some("This is a test project".to_string());
    /// params.project_field_configuration_id = Some(12345);
    /// params.project_issue_security_scheme_id = Some(67890);
    /// params.project_issue_type_scheme_id = Some(54321);
    ///
    /// let projects = project_cmd_runner.create_jira_project(params).await?;
    ///
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn create_jira_project(
        &self,
        params: ProjectCmdParams,
    ) -> Result<ProjectIdentifiers, Box<dyn std::error::Error>> {
        let p_key = params
            .project_key
            .as_deref()
            .ok_or_else(|| Box::new(Error::other("Error creating project: Empty project key")))?;
        let p_name = params
            .project_name
            .as_deref()
            .ok_or_else(|| Box::new(Error::other("Error creating project: Empty project name")))?;

        let mut project_data = CreateProjectDetails::new(p_key.to_string(), p_name.to_string());
        project_data.description = params.project_description;
        project_data.field_configuration_scheme = params.project_field_configuration_id;
        project_data.issue_security_scheme = params.project_issue_security_scheme_id;
        project_data.issue_type_scheme = params.project_issue_type_scheme_id;
        project_data.issue_type_screen_scheme = params.project_issue_type_screen_scheme_id;
        project_data.notification_scheme = params.project_notification_scheme_id;
        project_data.permission_scheme = params.project_permission_scheme_id;
        project_data.workflow_scheme = params.project_workflow_scheme_id;
        project_data.lead_account_id = params.project_lead_account_id;
        project_data.assignee_type = if params
            .project_assignee_type
            .unwrap_or("unassigned".to_string())
            == "lead"
        {
            Some(AssigneeType::ProjectLead)
        } else {
            Some(AssigneeType::Unassigned)
        };
        project_data.project_type_key = Some(ProjectTypeKey::Software);
        Ok(create_project(&self.cfg, project_data).await?)
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
    /// let project_cmd_runner = ProjectCmdRunner::new(&cfg_file);
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
    /// let project_cmd_runner = ProjectCmdRunner::new(&cfg_file);
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
        let p_key = if let Some(key) = &params.project_key {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error retrieving project issue types: Empty project key".to_string(),
            )));
        };
        let page_size = Some(params.projects_page_size.unwrap_or(10));
        let page_offset = Some(params.projects_page_offset.unwrap_or(0));
        match get_create_issue_meta_issue_types(&self.cfg, p_key, page_offset, page_size)
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
    /// let project_cmd_runner = ProjectCmdRunner::new(&cfg_file);
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
        let p_key = if let Some(key) = &params.project_key {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error retrieving project issue types ids: Empty project key".to_string(),
            )));
        };
        let issue_type = if let Some(key) = &params.project_issue_type {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error retrieving project issue types ids: Empty project issue type key"
                    .to_string(),
            )));
        };
        let page_size = Some(params.projects_page_size.unwrap_or(10));
        let page_offset = Some(params.projects_page_offset.unwrap_or(0));
        match get_create_issue_meta_issue_type_id(
            &self.cfg,
            p_key,
            issue_type,
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
    /// Jira project key when acting on an existing project.
    pub project_key: Option<String>,
    /// Issue type identifier used when querying issue type fields.
    pub project_issue_type: Option<String>,
    /// Name for the project being created.
    pub project_name: Option<String>,
    /// Description for the project being created.
    pub project_description: Option<String>,
    /// Field configuration id to associate to the project.
    pub project_field_configuration_id: Option<i64>,
    /// Issue security scheme id linked to the project.
    pub project_issue_security_scheme_id: Option<i64>,
    /// Issue type scheme id to apply to the project.
    pub project_issue_type_scheme_id: Option<i64>,
    /// Issue type screen scheme id to apply to the project.
    pub project_issue_type_screen_scheme_id: Option<i64>,
    /// Notification scheme id to attach to the project.
    pub project_notification_scheme_id: Option<i64>,
    /// Permission scheme id to attach to the project.
    pub project_permission_scheme_id: Option<i64>,
    /// Workflow scheme id to attach to the project.
    pub project_workflow_scheme_id: Option<i64>,
    /// Account id for the project lead.
    pub project_lead_account_id: Option<String>,
    /// Default assignee type for the project.
    pub project_assignee_type: Option<String>,
    /// Page size used when listing projects.
    pub projects_page_size: Option<i32>,
    /// Page offset used when listing projects.
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
            project_name: None,
            project_description: None,
            project_field_configuration_id: None,
            project_issue_security_scheme_id: None,
            project_issue_type_scheme_id: None,
            project_issue_type_screen_scheme_id: None,
            project_notification_scheme_id: None,
            project_permission_scheme_id: None,
            project_workflow_scheme_id: None,
            project_lead_account_id: None,
            project_assignee_type: None,
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
    ///     project_act: ProjectActionValues::GetIssueTypeFields,
    ///     project_key: Some("project_key".to_string()),
    ///     project_issue_type: Some("project_issue_type".to_string()),
    ///     project_name: None,
    ///     project_description: None,
    ///     project_field_configuration_id: None,
    ///     project_issue_security_scheme_id: None,
    ///     project_issue_type_scheme_id: None,
    ///     project_issue_type_screen_scheme_id: None,
    ///     project_notification_scheme_id: None,
    ///     project_permission_scheme_id: None,
    ///     project_workflow_scheme_id: None,
    ///     project_lead_account_id: None,
    ///     project_assignee_type: None,
    ///     pagination: PaginationArgs { page_size: Some(10), page_offset: None },
    ///     output: OutputArgs { output_format: None, output_type: None },
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
            project_name: value.project_name.clone(),
            project_description: value.project_description.clone(),
            project_field_configuration_id: value.project_field_configuration_id,
            project_issue_security_scheme_id: value.project_issue_security_scheme_id,
            project_issue_type_scheme_id: value.project_issue_type_scheme_id,
            project_issue_type_screen_scheme_id: value.project_issue_type_screen_scheme_id,
            project_notification_scheme_id: value.project_notification_scheme_id,
            project_permission_scheme_id: value.project_permission_scheme_id,
            project_workflow_scheme_id: value.project_workflow_scheme_id,
            project_lead_account_id: value.project_lead_account_id.clone(),
            project_assignee_type: value.project_assignee_type.clone(),
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

/// API contract for performing Jira project operations.
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ProjectCmdRunnerApi: Send + Sync {
    /// Creates a Jira project with the provided parameters.
    async fn create_jira_project(
        &self,
        params: ProjectCmdParams,
    ) -> Result<ProjectIdentifiers, Box<dyn std::error::Error>>;

    /// Lists Jira projects using pagination if provided.
    async fn list_jira_projects(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>>;

    /// Retrieves issue types available for a project key.
    async fn get_jira_project_issue_types(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<IssueTypeIssueCreateMetadata>, Box<dyn std::error::Error>>;

    /// Retrieves fields for a specific project issue type.
    async fn get_jira_project_issue_type_id(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<FieldCreateMetadata>, Box<dyn std::error::Error>>;
}

#[async_trait(?Send)]
impl ProjectCmdRunnerApi for ProjectCmdRunner {
    async fn create_jira_project(
        &self,
        params: ProjectCmdParams,
    ) -> Result<ProjectIdentifiers, Box<dyn std::error::Error>> {
        ProjectCmdRunner::create_jira_project(self, params).await
    }

    async fn list_jira_projects(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        ProjectCmdRunner::list_jira_projects(self, params).await
    }

    async fn get_jira_project_issue_types(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<IssueTypeIssueCreateMetadata>, Box<dyn std::error::Error>> {
        ProjectCmdRunner::get_jira_project_issue_types(self, params).await
    }

    async fn get_jira_project_issue_type_id(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<FieldCreateMetadata>, Box<dyn std::error::Error>> {
        ProjectCmdRunner::get_jira_project_issue_type_id(self, params).await
    }
}
