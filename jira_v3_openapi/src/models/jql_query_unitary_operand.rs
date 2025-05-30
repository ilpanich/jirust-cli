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

/// JqlQueryUnitaryOperand : An operand that can be part of a list operand.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryUnitaryOperand {
    /// Encoded value, which can be used directly in a JQL query.
    #[serde(rename = "encodedValue", skip_serializing_if = "Option::is_none")]
    pub encoded_value: Option<String>,
    /// The operand value.
    #[serde(rename = "value")]
    pub value: String,
    /// The list of function arguments.
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,
    /// Encoded operand, which can be used directly in a JQL query.
    #[serde(rename = "encodedOperand", skip_serializing_if = "Option::is_none")]
    pub encoded_operand: Option<String>,
    /// The name of the function.
    #[serde(rename = "function")]
    pub function: String,
    /// The keyword that is the operand value.
    #[serde(rename = "keyword")]
    pub keyword: Keyword,
}

impl JqlQueryUnitaryOperand {
    /// An operand that can be part of a list operand.
    pub fn new(value: String, arguments: Vec<String>, function: String, keyword: Keyword) -> JqlQueryUnitaryOperand {
        JqlQueryUnitaryOperand {
            encoded_value: None,
            value,
            arguments,
            encoded_operand: None,
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

