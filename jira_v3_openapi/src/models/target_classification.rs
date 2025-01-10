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

/// TargetClassification : Classification mapping for classifications in source issues to respective target classification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetClassification {
    /// An object with the key as the ID of the target classification and value with the list of the IDs of the current source classifications.
    #[serde(rename = "classifications")]
    pub classifications: std::collections::HashMap<String, Vec<String>>,
    /// ID of the source issueType to which issues present in `issueIdOrKeys` belongs.
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    /// ID or key of the source project to which issues present in `issueIdOrKeys` belongs.
    #[serde(rename = "projectKeyOrId", skip_serializing_if = "Option::is_none")]
    pub project_key_or_id: Option<String>,
}

impl TargetClassification {
    /// Classification mapping for classifications in source issues to respective target classification.
    pub fn new(classifications: std::collections::HashMap<String, Vec<String>>) -> TargetClassification {
        TargetClassification {
            classifications,
            issue_type: None,
            project_key_or_id: None,
        }
    }
}

