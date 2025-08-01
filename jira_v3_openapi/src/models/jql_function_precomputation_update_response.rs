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

/// JqlFunctionPrecomputationUpdateResponse : Result of updating JQL Function precomputations.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationUpdateResponse {
    /// List of precomputations that were not found and skipped. Only returned if the request passed skipNotFoundPrecomputations=true.
    #[serde(rename = "notFoundPrecomputationIDs", skip_serializing_if = "Option::is_none")]
    pub not_found_precomputation_ids: Option<Vec<String>>,
}

impl JqlFunctionPrecomputationUpdateResponse {
    /// Result of updating JQL Function precomputations.
    pub fn new() -> JqlFunctionPrecomputationUpdateResponse {
        JqlFunctionPrecomputationUpdateResponse {
            not_found_precomputation_ids: None,
        }
    }
}

