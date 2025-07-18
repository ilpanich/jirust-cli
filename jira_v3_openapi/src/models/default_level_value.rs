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

/// DefaultLevelValue : Details of scheme and new default level.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultLevelValue {
    /// The ID of the issue security level to set as default for the specified scheme. Providing null will reset the default level.
    #[serde(rename = "defaultLevelId")]
    pub default_level_id: String,
    /// The ID of the issue security scheme to set default level for.
    #[serde(rename = "issueSecuritySchemeId")]
    pub issue_security_scheme_id: String,
}

impl DefaultLevelValue {
    /// Details of scheme and new default level.
    pub fn new(default_level_id: String, issue_security_scheme_id: String) -> DefaultLevelValue {
        DefaultLevelValue {
            default_level_id,
            issue_security_scheme_id,
        }
    }
}

