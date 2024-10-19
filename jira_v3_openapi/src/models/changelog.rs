/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Changelog : A log of changes made to issue fields. Changelogs related to workflow associations are currently being deprecated.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Changelog {
    /// The user who made the change.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::UserDetails>>,
    /// The date on which the change took place.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The history metadata associated with the changed.
    #[serde(rename = "historyMetadata", skip_serializing_if = "Option::is_none")]
    pub history_metadata: Option<models::HistoryMetadata>,
    /// The ID of the changelog.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list of items changed.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::ChangeDetails>>,
}

impl Changelog {
    /// A log of changes made to issue fields. Changelogs related to workflow associations are currently being deprecated.
    pub fn new() -> Changelog {
        Changelog {
            author: None,
            created: None,
            history_metadata: None,
            id: None,
            items: None,
        }
    }
}

