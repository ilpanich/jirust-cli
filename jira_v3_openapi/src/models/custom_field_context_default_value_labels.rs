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

/// CustomFieldContextDefaultValueLabels : Default value for a labels custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueLabels {
    /// The default labels value.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueLabels {
    /// Default value for a labels custom field.
    pub fn new(labels: Vec<String>, r#type: String) -> CustomFieldContextDefaultValueLabels {
        CustomFieldContextDefaultValueLabels {
            labels,
            r#type,
        }
    }
}

