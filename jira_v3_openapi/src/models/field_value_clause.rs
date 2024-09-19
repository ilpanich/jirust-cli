/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FieldValueClause : A clause that asserts the current value of a field. For example, `summary ~ test`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldValueClause {
    #[serde(rename = "field")]
    pub field: Box<models::JqlQueryField>,
    #[serde(rename = "operand")]
    pub operand: Box<models::JqlQueryClauseOperand>,
    /// The operator between the field and operand.
    #[serde(rename = "operator")]
    pub operator: Operator,
}

impl FieldValueClause {
    /// A clause that asserts the current value of a field. For example, `summary ~ test`.
    pub fn new(field: models::JqlQueryField, operand: models::JqlQueryClauseOperand, operator: Operator) -> FieldValueClause {
        FieldValueClause {
            field: Box::new(field),
            operand: Box::new(operand),
            operator,
        }
    }
}
/// The operator between the field and operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "!=")]
    ExclamationEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = ">=")]
    GreaterThanEqual,
    #[serde(rename = "<=")]
    LessThanEqual,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "not in")]
    NotIn,
    #[serde(rename = "~")]
    Tilde,
    #[serde(rename = "~=")]
    TildeEqual,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "is not")]
    IsNot,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Equal
    }
}

