/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BulkProjectPermissions : Details of project permissions and associated issues and projects to look up.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkProjectPermissions {
    /// List of issue IDs.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<i64>>,
    /// List of project permissions.
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    /// List of project IDs.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<i64>>,
}

impl BulkProjectPermissions {
    /// Details of project permissions and associated issues and projects to look up.
    pub fn new(permissions: Vec<String>) -> BulkProjectPermissions {
        BulkProjectPermissions {
            issues: None,
            permissions,
            projects: None,
        }
    }
}

