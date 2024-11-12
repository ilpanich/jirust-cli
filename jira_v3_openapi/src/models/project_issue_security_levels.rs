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

/// ProjectIssueSecurityLevels : List of issue level security items in a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIssueSecurityLevels {
    /// Issue level security items list.
    #[serde(rename = "levels")]
    pub levels: Vec<models::SecurityLevel>,
}

impl ProjectIssueSecurityLevels {
    /// List of issue level security items in a project.
    pub fn new(levels: Vec<models::SecurityLevel>) -> ProjectIssueSecurityLevels {
        ProjectIssueSecurityLevels {
            levels,
        }
    }
}

