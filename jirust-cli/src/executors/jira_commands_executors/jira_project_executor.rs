use jira_v3_openapi::models::Project;

use crate::{
    args::commands::{ProjectActionValues, ProjectArgs},
    config::config_file::ConfigFile,
    runners::jira_cmd_runners::project_cmd_runner::{ProjectCmdParams, ProjectCmdRunner},
    utils::PrintableData,
};

use std::io::Error;

use super::ExecJiraCommand;

/// ProjectExecutor is responsible for executing the Jira project-related commands
///
/// # Fields
///
/// * `project_cmd_runner` - the runner responsible for running the Jira project-related commands
/// * `project_action` - the action to be performed on the Jira project
/// * `project_args` - the arguments passed to the Jira project command
pub struct ProjectExecutor {
    /// project_cmd_runner is the runner responsible for running the Jira project-related commands
    project_cmd_runner: ProjectCmdRunner,
    /// project_action is the action to be performed on the Jira project
    project_action: ProjectActionValues,
    /// project_args are the arguments passed to the Jira project command
    project_args: ProjectArgs,
}

/// ProjectExecutor implementation
///
/// #Methods
///
/// * `new(cfg_data: ConfigFile, project_action: ProjectActionValues, project_args: ProjectArgs) -> Self` - creates a new ProjectExecutor instance
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl ProjectExecutor {
    /// Creates a new ProjectExecutor instance
    ///
    /// # Arguments
    ///
    /// * `cfg_data: ConfigFile` - configuration file data
    /// * `project_action: ProjectActionValues` - project action
    /// * `project_args: ProjectArgs` - project arguments
    ///
    /// # Returns
    ///
    /// * `Self` - a new ProjectExecutor instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::jira_project_executor::ProjectExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{ProjectActionValues, ProjectArgs, PaginationArgs, OutputArgs};
    ///
    /// let cfg_data = ConfigFile::default();
    /// let project_action = ProjectActionValues::List;
    /// let project_args = ProjectArgs {
    ///     project_act: project_action.clone(),
    ///     project_key: None,
    ///     project_issue_type: None,
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
    ///     pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///     output: OutputArgs { output_format: None, output_type: None },
    /// };
    ///
    /// let project_executor = ProjectExecutor::new(cfg_data, project_action, project_args);
    /// ```
    pub fn new(
        cfg_data: ConfigFile,
        project_action: ProjectActionValues,
        project_args: ProjectArgs,
    ) -> Self {
        let project_cmd_runner = ProjectCmdRunner::new(&cfg_data);
        Self {
            project_cmd_runner,
            project_action,
            project_args,
        }
    }
}

/// Implementation of the ExecJiraCommand trait for ProjectExecutor
/// This trait is responsible for executing the Jira command
///
/// # Methods
///
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl ExecJiraCommand for ProjectExecutor {
    /// Executes the Jira command
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - a result with no return value in case of success or an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::ExecJiraCommand;
    /// use jirust_cli::executors::jira_commands_executors::jira_project_executor::ProjectExecutor;
    /// use jirust_cli::args::commands::{ProjectActionValues, ProjectArgs, PaginationArgs, OutputArgs};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_data = ConfigFile::default();
    /// let project_action = ProjectActionValues::List;
    /// let project_args = ProjectArgs {
    ///     project_act: project_action.clone(),
    ///     project_key: None,
    ///     project_issue_type: None,
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
    ///     pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///     output: OutputArgs { output_format: None, output_type: None },
    /// };
    ///
    /// let project_executor = ProjectExecutor::new(cfg_data, project_action, project_args);
    ///
    /// let res = project_executor.exec_jira_command().await?;
    /// # Ok(())
    /// # })
    /// # }
    ///
    async fn exec_jira_command(&self) -> Result<Vec<PrintableData>, Box<dyn std::error::Error>> {
        match self.project_action {
            ProjectActionValues::Create => {
                match self
                    .project_cmd_runner
                    .create_jira_project(ProjectCmdParams::from(&self.project_args))
                    .await
                {
                    Ok(new_project) => {
                        let mut project = Project::new();
                        project.id = Some(format!("{}", new_project.id));
                        project.key = Some(new_project.key);
                        Ok(vec![PrintableData::Project {
                            projects: vec![project],
                        }])
                    }
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error creating project: {err}"
                    )))),
                }
            }
            ProjectActionValues::List => {
                match self
                    .project_cmd_runner
                    .list_jira_projects(ProjectCmdParams::from(&self.project_args))
                    .await
                {
                    Ok(projects) => Ok(vec![PrintableData::Project { projects }]),
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error listing projects: {err}"
                    )))),
                }
            }
            ProjectActionValues::GetIssueTypes => {
                match self
                    .project_cmd_runner
                    .get_jira_project_issue_types(ProjectCmdParams::from(&self.project_args))
                    .await
                {
                    Ok(issue_types) => Ok(vec![PrintableData::IssueType { issue_types }]),
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error listing issue types: {err}"
                    )))),
                }
            }
            ProjectActionValues::GetIssueTypeFields => {
                match self
                    .project_cmd_runner
                    .get_jira_project_issue_type_id(ProjectCmdParams::from(&self.project_args))
                    .await
                {
                    Ok(issue_type_fields) => {
                        Ok(vec![PrintableData::IssueTypeField { issue_type_fields }])
                    }
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error listing issue type fields: {err}"
                    )))),
                }
            }
        }
    }
}
