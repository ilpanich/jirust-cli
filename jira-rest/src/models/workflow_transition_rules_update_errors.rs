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

/// WorkflowTransitionRulesUpdateErrors : Details of any errors encountered while updating workflow transition rules.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionRulesUpdateErrors {
    /// A list of workflows.
    #[serde(rename = "updateResults")]
    pub update_results: Vec<models::WorkflowTransitionRulesUpdateErrorDetails>,
}

impl WorkflowTransitionRulesUpdateErrors {
    /// Details of any errors encountered while updating workflow transition rules.
    pub fn new(update_results: Vec<models::WorkflowTransitionRulesUpdateErrorDetails>) -> WorkflowTransitionRulesUpdateErrors {
        WorkflowTransitionRulesUpdateErrors {
            update_results,
        }
    }
}

