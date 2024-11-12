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

/// WorkflowRulesSearchDetails : Details of workflow transition rules.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRulesSearchDetails {
    /// List of workflow rule IDs that do not belong to the workflow or can not be found.
    #[serde(rename = "invalidRules", skip_serializing_if = "Option::is_none")]
    pub invalid_rules: Option<Vec<uuid::Uuid>>,
    /// List of valid workflow transition rules.
    #[serde(rename = "validRules", skip_serializing_if = "Option::is_none")]
    pub valid_rules: Option<Vec<models::WorkflowTransitionRules>>,
    /// The workflow ID.
    #[serde(rename = "workflowEntityId", skip_serializing_if = "Option::is_none")]
    pub workflow_entity_id: Option<uuid::Uuid>,
}

impl WorkflowRulesSearchDetails {
    /// Details of workflow transition rules.
    pub fn new() -> WorkflowRulesSearchDetails {
        WorkflowRulesSearchDetails {
            invalid_rules: None,
            valid_rules: None,
            workflow_entity_id: None,
        }
    }
}

