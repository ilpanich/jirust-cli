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

/// FunctionReferenceData : Details of functions that can be used in advanced searches.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionReferenceData {
    /// The display name of the function.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether the function can take a list of arguments.
    #[serde(rename = "isList", skip_serializing_if = "Option::is_none")]
    pub is_list: Option<IsList>,
    /// Whether the function supports both single and list value operators.
    #[serde(rename = "supportsListAndSingleValueOperators", skip_serializing_if = "Option::is_none")]
    pub supports_list_and_single_value_operators: Option<SupportsListAndSingleValueOperators>,
    /// The data types returned by the function.
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// The function identifier.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl FunctionReferenceData {
    /// Details of functions that can be used in advanced searches.
    pub fn new() -> FunctionReferenceData {
        FunctionReferenceData {
            display_name: None,
            is_list: None,
            supports_list_and_single_value_operators: None,
            types: None,
            value: None,
        }
    }
}
/// Whether the function can take a list of arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsList {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for IsList {
    fn default() -> IsList {
        Self::True
    }
}
/// Whether the function supports both single and list value operators.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportsListAndSingleValueOperators {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for SupportsListAndSingleValueOperators {
    fn default() -> SupportsListAndSingleValueOperators {
        Self::True
    }
}

