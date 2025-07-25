/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JqlQueryClauseOperand : Details of an operand in a JQL clause.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryClauseOperand {
    /// Encoded operand, which can be used directly in a JQL query.
    #[serde(rename = "encodedOperand", skip_serializing_if = "Option::is_none")]
    pub encoded_operand: Option<String>,
    /// The list of operand values.
    #[serde(rename = "values")]
    pub values: Vec<models::JqlQueryUnitaryOperand>,
    /// Encoded value, which can be used directly in a JQL query.
    #[serde(rename = "encodedValue", skip_serializing_if = "Option::is_none")]
    pub encoded_value: Option<String>,
    /// The operand value.
    #[serde(rename = "value")]
    pub value: String,
    /// The list of function arguments.
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,
    /// The name of the function.
    #[serde(rename = "function")]
    pub function: String,
    /// The keyword that is the operand value.
    #[serde(rename = "keyword")]
    pub keyword: Keyword,
}

impl JqlQueryClauseOperand {
    /// Details of an operand in a JQL clause.
    pub fn new(values: Vec<models::JqlQueryUnitaryOperand>, value: String, arguments: Vec<String>, function: String, keyword: Keyword) -> JqlQueryClauseOperand {
        JqlQueryClauseOperand {
            encoded_operand: None,
            values,
            encoded_value: None,
            value,
            arguments,
            function,
            keyword,
        }
    }
}
/// The keyword that is the operand value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Keyword {
    #[serde(rename = "empty")]
    Empty,
}

impl Default for Keyword {
    fn default() -> Keyword {
        Self::Empty
    }
}

