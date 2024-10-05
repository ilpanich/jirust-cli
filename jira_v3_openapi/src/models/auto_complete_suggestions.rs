/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AutoCompleteSuggestions : The results from a JQL query.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoCompleteSuggestions {
    /// The list of suggested item.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::AutoCompleteSuggestion>>,
}

impl AutoCompleteSuggestions {
    /// The results from a JQL query.
    pub fn new() -> AutoCompleteSuggestions {
        AutoCompleteSuggestions {
            results: None,
        }
    }
}

