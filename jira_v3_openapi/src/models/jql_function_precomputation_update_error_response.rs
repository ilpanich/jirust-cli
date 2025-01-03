/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JqlFunctionPrecomputationUpdateErrorResponse : Error response returned updating JQL Function precomputations fails.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationUpdateErrorResponse {
    /// The list of error messages produced by this operation.
    #[serde(rename = "errorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    /// List of precomputations that were not found.
    #[serde(rename = "notFoundPrecomputationIDs", skip_serializing_if = "Option::is_none")]
    pub not_found_precomputation_ids: Option<Vec<String>>,
}

impl JqlFunctionPrecomputationUpdateErrorResponse {
    /// Error response returned updating JQL Function precomputations fails.
    pub fn new() -> JqlFunctionPrecomputationUpdateErrorResponse {
        JqlFunctionPrecomputationUpdateErrorResponse {
            error_messages: None,
            not_found_precomputation_ids: None,
        }
    }
}
