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

/// CustomFieldContextDefaultValueSingleOption : The default value for a single select custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueSingleOption {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default option.
    #[serde(rename = "optionId")]
    pub option_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueSingleOption {
    /// The default value for a single select custom field.
    pub fn new(context_id: String, option_id: String, r#type: String) -> CustomFieldContextDefaultValueSingleOption {
        CustomFieldContextDefaultValueSingleOption {
            context_id,
            option_id,
            r#type,
        }
    }
}

