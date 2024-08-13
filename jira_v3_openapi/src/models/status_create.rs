/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// StatusCreate : Details of the status being created.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusCreate {
    /// The description of the status.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the status.
    #[serde(rename = "name")]
    pub name: String,
    /// The category of the status.
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCategory,
}

impl StatusCreate {
    /// Details of the status being created.
    pub fn new(name: String, status_category: StatusCategory) -> StatusCreate {
        StatusCreate {
            description: None,
            name,
            status_category,
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
