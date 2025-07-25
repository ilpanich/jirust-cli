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

/// JqlFunctionPrecomputationBean : Jql function precomputation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationBean {
    /// The list of arguments function was invoked with.
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    /// The timestamp of the precomputation creation.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The error message to be displayed to the user.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The field the function was executed against.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The function key.
    #[serde(rename = "functionKey", skip_serializing_if = "Option::is_none")]
    pub function_key: Option<String>,
    /// The name of the function.
    #[serde(rename = "functionName", skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// The id of the precomputation.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The operator in context of which function was executed.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// The timestamp of the precomputation last update.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// The timestamp of the precomputation last usage.
    #[serde(rename = "used", skip_serializing_if = "Option::is_none")]
    pub used: Option<String>,
    /// The JQL fragment stored as the precomputation.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl JqlFunctionPrecomputationBean {
    /// Jql function precomputation.
    pub fn new() -> JqlFunctionPrecomputationBean {
        JqlFunctionPrecomputationBean {
            arguments: None,
            created: None,
            error: None,
            field: None,
            function_key: None,
            function_name: None,
            id: None,
            operator: None,
            updated: None,
            used: None,
            value: None,
        }
    }
}

