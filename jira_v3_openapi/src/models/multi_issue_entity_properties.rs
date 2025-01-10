/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MultiIssueEntityProperties : A list of issues and their respective properties to set or update. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiIssueEntityProperties {
    /// A list of issue IDs and their respective properties.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<models::IssueEntityPropertiesForMultiUpdate>>,
}

impl MultiIssueEntityProperties {
    /// A list of issues and their respective properties to set or update. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
    pub fn new() -> MultiIssueEntityProperties {
        MultiIssueEntityProperties {
            issues: None,
        }
    }
}

