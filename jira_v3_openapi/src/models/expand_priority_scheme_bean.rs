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

/// ExpandPrioritySchemeBean : A priority scheme with less fields to be used in for an API expand response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpandPrioritySchemeBean {
    /// The ID of the priority scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the priority scheme.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the priority scheme.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl ExpandPrioritySchemeBean {
    /// A priority scheme with less fields to be used in for an API expand response.
    pub fn new() -> ExpandPrioritySchemeBean {
        ExpandPrioritySchemeBean {
            id: None,
            name: None,
            param_self: None,
        }
    }
}

