/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowsWithTransitionRulesDetails : Details of workflows and their transition rules to delete.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsWithTransitionRulesDetails {
    /// The list of workflows with transition rules to delete.
    #[serde(rename = "workflows")]
    pub workflows: Vec<models::WorkflowTransitionRulesDetails>,
}

impl WorkflowsWithTransitionRulesDetails {
    /// Details of workflows and their transition rules to delete.
    pub fn new(workflows: Vec<models::WorkflowTransitionRulesDetails>) -> WorkflowsWithTransitionRulesDetails {
        WorkflowsWithTransitionRulesDetails {
            workflows,
        }
    }
}

