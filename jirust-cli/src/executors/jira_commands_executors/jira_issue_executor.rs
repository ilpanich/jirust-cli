use crate::{
    args::commands::{IssueActionValues, IssueArgs},
    config::config_file::ConfigFile,
    runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdParams, IssueCmdRunner},
    utils::{print_data, OutputType, PrintableData},
};

use super::ExecJiraCommand;

pub struct IssueExecutor {
    issue_cmd_runner: IssueCmdRunner,
    issue_action: IssueActionValues,
    issue_args: IssueArgs,
}

impl IssueExecutor {
    pub fn new(
        cfg_data: ConfigFile,
        issue_action: IssueActionValues,
        issue_args: IssueArgs,
    ) -> Self {
        let issue_cmd_runner = IssueCmdRunner::new(cfg_data.clone());
        Self {
            issue_cmd_runner,
            issue_action,
            issue_args,
        }
    }
}

impl ExecJiraCommand for IssueExecutor {
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.issue_action {
            IssueActionValues::Assign => {
                let _res = self
                    .issue_cmd_runner
                    .assign_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
            IssueActionValues::Create => {
                let res = self
                    .issue_cmd_runner
                    .create_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
                print_data(
                    PrintableData::IssueCreated {
                        issues: (vec![res]),
                    },
                    self.issue_args.output.output,
                    OutputType::Basic,
                )
            }
            IssueActionValues::Delete => {
                let _res = self
                    .issue_cmd_runner
                    .delete_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
            IssueActionValues::Get => {
                let res = self
                    .issue_cmd_runner
                    .get_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
                print_data(
                    PrintableData::IssueData { issues: vec![res] },
                    self.issue_args.output.output,
                    OutputType::Single,
                )
            }
            IssueActionValues::Transition => {
                let _res = self
                    .issue_cmd_runner
                    .transition_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
            IssueActionValues::Update => {
                let _res = self
                    .issue_cmd_runner
                    .update_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
        }
        Ok(())
    }
}
