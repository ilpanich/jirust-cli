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

/// SecurityLevelPayload : The payload for creating a security level. See https://support.atlassian.com/jira-cloud-administration/docs/configure-issue-security-schemes/
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityLevelPayload {
    /// The description of the security level
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the security level is default for the security scheme
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the security level
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The members of the security level
    #[serde(rename = "securityLevelMembers", skip_serializing_if = "Option::is_none")]
    pub security_level_members: Option<Vec<models::SecurityLevelMemberPayload>>,
}

impl SecurityLevelPayload {
    /// The payload for creating a security level. See https://support.atlassian.com/jira-cloud-administration/docs/configure-issue-security-schemes/
    pub fn new() -> SecurityLevelPayload {
        SecurityLevelPayload {
            description: None,
            is_default: None,
            name: None,
            security_level_members: None,
        }
    }
}

