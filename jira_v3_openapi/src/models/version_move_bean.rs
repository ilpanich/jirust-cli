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
pub struct VersionMoveBean {
    /// The URL (self link) of the version after which to place the moved version. Cannot be used with `position`.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// An absolute position in which to place the moved version. Cannot be used with `after`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

impl VersionMoveBean {
    pub fn new() -> VersionMoveBean {
        VersionMoveBean {
            after: None,
            position: None,
        }
    }
}
/// An absolute position in which to place the moved version. Cannot be used with `after`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Position {
    #[serde(rename = "Earlier")]
    Earlier,
    #[serde(rename = "Later")]
    Later,
    #[serde(rename = "First")]
    First,
    #[serde(rename = "Last")]
    Last,
}

impl Default for Position {
    fn default() -> Position {
        Self::Earlier
    }
}

