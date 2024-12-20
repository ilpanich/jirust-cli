/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AddNotificationsDetails : Details of notifications which should be added to the notification scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddNotificationsDetails {
    /// The list of notifications which should be added to the notification scheme.
    #[serde(rename = "notificationSchemeEvents")]
    pub notification_scheme_events: Vec<models::NotificationSchemeEventDetails>,
}

impl AddNotificationsDetails {
    /// Details of notifications which should be added to the notification scheme.
    pub fn new(notification_scheme_events: Vec<models::NotificationSchemeEventDetails>) -> AddNotificationsDetails {
        AddNotificationsDetails {
            notification_scheme_events,
        }
    }
}

