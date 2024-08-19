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

/// ProjectWithDataPolicy : Details about data policies for a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectWithDataPolicy {
    /// Data policy.
    #[serde(rename = "dataPolicy", skip_serializing_if = "Option::is_none")]
    pub data_policy: Option<Box<models::ProjectDataPolicy>>,
    /// The project ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl ProjectWithDataPolicy {
    /// Details about data policies for a project.
    pub fn new() -> ProjectWithDataPolicy {
        ProjectWithDataPolicy {
            data_policy: None,
            id: None,
        }
    }
}

