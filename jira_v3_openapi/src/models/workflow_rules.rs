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

/// WorkflowRules : A collection of transition rules.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRules {
    #[serde(rename = "conditionsTree", skip_serializing_if = "Option::is_none")]
    pub conditions_tree: Option<Box<models::WorkflowCondition>>,
    /// The workflow post functions.
    #[serde(rename = "postFunctions", skip_serializing_if = "Option::is_none")]
    pub post_functions: Option<Vec<models::WorkflowTransitionRule>>,
    /// The workflow validators.
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<models::WorkflowTransitionRule>>,
}

impl WorkflowRules {
    /// A collection of transition rules.
    pub fn new() -> WorkflowRules {
        WorkflowRules {
            conditions_tree: None,
            post_functions: None,
            validators: None,
        }
    }
}
