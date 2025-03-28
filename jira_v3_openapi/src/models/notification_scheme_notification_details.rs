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

/// NotificationSchemeNotificationDetails : Details of a notification within a notification scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationSchemeNotificationDetails {
    /// The notification type, e.g `CurrentAssignee`, `Group`, `EmailAddress`.
    #[serde(rename = "notificationType")]
    pub notification_type: String,
    /// The value corresponding to the specified notification type.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
}

impl NotificationSchemeNotificationDetails {
    /// Details of a notification within a notification scheme.
    pub fn new(notification_type: String) -> NotificationSchemeNotificationDetails {
        NotificationSchemeNotificationDetails {
            notification_type,
            parameter: None,
        }
    }
}

