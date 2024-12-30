use jira_v3_openapi::apis::issue_links_api::link_issues;
use jira_v3_openapi::models::{
    IssueLinkType, LinkIssueRequestJsonBean, LinkedIssue,
};
use jira_v3_openapi::apis::configuration::Configuration;
use serde_json::Value;

use crate::args::commands::LinkIssueArgs;
use crate::config::config_file::{AuthData, ConfigFile};

/// Issue command runner
/// This struct is responsible for running the issue command
/// It uses the Jira API to perform the operations
///
/// # Fields
///
/// * `cfg` - Configuration object
pub struct LinkIssueCmdRunner {
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
impl LinkIssueCmdRunner {
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
    pub fn new(cfg_file: ConfigFile) -> LinkIssueCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        LinkIssueCmdRunner { cfg: config }
    }

    pub async fn link_jira_issues(&self, params: LinkIssueCmdParams) -> Result<Value, String> {
        let mut link_requests: Vec<LinkIssueRequestJsonBean> = Vec::new();
        if params.destination_issue_key.is_some() {
            let link_request = LinkIssueRequestJsonBean {
                comment: None,
                inward_issue: Box::new(LinkedIssue {
                    key: Some(params.origin_issue_key),
                    id: None,
                    fields: None,
                    param_self: None,
                }),
                outward_issue: Box::new(LinkedIssue {
                    key: params.destination_issue_key,
                    id: None,
                    fields: None,
                    param_self: None,
                }),
                r#type: Box::new(IssueLinkType {
                    name: Some(params.link_type),
                    inward: None,
                    outward: None,
                    id: None,
                    param_self: None,
                }),
            };
            link_requests.push(link_request);
        } else {
            todo!("Implement Issues extraction from changelog")
        };

        for link_issue_request_json_bean in link_requests {
            link_issues(&self.cfg, link_issue_request_json_bean)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(Value::Null)
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
pub struct LinkIssueCmdParams {
    /// Jira project key
    pub origin_issue_key: String,
    /// Jira issue key
    pub destination_issue_key: Option<String>,
    /// Jira issue fields
    pub link_type: String,
    /// Jira issue transition
    pub changelog_file: Option<String>,
}

/// Implementation of IssueCmdParams struct
///
/// # Methods
///
/// * `new` - Creates a new IssueCmdParams instance
impl LinkIssueCmdParams {
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
    pub fn new() -> LinkIssueCmdParams {
        LinkIssueCmdParams {
            origin_issue_key: "".to_string(),
            destination_issue_key: None,
            link_type: "".to_string(),
            changelog_file: None,
        }
    }
}

/// Implementation of From trait for IssueCmdParams struct
/// to convert IssueArgs struct to IssueCmdParams struct
impl From<&LinkIssueArgs> for LinkIssueCmdParams {
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
    fn from(value: &LinkIssueArgs) -> Self {
        if (value.destination_issue_key.is_none() && value.changelog_file.is_none())
            || (value.destination_issue_key.is_some() && value.changelog_file.is_some())
        {
            panic!("Either destination issue key or changelog file is required");
        }
        LinkIssueCmdParams {
            origin_issue_key: value.origin_issue_key.clone(),
            destination_issue_key: value.destination_issue_key.clone(),
            link_type: value.link_type.clone(),
            changelog_file: value.changelog_file.clone(),
        }
    }
}

/// Default implementation for IssueCmdParams struct
impl Default for LinkIssueCmdParams {
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
        LinkIssueCmdParams::new()
    }
}
