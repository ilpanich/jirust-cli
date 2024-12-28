/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NotificationSchemeEventTypeId : The ID of an event that is being mapped to notifications.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationSchemeEventTypeId {
    /// The ID of the notification scheme event.
    #[serde(rename = "id")]
    pub id: String,
}

impl NotificationSchemeEventTypeId {
    /// The ID of an event that is being mapped to notifications.
    pub fn new(id: String) -> NotificationSchemeEventTypeId {
        NotificationSchemeEventTypeId {
            id,
        }
    }
}

