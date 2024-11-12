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

/// NotificationEvent : Details about a notification event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationEvent {
    /// The description of the event.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the event. The event can be a [Jira system event](https://confluence.atlassian.com/x/8YdKLg#Creatinganotificationscheme-eventsEvents) or a [custom event](https://confluence.atlassian.com/x/AIlKLg).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the event.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The template of the event. Only custom events configured by Jira administrators have template.
    #[serde(rename = "templateEvent", skip_serializing_if = "Option::is_none")]
    pub template_event: Option<Box<models::NotificationEvent>>,
}

impl NotificationEvent {
    /// Details about a notification event.
    pub fn new() -> NotificationEvent {
        NotificationEvent {
            description: None,
            id: None,
            name: None,
            template_event: None,
        }
    }
}

