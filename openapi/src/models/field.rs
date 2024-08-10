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

/// Field : Details of a field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// Number of contexts where the field is used.
    #[serde(rename = "contextsCount", skip_serializing_if = "Option::is_none")]
    pub contexts_count: Option<i64>,
    /// The description of the field.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the field.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the field is locked.
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// Whether the field is shown on screen or not.
    #[serde(rename = "isUnscreenable", skip_serializing_if = "Option::is_none")]
    pub is_unscreenable: Option<bool>,
    /// The key of the field.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "lastUsed", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<Box<models::FieldLastUsed>>,
    /// The name of the field.
    #[serde(rename = "name")]
    pub name: String,
    /// Number of projects where the field is used.
    #[serde(rename = "projectsCount", skip_serializing_if = "Option::is_none")]
    pub projects_count: Option<i64>,
    #[serde(rename = "schema")]
    pub schema: Box<models::JsonTypeBean>,
    /// Number of screens where the field is used.
    #[serde(rename = "screensCount", skip_serializing_if = "Option::is_none")]
    pub screens_count: Option<i64>,
    /// The searcher key of the field. Returned for custom fields.
    #[serde(rename = "searcherKey", skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<String>,
    /// The stable ID of the field.
    #[serde(rename = "stableId", skip_serializing_if = "Option::is_none")]
    pub stable_id: Option<String>,
}

impl Field {
    /// Details of a field.
    pub fn new(id: String, name: String, schema: models::JsonTypeBean) -> Field {
        Field {
            contexts_count: None,
            description: None,
            id,
            is_locked: None,
            is_unscreenable: None,
            key: None,
            last_used: None,
            name,
            projects_count: None,
            schema: Box::new(schema),
            screens_count: None,
            searcher_key: None,
            stable_id: None,
        }
    }
}

