/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueEntityPropertiesForMultiUpdate : An issue ID with entity property values. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueEntityPropertiesForMultiUpdate {
    /// The ID of the issue.
    #[serde(rename = "issueID", skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<i64>,
    /// Entity properties to set on the issue. The maximum length of an issue property value is 32768 characters.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, models::JsonNode>>,
}

impl IssueEntityPropertiesForMultiUpdate {
    /// An issue ID with entity property values. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
    pub fn new() -> IssueEntityPropertiesForMultiUpdate {
        IssueEntityPropertiesForMultiUpdate {
            issue_id: None,
            properties: None,
        }
    }
}

