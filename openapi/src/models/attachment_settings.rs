/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AttachmentSettings : Details of the instance's attachment settings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentSettings {
    /// Whether the ability to add attachments is enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The maximum size of attachments permitted, in bytes.
    #[serde(rename = "uploadLimit", skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i64>,
}

impl AttachmentSettings {
    /// Details of the instance's attachment settings.
    pub fn new() -> AttachmentSettings {
        AttachmentSettings {
            enabled: None,
            upload_limit: None,
        }
    }
}

