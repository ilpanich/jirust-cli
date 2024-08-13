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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvatarUrlsBean {
    /// The URL of the item's 16x16 pixel avatar.
    #[serde(rename = "16x16", skip_serializing_if = "Option::is_none")]
    pub param_16x16: Option<String>,
    /// The URL of the item's 24x24 pixel avatar.
    #[serde(rename = "24x24", skip_serializing_if = "Option::is_none")]
    pub param_24x24: Option<String>,
    /// The URL of the item's 32x32 pixel avatar.
    #[serde(rename = "32x32", skip_serializing_if = "Option::is_none")]
    pub param_32x32: Option<String>,
    /// The URL of the item's 48x48 pixel avatar.
    #[serde(rename = "48x48", skip_serializing_if = "Option::is_none")]
    pub param_48x48: Option<String>,
}

impl AvatarUrlsBean {
    pub fn new() -> AvatarUrlsBean {
        AvatarUrlsBean {
            param_16x16: None,
            param_24x24: None,
            param_32x32: None,
            param_48x48: None,
        }
    }
}

