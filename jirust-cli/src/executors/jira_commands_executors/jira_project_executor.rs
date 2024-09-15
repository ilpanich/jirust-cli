use crate::{
    args::commands::{OutputValues, ProjectActionValues, ProjectArgs},
    config::config_file::ConfigFile,
    runners::jira_cmd_runners::project_cmd_runner::{ProjectCmdParams, ProjectCmdRunner},
    utils::{print_data, OutputType, PrintableData},
};

use super::ExecJiraCommand;

/// ProjectExecutor is responsible for executing the Jira project-related commands
pub struct ProjectExecutor {
    project_cmd_runner: ProjectCmdRunner,
    project_action: ProjectActionValues,
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
    /// use jirust_cli::args::commands::{ProjectActionValues, ProjectArgs, PaginationArgs};
    ///
    /// let cfg_data = ConfigFile::default();
    /// let project_action = ProjectActionValues::List;
    /// let project_args = ProjectArgs {
    ///     project_act: project_action.clone(),
    ///     project_key: Some("project_key".to_string()),
    ///     project_issue_type: None,
    ///     pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    /// };
    ///
    /// let project_executor = ProjectExecutor::new(cfg_data, project_action, project_args);
    /// ```
    pub fn new(
        cfg_data: ConfigFile,
        project_action: ProjectActionValues,
        project_args: ProjectArgs,
    ) -> Self {
        let project_cmd_runner = ProjectCmdRunner::new(cfg_data.clone());
        Self {
            project_cmd_runner,
            project_action,
            project_args,
        }
    }
}

/// Implementation of the ExecJiraCommand trait for ProjectExecutor
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
    /// use jirust_cli::args::commands::{ProjectActionValues, ProjectArgs, PaginationArgs};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_data = ConfigFile::default();
    /// let project_action = ProjectActionValues::List;
    /// let project_args = ProjectArgs {
    ///     project_act: project_action.clone(),
    ///     project_key: Some("project_key".to_string()),
    ///     project_issue_type: None,
    ///     pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    /// };
    ///
    /// let project_executor = ProjectExecutor::new(cfg_data, project_action, project_args);
    ///
    /// let res = project_executor.exec_jira_command().await?;
    /// # Ok(())
    /// # })
    /// # }
    ///
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.project_action {
            ProjectActionValues::List => {
                let res = self
                    .project_cmd_runner
                    .list_jira_projects(ProjectCmdParams::from(&self.project_args))
                    .await?;
                print_data(
                    PrintableData::Project { projects: res },
                    self.project_args
                        .output
                        .output
                        .unwrap_or(OutputValues::Json),
                    OutputType::Full,
                );
            }
            ProjectActionValues::GetIssueTypes => {
                let res = self
                    .project_cmd_runner
                    .get_jira_project_issue_types(ProjectCmdParams::from(&self.project_args))
                    .await?;
                print_data(
                    PrintableData::IssueType { issue_types: res },
                    self.project_args
                        .output
                        .output
                        .unwrap_or(OutputValues::Json),
                    OutputType::Full,
                );
            }
            ProjectActionValues::GetIssueTypeFields => {
                let res = self
                    .project_cmd_runner
                    .get_jira_project_issue_type_id(ProjectCmdParams::from(&self.project_args))
                    .await?;
                print_data(
                    PrintableData::IssueTypeField {
                        issue_type_fields: res,
                    },
                    self.project_args
                        .output
                        .output
                        .unwrap_or(OutputValues::Json),
                    OutputType::Full,
                );
            }
        }
        Ok(())
    }
}
