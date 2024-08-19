/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GroupLabel : A group label.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupLabel {
    /// The group label name.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The title of the group label.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of the group label.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl GroupLabel {
    /// A group label.
    pub fn new() -> GroupLabel {
        GroupLabel {
            text: None,
            title: None,
            r#type: None,
        }
    }
}
/// The type of the group label.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "MULTIPLE")]
    Multiple,
}

impl Default for Type {
    fn default() -> Type {
        Self::Admin
    }
}

