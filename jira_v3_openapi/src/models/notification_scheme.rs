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

/// NotificationScheme : Details about a notification scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationScheme {
    /// The description of the notification scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Expand options that include additional notification scheme details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The ID of the notification scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the notification scheme.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The notification events and associated recipients.
    #[serde(rename = "notificationSchemeEvents", skip_serializing_if = "Option::is_none")]
    pub notification_scheme_events: Option<Vec<models::NotificationSchemeEvent>>,
    /// The list of project IDs associated with the notification scheme.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<i64>>,
    /// The scope of the notification scheme.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl NotificationScheme {
    /// Details about a notification scheme.
    pub fn new() -> NotificationScheme {
        NotificationScheme {
            description: None,
            expand: None,
            id: None,
            name: None,
            notification_scheme_events: None,
            projects: None,
            scope: None,
            param_self: None,
        }
    }
}

