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

/// WorkflowSimpleCondition : A workflow transition rule condition. This object returns `nodeType` as `simple`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSimpleCondition {
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(rename = "nodeType")]
    pub node_type: String,
    /// The type of the transition rule.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl WorkflowSimpleCondition {
    /// A workflow transition rule condition. This object returns `nodeType` as `simple`.
    pub fn new(node_type: String, r#type: String) -> WorkflowSimpleCondition {
        WorkflowSimpleCondition {
            configuration: None,
            node_type,
            r#type,
        }
    }
}

