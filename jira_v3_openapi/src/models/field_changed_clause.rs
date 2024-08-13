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

/// FieldChangedClause : A clause that asserts whether a field was changed. For example, `status CHANGED AFTER startOfMonth(-1M)`.See [CHANGED](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-CHANGEDCHANGED) for more information about the CHANGED operator.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldChangedClause {
    #[serde(rename = "field")]
    pub field: Box<models::JqlQueryField>,
    /// The operator applied to the field.
    #[serde(rename = "operator")]
    pub operator: Operator,
    /// The list of time predicates.
    #[serde(rename = "predicates")]
    pub predicates: Vec<models::JqlQueryClauseTimePredicate>,
}

impl FieldChangedClause {
    /// A clause that asserts whether a field was changed. For example, `status CHANGED AFTER startOfMonth(-1M)`.See [CHANGED](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-CHANGEDCHANGED) for more information about the CHANGED operator.
    pub fn new(field: models::JqlQueryField, operator: Operator, predicates: Vec<models::JqlQueryClauseTimePredicate>) -> FieldChangedClause {
        FieldChangedClause {
            field: Box::new(field),
            operator,
            predicates,
        }
    }
}
/// The operator applied to the field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "changed")]
    Changed,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Changed
    }
}
