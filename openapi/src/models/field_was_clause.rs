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

/// FieldWasClause : A clause that asserts a previous value of a field. For example, `status WAS \"Resolved\" BY currentUser() BEFORE \"2019/02/02\"`. See [WAS](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-WASWAS) for more information about the WAS operator.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldWasClause {
    #[serde(rename = "field")]
    pub field: Box<models::JqlQueryField>,
    #[serde(rename = "operand")]
    pub operand: Box<models::JqlQueryClauseOperand>,
    /// The operator between the field and operand.
    #[serde(rename = "operator")]
    pub operator: Operator,
    /// The list of time predicates.
    #[serde(rename = "predicates")]
    pub predicates: Vec<models::JqlQueryClauseTimePredicate>,
}

impl FieldWasClause {
    /// A clause that asserts a previous value of a field. For example, `status WAS \"Resolved\" BY currentUser() BEFORE \"2019/02/02\"`. See [WAS](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-WASWAS) for more information about the WAS operator.
    pub fn new(field: models::JqlQueryField, operand: models::JqlQueryClauseOperand, operator: Operator, predicates: Vec<models::JqlQueryClauseTimePredicate>) -> FieldWasClause {
        FieldWasClause {
            field: Box::new(field),
            operand: Box::new(operand),
            operator,
            predicates,
        }
    }
}
/// The operator between the field and operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "was")]
    Was,
    #[serde(rename = "was in")]
    WasIn,
    #[serde(rename = "was not in")]
    WasNotIn,
    #[serde(rename = "was not")]
    WasNot,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Was
    }
}

