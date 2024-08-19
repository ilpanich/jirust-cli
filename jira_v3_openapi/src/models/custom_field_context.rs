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

/// CustomFieldContext : The details of a custom field context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContext {
    /// The description of the context.
    #[serde(rename = "description")]
    pub description: String,
    /// The ID of the context.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the context apply to all issue types.
    #[serde(rename = "isAnyIssueType")]
    pub is_any_issue_type: bool,
    /// Whether the context is global.
    #[serde(rename = "isGlobalContext")]
    pub is_global_context: bool,
    /// The name of the context.
    #[serde(rename = "name")]
    pub name: String,
}

impl CustomFieldContext {
    /// The details of a custom field context.
    pub fn new(description: String, id: String, is_any_issue_type: bool, is_global_context: bool, name: String) -> CustomFieldContext {
        CustomFieldContext {
            description,
            id,
            is_any_issue_type,
            is_global_context,
            name,
        }
    }
}

