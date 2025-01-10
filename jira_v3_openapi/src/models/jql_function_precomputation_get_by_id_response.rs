/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JqlFunctionPrecomputationGetByIdResponse : Get precomputations by ID response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationGetByIdResponse {
    /// List of precomputations that were not found.
    #[serde(rename = "notFoundPrecomputationIDs", skip_serializing_if = "Option::is_none")]
    pub not_found_precomputation_ids: Option<Vec<String>>,
    /// The list of precomputations.
    #[serde(rename = "precomputations", skip_serializing_if = "Option::is_none")]
    pub precomputations: Option<Vec<models::JqlFunctionPrecomputationBean>>,
}

impl JqlFunctionPrecomputationGetByIdResponse {
    /// Get precomputations by ID response.
    pub fn new() -> JqlFunctionPrecomputationGetByIdResponse {
        JqlFunctionPrecomputationGetByIdResponse {
            not_found_precomputation_ids: None,
            precomputations: None,
        }
    }
}

