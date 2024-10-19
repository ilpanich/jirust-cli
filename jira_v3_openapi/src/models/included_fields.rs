/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncludedFields {
    #[serde(rename = "actuallyIncluded", skip_serializing_if = "Option::is_none")]
    pub actually_included: Option<Vec<String>>,
    #[serde(rename = "excluded", skip_serializing_if = "Option::is_none")]
    pub excluded: Option<Vec<String>>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<String>>,
}

impl IncludedFields {
    pub fn new() -> IncludedFields {
        IncludedFields {
            actually_included: None,
            excluded: None,
            included: None,
        }
    }
}

