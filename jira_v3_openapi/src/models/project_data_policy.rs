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

/// ProjectDataPolicy : Details about data policy.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDataPolicy {
    /// Whether the project contains any content inaccessible to the requesting application.
    #[serde(rename = "anyContentBlocked", skip_serializing_if = "Option::is_none")]
    pub any_content_blocked: Option<bool>,
}

impl ProjectDataPolicy {
    /// Details about data policy.
    pub fn new() -> ProjectDataPolicy {
        ProjectDataPolicy {
            any_content_blocked: None,
        }
    }
}

