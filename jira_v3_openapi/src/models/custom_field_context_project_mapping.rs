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

/// CustomFieldContextProjectMapping : Details of a context to project association.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextProjectMapping {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// Whether context is global.
    #[serde(rename = "isGlobalContext", skip_serializing_if = "Option::is_none")]
    pub is_global_context: Option<bool>,
    /// The ID of the project.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl CustomFieldContextProjectMapping {
    /// Details of a context to project association.
    pub fn new(context_id: String) -> CustomFieldContextProjectMapping {
        CustomFieldContextProjectMapping {
            context_id,
            is_global_context: None,
            project_id: None,
        }
    }
}
