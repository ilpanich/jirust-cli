/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ProjectFeature : Details of a project feature.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectFeature {
    /// The key of the feature.
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// URI for the image representing the feature.
    #[serde(rename = "imageUri", skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    /// Localized display description for the feature.
    #[serde(rename = "localisedDescription", skip_serializing_if = "Option::is_none")]
    pub localised_description: Option<String>,
    /// Localized display name for the feature.
    #[serde(rename = "localisedName", skip_serializing_if = "Option::is_none")]
    pub localised_name: Option<String>,
    /// List of keys of the features required to enable the feature.
    #[serde(rename = "prerequisites", skip_serializing_if = "Option::is_none")]
    pub prerequisites: Option<Vec<String>>,
    /// The ID of the project.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The state of the feature. When updating the state of a feature, only ENABLED and DISABLED are supported. Responses can contain all values
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Whether the state of the feature can be updated.
    #[serde(rename = "toggleLocked", skip_serializing_if = "Option::is_none")]
    pub toggle_locked: Option<bool>,
}

impl ProjectFeature {
    /// Details of a project feature.
    pub fn new() -> ProjectFeature {
        ProjectFeature {
            feature: None,
            image_uri: None,
            localised_description: None,
            localised_name: None,
            prerequisites: None,
            project_id: None,
            state: None,
            toggle_locked: None,
        }
    }
}
/// The state of the feature. When updating the state of a feature, only ENABLED and DISABLED are supported. Responses can contain all values
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "COMING_SOON")]
    ComingSoon,
}

impl Default for State {
    fn default() -> State {
        Self::Enabled
    }
}

