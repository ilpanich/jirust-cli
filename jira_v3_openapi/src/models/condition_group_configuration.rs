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

/// ConditionGroupConfiguration : The conditions group associated with the transition.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConditionGroupConfiguration {
    /// The nested conditions of the condition group.
    #[serde(rename = "conditionGroups", skip_serializing_if = "Option::is_none")]
    pub condition_groups: Option<Vec<models::ConditionGroupConfiguration>>,
    /// The rules for this condition.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::WorkflowRuleConfiguration>>,
    /// Determines how the conditions in the group are evaluated. Accepts either `ANY` or `ALL`. If `ANY` is used, at least one condition in the group must be true for the group to evaluate to true. If `ALL` is used, all conditions in the group must be true for the group to evaluate to true.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

impl ConditionGroupConfiguration {
    /// The conditions group associated with the transition.
    pub fn new() -> ConditionGroupConfiguration {
        ConditionGroupConfiguration {
            condition_groups: None,
            conditions: None,
            operation: None,
        }
    }
}
/// Determines how the conditions in the group are evaluated. Accepts either `ANY` or `ALL`. If `ANY` is used, at least one condition in the group must be true for the group to evaluate to true. If `ALL` is used, all conditions in the group must be true for the group to evaluate to true.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "ALL")]
    All,
}

impl Default for Operation {
    fn default() -> Operation {
        Self::Any
    }
}

