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

/// ProjectAvatars : List of project avatars.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectAvatars {
    /// List of avatars added to Jira. These avatars may be deleted.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<models::Avatar>>,
    /// List of avatars included with Jira. These avatars cannot be deleted.
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<models::Avatar>>,
}

impl ProjectAvatars {
    /// List of project avatars.
    pub fn new() -> ProjectAvatars {
        ProjectAvatars {
            custom: None,
            system: None,
        }
    }
}
