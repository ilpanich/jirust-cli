/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// StatusUpdate : Details of the status being updated.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusUpdate {
    /// The description of the status.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the status.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the status.
    #[serde(rename = "name")]
    pub name: String,
    /// The category of the status.
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCategory,
}

impl StatusUpdate {
    /// Details of the status being updated.
    pub fn new(id: String, name: String, status_category: StatusCategory) -> StatusUpdate {
        StatusUpdate {
            description: None,
            id,
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

