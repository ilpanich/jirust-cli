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

/// UpdateUiModificationDetails : The details of a UI modification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUiModificationDetails {
    /// List of contexts of the UI modification. The maximum number of contexts is 1000. If provided, replaces all existing contexts.
    #[serde(rename = "contexts", skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<models::UiModificationContextDetails>>,
    /// The data of the UI modification. The maximum size of the data is 50000 characters.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The description of the UI modification. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the UI modification. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateUiModificationDetails {
    /// The details of a UI modification.
    pub fn new() -> UpdateUiModificationDetails {
        UpdateUiModificationDetails {
            contexts: None,
            data: None,
            description: None,
            name: None,
        }
    }
}

