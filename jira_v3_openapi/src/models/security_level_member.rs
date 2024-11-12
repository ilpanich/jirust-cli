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

/// SecurityLevelMember : Issue security level member.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityLevelMember {
    /// The user or group being granted the permission. It consists of a `type` and a type-dependent `parameter`. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information.
    #[serde(rename = "holder")]
    pub holder: Box<models::PermissionHolder>,
    /// The ID of the issue security level member.
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the issue security level.
    #[serde(rename = "issueSecurityLevelId")]
    pub issue_security_level_id: String,
    /// The ID of the issue security scheme.
    #[serde(rename = "issueSecuritySchemeId")]
    pub issue_security_scheme_id: String,
    #[serde(rename = "managed", skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
}

impl SecurityLevelMember {
    /// Issue security level member.
    pub fn new(holder: models::PermissionHolder, id: String, issue_security_level_id: String, issue_security_scheme_id: String) -> SecurityLevelMember {
        SecurityLevelMember {
            holder: Box::new(holder),
            id,
            issue_security_level_id,
            issue_security_scheme_id,
            managed: None,
        }
    }
}

