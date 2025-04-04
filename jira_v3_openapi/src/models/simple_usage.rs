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

/// SimpleUsage : Represents a usage of an entity by a project ID and related issue type IDs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleUsage {
    /// The issue type IDs for the usage.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    /// The project ID for the usage.
    #[serde(rename = "projectId")]
    pub project_id: String,
}

impl SimpleUsage {
    /// Represents a usage of an entity by a project ID and related issue type IDs.
    pub fn new(issue_type_ids: Vec<String>, project_id: String) -> SimpleUsage {
        SimpleUsage {
            issue_type_ids,
            project_id,
        }
    }
}

