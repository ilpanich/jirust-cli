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

/// JqlReferenceData : Lists of JQL reference data.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlReferenceData {
    /// List of JQL query reserved words.
    #[serde(rename = "jqlReservedWords", skip_serializing_if = "Option::is_none")]
    pub jql_reserved_words: Option<Vec<String>>,
    /// List of fields usable in JQL queries.
    #[serde(rename = "visibleFieldNames", skip_serializing_if = "Option::is_none")]
    pub visible_field_names: Option<Vec<models::FieldReferenceData>>,
    /// List of functions usable in JQL queries.
    #[serde(rename = "visibleFunctionNames", skip_serializing_if = "Option::is_none")]
    pub visible_function_names: Option<Vec<models::FunctionReferenceData>>,
}

impl JqlReferenceData {
    /// Lists of JQL reference data.
    pub fn new() -> JqlReferenceData {
        JqlReferenceData {
            jql_reserved_words: None,
            visible_field_names: None,
            visible_function_names: None,
        }
    }
}

