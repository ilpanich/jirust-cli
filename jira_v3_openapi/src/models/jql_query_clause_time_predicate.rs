/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JqlQueryClauseTimePredicate : A time predicate for a temporal JQL clause.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryClauseTimePredicate {
    #[serde(rename = "operand")]
    pub operand: Box<models::JqlQueryClauseOperand>,
    /// The operator between the field and the operand.
    #[serde(rename = "operator")]
    pub operator: Operator,
}

impl JqlQueryClauseTimePredicate {
    /// A time predicate for a temporal JQL clause.
    pub fn new(operand: models::JqlQueryClauseOperand, operator: Operator) -> JqlQueryClauseTimePredicate {
        JqlQueryClauseTimePredicate {
            operand: Box::new(operand),
            operator,
        }
    }
}
/// The operator between the field and the operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "after")]
    After,
    #[serde(rename = "from")]
    From,
    #[serde(rename = "to")]
    To,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "during")]
    During,
    #[serde(rename = "by")]
    By,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Before
    }
}

