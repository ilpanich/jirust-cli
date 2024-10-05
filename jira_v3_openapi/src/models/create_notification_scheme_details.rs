/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreateNotificationSchemeDetails : Details of an notification scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNotificationSchemeDetails {
    /// The description of the notification scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the notification scheme. Must be unique (case-insensitive).
    #[serde(rename = "name")]
    pub name: String,
    /// The list of notifications which should be added to the notification scheme.
    #[serde(rename = "notificationSchemeEvents", skip_serializing_if = "Option::is_none")]
    pub notification_scheme_events: Option<Vec<models::NotificationSchemeEventDetails>>,
}

impl CreateNotificationSchemeDetails {
    /// Details of an notification scheme.
    pub fn new(name: String) -> CreateNotificationSchemeDetails {
        CreateNotificationSchemeDetails {
            description: None,
            name,
            notification_scheme_events: None,
        }
    }
}

