/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowTransitionRule : A workflow transition rule.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionRule {
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(rename = "configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Option<serde_json::Value>>,
    /// The type of the transition rule.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl WorkflowTransitionRule {
    /// A workflow transition rule.
    pub fn new(r#type: String) -> WorkflowTransitionRule {
        WorkflowTransitionRule {
            configuration: None,
            r#type,
        }
    }
}

