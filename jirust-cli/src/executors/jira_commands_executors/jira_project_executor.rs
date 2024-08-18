use crate::{
    args::commands::{ProjectActionValues, ProjectArgs},
    config::config_file::ConfigFile,
    runners::jira_cmd_runners::project_cmd_runner::{ProjectCmdParams, ProjectCmdRunner},
    utils::{table_printer::*, TablePrintable},
};

use super::ExecJiraCommand;

pub struct ProjectExecutor {
    project_cmd_runner: ProjectCmdRunner,
    project_action: ProjectActionValues,
    project_args: ProjectArgs,
}

impl ProjectExecutor {
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

impl ExecJiraCommand for ProjectExecutor {
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.project_action {
            ProjectActionValues::List => {
                let res = self
                    .project_cmd_runner
                    .list_jira_projects(ProjectCmdParams::from(&self.project_args))
                    .await?;
                print_table_full(TablePrintable::Project { projects: res });
            }
            ProjectActionValues::GetIssueTypes => {
                let res = self
                    .project_cmd_runner
                    .get_jira_project_issue_types(ProjectCmdParams::from(&self.project_args))
                    .await?;
                print_table_full(TablePrintable::IssueType { issue_types: res });
            }
            ProjectActionValues::GetIssueTypeFields => {
                let res = self
                    .project_cmd_runner
                    .get_jira_project_issue_type_id(ProjectCmdParams::from(&self.project_args))
                    .await?;
                print_table_full(TablePrintable::IssueTypeFields {
                    issue_type_fields: res,
                });
            }
        }
        Ok(())
    }
}
