use async_trait::async_trait;
use jira_v3_openapi::apis::issue_search_api::search_and_reconsile_issues_using_jql_post;
use jira_v3_openapi::apis::issues_api::*;
use jira_v3_openapi::models::user::AccountType;
use jira_v3_openapi::models::{
    CreatedIssue, IssueBean, IssueTransition, SearchAndReconcileRequestBean, Transitions, User,
};
use jira_v3_openapi::{apis::configuration::Configuration, models::IssueUpdateDetails};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Error;

#[cfg(test)]
use mockall::automock;

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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
    /// ```
    pub fn new(cfg_file: &ConfigFile) -> IssueCmdRunner {
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
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

        let i_key = if let Some(key) = &params.issue_key {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error assigning issue: Empty issue key".to_string(),
            )));
        };

        Ok(assign_issue(&self.cfg, i_key, user_data).await?)
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
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
        let mut issue_fields = params.issue_fields.unwrap_or_default();
        issue_fields.insert(
            "project".to_string(),
            serde_json::json!({"key": params.project_key.expect("Project Key is required to create an issue!")}),
        );
        let issue_data = IssueUpdateDetails {
            fields: Some(issue_fields),
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
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
        let i_key = if let Some(key) = &params.issue_key {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error deleting issue: Empty issue key".to_string(),
            )));
        };

        Ok(delete_issue(&self.cfg, i_key, Some("true")).await?)
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
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
        let i_key = if let Some(key) = &params.issue_key {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error retrieving issue: Empty issue key".to_string(),
            )));
        };
        Ok(get_issue(&self.cfg, i_key, None, None, None, None, None, None).await?)
    }

    /// This method searches for Jira issues using the provided JQL query parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Issue command parameters
    ///
    /// # Returns
    ///
    /// * `Vec<IssueBean>` - A vector of Jira issue beans
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
    /// let mut params = IssueCmdParams::new();
    /// params.query = Some("field=value".to_string());
    ///
    /// let result = issue_cmd_runner.search_jira_issues(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn search_jira_issues(
        &self,
        params: IssueCmdParams,
    ) -> Result<Vec<IssueBean>, Box<dyn std::error::Error>> {
        let search_params: SearchAndReconcileRequestBean = SearchAndReconcileRequestBean {
            fields: Some(vec!["*navigable".to_string(), "-comment".to_string()]),
            jql: params.query,
            ..Default::default()
        };
        match search_and_reconsile_issues_using_jql_post(&self.cfg, search_params).await {
            Ok(result) => {
                if let Some(issues) = result.issues {
                    Ok(issues)
                } else {
                    Ok(vec![])
                }
            }
            Err(e) => Err(Box::new(e)),
        }
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
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
        let i_key = if let Some(key) = &params.issue_key {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error with issue transition: Empty issue key".to_string(),
            )));
        };

        let trans = if let Some(transition) = &params.transition {
            transition.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error with issue transition: Empty transition".to_string(),
            )));
        };

        let transition = IssueTransition {
            id: Some(trans.to_string()),
            ..Default::default()
        };
        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: Some(transition),
            update: None,
        };
        Ok(do_transition(&self.cfg, i_key, issue_data).await?)
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
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
        let i_key = if let Some(key) = &params.issue_key {
            key.as_str()
        } else {
            return Err(Box::new(Error::other(
                "Error updating issue: Empty issue key".to_string(),
            )));
        };

        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: None,
            update: None,
        };
        Ok(edit_issue(
            &self.cfg,
            i_key,
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
    /// let issue_cmd_runner = IssueCmdRunner::new(&cfg_file);
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
/// * `query` - Jira issue query
pub struct IssueCmdParams {
    /// Jira project key
    pub project_key: Option<String>,
    /// Jira issue key
    pub issue_key: Option<String>,
    /// Jira issue fields
    pub issue_fields: Option<HashMap<String, Value>>,
    /// Jira issue transition
    pub transition: Option<String>,
    /// Jira issue assignee
    pub assignee: Option<String>,
    /// Jira issue query
    pub query: Option<String>,
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
            project_key: Some("".to_string()),
            issue_key: None,
            issue_fields: None,
            transition: None,
            assignee: None,
            query: None,
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
    ///    project_key: Some("project_key".to_string()),
    ///    issue_key: Some("issue_key".to_string()),
    ///    issue_fields: Some(vec![("key".to_string(), r#"{ "key": "value" }"#.to_string())]),
    ///    transition_to: Some("transition_to".to_string()),
    ///    assignee: Some("assignee".to_string()),
    ///    query: None,
    ///    pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///    output: OutputArgs { output_format: None, output_type: None },
    /// };
    ///
    /// let params = IssueCmdParams::from(&issue_args);
    ///
    /// assert_eq!(params.project_key, Some("project_key".to_string()));
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
            query: value.query.clone(),
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
    ///    output: OutputArgs { output_format: None, output_type: None },
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
    /// assert_eq!(params.project_key, Some("".to_string()));
    /// assert_eq!(params.issue_key, None);
    /// assert_eq!(params.issue_fields, None);
    /// assert_eq!(params.transition, None);
    /// assert_eq!(params.assignee, None);
    /// ```
    fn default() -> Self {
        IssueCmdParams::new()
    }
}

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait IssueCmdRunnerApi: Send + Sync {
    async fn assign_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>>;

    async fn create_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<CreatedIssue, Box<dyn std::error::Error>>;

    async fn delete_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn get_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<IssueBean, Box<dyn std::error::Error>>;

    async fn search_jira_issues(
        &self,
        params: IssueCmdParams,
    ) -> Result<Vec<IssueBean>, Box<dyn std::error::Error>>;

    async fn transition_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>>;

    async fn update_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>>;

    async fn get_issue_available_transitions(
        &self,
        params: IssueTransitionCmdParams,
    ) -> Result<Transitions, Box<dyn std::error::Error>>;
}

#[async_trait(?Send)]
impl IssueCmdRunnerApi for IssueCmdRunner {
    async fn assign_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        IssueCmdRunner::assign_jira_issue(self, params).await
    }

    async fn create_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<CreatedIssue, Box<dyn std::error::Error>> {
        IssueCmdRunner::create_jira_issue(self, params).await
    }

    async fn delete_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        IssueCmdRunner::delete_jira_issue(self, params).await
    }

    async fn get_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<IssueBean, Box<dyn std::error::Error>> {
        IssueCmdRunner::get_jira_issue(self, params).await
    }

    async fn search_jira_issues(
        &self,
        params: IssueCmdParams,
    ) -> Result<Vec<IssueBean>, Box<dyn std::error::Error>> {
        IssueCmdRunner::search_jira_issues(self, params).await
    }

    async fn transition_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        IssueCmdRunner::transition_jira_issue(self, params).await
    }

    async fn update_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        IssueCmdRunner::update_jira_issue(self, params).await
    }

    async fn get_issue_available_transitions(
        &self,
        params: IssueTransitionCmdParams,
    ) -> Result<Transitions, Box<dyn std::error::Error>> {
        IssueCmdRunner::get_issue_available_transitions(self, params).await
    }
}
