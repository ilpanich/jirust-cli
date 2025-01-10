/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// HistoryMetadata : Details of issue history metadata.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryMetadata {
    /// The activity described in the history record.
    #[serde(rename = "activityDescription", skip_serializing_if = "Option::is_none")]
    pub activity_description: Option<String>,
    /// The key of the activity described in the history record.
    #[serde(rename = "activityDescriptionKey", skip_serializing_if = "Option::is_none")]
    pub activity_description_key: Option<String>,
    /// Details of the user whose action created the history record.
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<models::HistoryMetadataParticipant>,
    /// Details of the cause that triggered the creation the history record.
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<models::HistoryMetadataParticipant>,
    /// The description of the history record.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The description key of the history record.
    #[serde(rename = "descriptionKey", skip_serializing_if = "Option::is_none")]
    pub description_key: Option<String>,
    /// The description of the email address associated the history record.
    #[serde(rename = "emailDescription", skip_serializing_if = "Option::is_none")]
    pub email_description: Option<String>,
    /// The description key of the email address associated the history record.
    #[serde(rename = "emailDescriptionKey", skip_serializing_if = "Option::is_none")]
    pub email_description_key: Option<String>,
    /// Additional arbitrary information about the history record.
    #[serde(rename = "extraData", skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<std::collections::HashMap<String, String>>,
    /// Details of the system that generated the history record.
    #[serde(rename = "generator", skip_serializing_if = "Option::is_none")]
    pub generator: Option<models::HistoryMetadataParticipant>,
    /// The type of the history record.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl HistoryMetadata {
    /// Details of issue history metadata.
    pub fn new() -> HistoryMetadata {
        HistoryMetadata {
            activity_description: None,
            activity_description_key: None,
            actor: None,
            cause: None,
            description: None,
            description_key: None,
            email_description: None,
            email_description_key: None,
            extra_data: None,
            generator: None,
            r#type: None,
        }
    }
}

