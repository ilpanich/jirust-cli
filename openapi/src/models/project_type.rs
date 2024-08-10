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

/// ProjectType : Details about a project type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectType {
    /// The color of the project type.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The key of the project type's description.
    #[serde(rename = "descriptionI18nKey", skip_serializing_if = "Option::is_none")]
    pub description_i18n_key: Option<String>,
    /// The formatted key of the project type.
    #[serde(rename = "formattedKey", skip_serializing_if = "Option::is_none")]
    pub formatted_key: Option<String>,
    /// The icon of the project type.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The key of the project type.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl ProjectType {
    /// Details about a project type.
    pub fn new() -> ProjectType {
        ProjectType {
            color: None,
            description_i18n_key: None,
            formatted_key: None,
            icon: None,
            key: None,
        }
    }
}

