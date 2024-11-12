/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AutoCompleteSuggestion : A field auto-complete suggestion.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoCompleteSuggestion {
    /// The display name of a suggested item. If `fieldValue` or `predicateValue` are provided, the matching text is highlighted with the HTML bold tag.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The value of a suggested item.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl AutoCompleteSuggestion {
    /// A field auto-complete suggestion.
    pub fn new() -> AutoCompleteSuggestion {
        AutoCompleteSuggestion {
            display_name: None,
            value: None,
        }
    }
}

