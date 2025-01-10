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

/// JiraStatus : Details of a status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraStatus {
    /// The description of the status.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the status.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the status.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<models::StatusScope>>,
    /// The category of the status.
    #[serde(rename = "statusCategory", skip_serializing_if = "Option::is_none")]
    pub status_category: Option<StatusCategory>,
    /// Projects and issue types where the status is used. Only available if the `usages` expand is requested.
    #[serde(rename = "usages", skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<models::ProjectIssueTypes>>,
    /// The workflows that use this status. Only available if the `workflowUsages` expand is requested.
    #[serde(rename = "workflowUsages", skip_serializing_if = "Option::is_none")]
    pub workflow_usages: Option<Vec<models::WorkflowUsages>>,
}

impl JiraStatus {
    /// Details of a status.
    pub fn new() -> JiraStatus {
        JiraStatus {
            description: None,
            id: None,
            name: None,
            scope: None,
            status_category: None,
            usages: None,
            workflow_usages: None,
        }
    }
}
/// The category of the status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCategory {
    #[serde(rename = "TODO")]
    Todo,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "DONE")]
    Done,
}

impl Default for StatusCategory {
    fn default() -> StatusCategory {
        Self::Todo
    }
}

