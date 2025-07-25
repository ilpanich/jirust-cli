/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NotificationSchemeEventDetails : Details of a notification scheme event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationSchemeEventDetails {
    /// The ID of the event.
    #[serde(rename = "event")]
    pub event: models::NotificationSchemeEventTypeId,
    /// The list of notifications mapped to a specified event.
    #[serde(rename = "notifications")]
    pub notifications: Vec<models::NotificationSchemeNotificationDetails>,
}

impl NotificationSchemeEventDetails {
    /// Details of a notification scheme event.
    pub fn new(event: models::NotificationSchemeEventTypeId, notifications: Vec<models::NotificationSchemeNotificationDetails>) -> NotificationSchemeEventDetails {
        NotificationSchemeEventDetails {
            event,
            notifications,
        }
    }
}

