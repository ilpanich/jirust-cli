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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveFieldBean {
    /// The ID of the screen tab field after which to place the moved screen tab field. Required if `position` isn't provided.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The named position to which the screen tab field should be moved. Required if `after` isn't provided.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

impl MoveFieldBean {
    pub fn new() -> MoveFieldBean {
        MoveFieldBean {
            after: None,
            position: None,
        }
    }
}
/// The named position to which the screen tab field should be moved. Required if `after` isn't provided.
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

