/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowCompoundCondition : A compound workflow transition rule condition. This object returns `nodeType` as `compound`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCompoundCondition {
    /// The list of workflow conditions.
    #[serde(rename = "conditions")]
    pub conditions: Vec<models::WorkflowCondition>,
    #[serde(rename = "nodeType")]
    pub node_type: String,
    /// The compound condition operator.
    #[serde(rename = "operator")]
    pub operator: Operator,
}

impl WorkflowCompoundCondition {
    /// A compound workflow transition rule condition. This object returns `nodeType` as `compound`.
    pub fn new(conditions: Vec<models::WorkflowCondition>, node_type: String, operator: Operator) -> WorkflowCompoundCondition {
        WorkflowCompoundCondition {
            conditions,
            node_type,
            operator,
        }
    }
}
/// The compound condition operator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::And
    }
}

