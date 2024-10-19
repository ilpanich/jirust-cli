/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomFieldContextDefaultValueProject : The default value for a project custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueProject {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default project.
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueProject {
    /// The default value for a project custom field.
    pub fn new(context_id: String, project_id: String, r#type: String) -> CustomFieldContextDefaultValueProject {
        CustomFieldContextDefaultValueProject {
            context_id,
            project_id,
            r#type,
        }
    }
}

