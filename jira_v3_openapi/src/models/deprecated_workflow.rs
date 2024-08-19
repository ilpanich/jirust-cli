/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DeprecatedWorkflow : Details about a workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeprecatedWorkflow {
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The description of the workflow.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The datetime the workflow was last modified.
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "lastModifiedUser", skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// The account ID of the user that last modified the workflow.
    #[serde(rename = "lastModifiedUserAccountId", skip_serializing_if = "Option::is_none")]
    pub last_modified_user_account_id: Option<String>,
    /// The name of the workflow.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The scope where this workflow applies
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// The number of steps included in the workflow.
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<i32>,
}

impl DeprecatedWorkflow {
    /// Details about a workflow.
    pub fn new() -> DeprecatedWorkflow {
        DeprecatedWorkflow {
            default: None,
            description: None,
            last_modified_date: None,
            last_modified_user: None,
            last_modified_user_account_id: None,
            name: None,
            scope: None,
            steps: None,
        }
    }
}

