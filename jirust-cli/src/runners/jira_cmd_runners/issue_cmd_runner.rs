use jira_v3_openapi::apis::issues_api::*;
use jira_v3_openapi::models::user::AccountType;
use jira_v3_openapi::models::{CreatedIssue, IssueBean, IssueTransition, Transitions, User};
use jira_v3_openapi::{apis::configuration::Configuration, models::IssueUpdateDetails};
use serde_json::Value;
use std::collections::HashMap;

use crate::args::commands::TransitionArgs;
use crate::{
    args::commands::IssueArgs,
    config::config_file::{AuthData, ConfigFile},
};

/// Issue command runner
/// This struct is responsible for running the issue command
/// It uses the Jira API to perform the operations
///
/// # Fields
///
/// * `cfg` - Configuration object
pub struct IssueCmdRunner {
    /// Configuration object
    cfg: Configuration,
}

/// Implementation of IssueCmdRunner
///
/// # Methods
///
/// * `new` - Creates a new instance of IssueCmdRunner
/// * `assign_jira_issue` - Assigns a Jira issue to a user
/// * `create_jira_issue` - Creates a Jira issue
/// * `delete_jira_issue` - Deletes a Jira issue
/// * `get_jira_issue` - Gets a Jira issue
/// * `transition_jira_issue` - Transitions a Jira issue
/// * `update_jira_issue` - Updates a Jira issue
/// * `get_issue_available_transitions` - Gets available transitions for a Jira issue
impl IssueCmdRunner {
    /// Creates a new instance of IssueCmdRunner
    ///
    /// # Arguments
    ///
    /// * `cfg_file` - Configuration file
    ///
    /// # Returns
    ///
    /// * `IssueCmdRunner` - Instance of IssueCmdRunner
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::IssueCmdRunner;
    /// use toml::Table;
    ///
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    ///
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    /// ```
    pub fn new(cfg_file: ConfigFile) -> IssueCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        IssueCmdRunner { cfg: config }
    }

    /// Assigns a Jira issue to a user
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `Value` - JSON value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    /// let mut params = IssueCmdParams::new();
    /// params.assignee = Some("assignee".to_string());
    ///
    /// let result = issue_cmd_runner.assign_jira_issue(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn assign_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let user_data = User {
            account_id: Some(params.assignee.expect("Assignee is required")),
            account_type: Some(AccountType::Atlassian),
            ..Default::default()
        };

        Ok(assign_issue(
            &self.cfg,
            params.issue_key.unwrap_or("".to_string()).as_str(),
            user_data,
        )
        .await?)
    }

    /// Creates a Jira issue
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `CreatedIssue` - Created issue
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    /// let params = IssueCmdParams::new();
    ///
    /// let result = issue_cmd_runner.create_jira_issue(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn create_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<CreatedIssue, Box<dyn std::error::Error>> {
        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: None,
            update: None,
        };
        Ok(create_issue(&self.cfg, issue_data, None).await?)
    }

    /// Deletes a Jira issue
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `()` - Empty tuple
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    /// let mut params = IssueCmdParams::new();
    /// params.issue_key = Some("issue_key".to_string());
    ///
    /// let result = issue_cmd_runner.delete_jira_issue(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn delete_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(delete_issue(
            &self.cfg,
            params.issue_key.expect("Issue key is required!").as_str(),
            Some("true"),
        )
        .await?)
    }

    /// Gets a Jira issue
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `IssueBean` - Jira issue
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    /// let mut params = IssueCmdParams::new();
    /// params.issue_key = Some("issue_key".to_string());
    ///
    /// let result = issue_cmd_runner.get_jira_issue(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn get_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<IssueBean, Box<dyn std::error::Error>> {
        Ok(get_issue(
            &self.cfg,
            params.issue_key.expect("Issue key is required!").as_str(),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?)
    }

    /// Transitions a Jira issue
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `Value` - Jira issue transition
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    ///
    /// let mut params = IssueCmdParams::new();
    /// params.transition = Some("transition_id".to_string());
    ///
    /// let result = issue_cmd_runner.transition_jira_issue(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn transition_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let transition = IssueTransition {
            id: Some(params.transition.expect("Transition is required!")),
            ..Default::default()
        };
        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: Some(transition),
            update: None,
        };
        Ok(do_transition(
            &self.cfg,
            params.issue_key.expect("Issue key is required").as_str(),
            issue_data,
        )
        .await?)
    }

    /// Updates a Jira issue
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `Value` - Jira issue update
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    /// let params = IssueCmdParams::new();
    ///
    /// let result = issue_cmd_runner.update_jira_issue(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn update_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: None,
            update: None,
        };
        Ok(edit_issue(
            &self.cfg,
            params.issue_key.expect("Issue key is required!").as_str(),
            issue_data,
            None,
            None,
            None,
            Some(true),
            None,
        )
        .await?)
    }

    /// Gets available transitions for a Jira issue
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `Transitions` - Jira issue transitions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueTransitionCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let issue_cmd_runner = IssueCmdRunner::new(cfg_file);
    /// let mut params = IssueTransitionCmdParams::new();
    /// params.issue_key = "issue_key".to_string();
    ///
    /// let result = issue_cmd_runner.get_issue_available_transitions(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn get_issue_available_transitions(
        &self,
        params: IssueTransitionCmdParams,
    ) -> Result<Transitions, Box<dyn std::error::Error>> {
        Ok(get_transitions(
            &self.cfg,
            &params.issue_key,
            None,
            None,
            None,
            Some(false),
            None,
        )
        .await?)
    }
}

/// Issue command parameters
///
/// # Fields
///
/// * `project_key` - Jira project key
/// * `issue_key` - Jira issue key
/// * `issue_fields` - Jira issue fields
/// * `transition` - Jira issue transition
/// * `assignee` - Jira issue assignee
pub struct IssueCmdParams {
    /// Jira project key
    pub project_key: String,
    /// Jira issue key
    pub issue_key: Option<String>,
    /// Jira issue fields
    pub issue_fields: Option<HashMap<String, Value>>,
    /// Jira issue transition
    pub transition: Option<String>,
    /// Jira issue assignee
    pub assignee: Option<String>,
}

/// Implementation of IssueCmdParams struct
///
/// # Methods
///
/// * `new` - Creates a new IssueCmdParams instance
impl IssueCmdParams {
    /// Creates a new IssueCmdParams instance
    ///
    /// # Returns
    ///
    /// * `IssueCmdParams` - Issue command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::IssueCmdParams;
    ///
    /// let params = IssueCmdParams::new();
    /// ```
    pub fn new() -> IssueCmdParams {
        IssueCmdParams {
            project_key: "".to_string(),
            issue_key: None,
            issue_fields: None,
            transition: None,
            assignee: None,
        }
    }
}

/// Implementation of From trait for IssueCmdParams struct
/// to convert IssueArgs struct to IssueCmdParams struct
impl From<&IssueArgs> for IssueCmdParams {
    /// Converts IssueArgs struct to IssueCmdParams struct
    /// to create a new IssueCmdParams instance
    ///
    /// # Arguments
    ///
    /// * `value` - IssueArgs struct
    ///
    /// # Returns
    ///
    /// * `IssueCmdParams` - Issue command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::IssueCmdParams;
    /// use jirust_cli::args::commands::{IssueArgs, PaginationArgs, OutputArgs, IssueActionValues};
    /// use std::collections::HashMap;
    /// use serde_json::Value;
    ///
    /// let issue_args = IssueArgs {
    ///    issue_act: IssueActionValues::Get,
    ///    project_key: "project_key".to_string(),
    ///    issue_key: Some("issue_key".to_string()),
    ///    issue_fields: Some(vec![("key".to_string(), r#"{ "key": "value" }"#.to_string())]),
    ///    transition_to: Some("transition_to".to_string()),
    ///    assignee: Some("assignee".to_string()),
    ///    pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///    output: OutputArgs { output: None },
    /// };
    ///
    /// let params = IssueCmdParams::from(&issue_args);
    ///
    /// assert_eq!(params.project_key, "project_key".to_string());
    /// assert_eq!(params.issue_key.unwrap(), "issue_key".to_string());
    /// assert_eq!(params.transition.unwrap(), "transition_to".to_string());
    /// assert_eq!(params.assignee.unwrap(), "assignee".to_string());
    /// ```
    fn from(value: &IssueArgs) -> Self {
        IssueCmdParams {
            project_key: value.project_key.clone(),
            issue_key: value.issue_key.clone(),
            issue_fields: Some(
                value
                    .issue_fields
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|elem| {
                        (
                            elem.0.clone(),
                            serde_json::from_str(elem.1.clone().as_str()).unwrap_or(Value::Null),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
            ),
            transition: value.transition_to.clone(),
            assignee: value.assignee.clone(),
        }
    }
}

/// Issue transition command parameters
///
/// # Fields
///
/// * `issue_key` - Jira issue key
pub struct IssueTransitionCmdParams {
    /// Jira issue key
    pub issue_key: String,
}

/// Implementation of IssueTransitionCmdParams struct
///
/// # Methods
///
/// * `new` - Creates a new IssueTransitionCmdParams instance
impl IssueTransitionCmdParams {
    /// Creates a new IssueTransitionCmdParams instance
    ///
    /// # Returns
    ///
    /// * `IssueTransitionCmdParams` - Issue transition command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::IssueTransitionCmdParams;
    ///
    /// let params = IssueTransitionCmdParams::new();
    /// ```
    pub fn new() -> IssueTransitionCmdParams {
        IssueTransitionCmdParams {
            issue_key: "".to_string(),
        }
    }
}

/// Implementation of From trait for IssueTransitionCmdParams struct
/// to convert TransitionArgs struct to IssueTransitionCmdParams struct
impl From<&TransitionArgs> for IssueTransitionCmdParams {
    /// Converts TransitionArgs struct to IssueTransitionCmdParams struct
    /// to create a new IssueTransitionCmdParams instance
    ///
    /// # Arguments
    ///
    /// * `value` - TransitionArgs struct
    ///
    /// # Returns
    ///
    /// * `IssueTransitionCmdParams` - Issue transition command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::IssueTransitionCmdParams;
    /// use jirust_cli::args::commands::{TransitionArgs, TransitionActionValues, OutputArgs};
    ///
    /// let transition_args = TransitionArgs {
    ///    transition_act: TransitionActionValues::List,
    ///    issue_key: "issue_key".to_string(),
    ///    output: OutputArgs { output: None },
    /// };
    ///
    /// let params = IssueTransitionCmdParams::from(&transition_args);
    ///
    /// assert_eq!(params.issue_key, "issue_key".to_string());
    /// ```
    fn from(value: &TransitionArgs) -> Self {
        IssueTransitionCmdParams {
            issue_key: value.issue_key.clone(),
        }
    }
}

/// Default implementation for IssueCmdParams struct
impl Default for IssueTransitionCmdParams {
    /// Creates a default IssueTransitionCmdParams instance
    ///
    /// # Returns
    ///
    /// A IssueTransitionCmdParams instance with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::IssueTransitionCmdParams;
    ///
    /// let params = IssueTransitionCmdParams::default();
    ///
    /// assert_eq!(params.issue_key, "".to_string());
    /// ```
    fn default() -> Self {
        IssueTransitionCmdParams::new()
    }
}

/// Default implementation for IssueCmdParams struct
impl Default for IssueCmdParams {
    /// Creates a default IssueCmdParams instance
    ///
    /// # Returns
    ///
    /// A IssueCmdParams instance with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::issue_cmd_runner::IssueCmdParams;
    ///
    /// let params = IssueCmdParams::default();
    ///
    /// assert_eq!(params.project_key, "".to_string());
    /// assert_eq!(params.issue_key, None);
    /// assert_eq!(params.issue_fields, None);
    /// assert_eq!(params.transition, None);
    /// assert_eq!(params.assignee, None);
    /// ```
    fn default() -> Self {
        IssueCmdParams::new()
    }
}
