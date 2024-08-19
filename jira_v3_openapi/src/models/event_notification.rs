/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EventNotification : Details about a notification associated with an event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventNotification {
    /// The email address.
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Expand options that include additional event notification details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The custom user or group field.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<Box<models::FieldDetails>>,
    /// The specified group.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<models::GroupName>>,
    /// The ID of the notification.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Identifies the recipients of the notification.
    #[serde(rename = "notificationType", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<NotificationType>,
    /// As a group's name can change, use of `recipient` is recommended. The identifier associated with the `notificationType` value that defines the receiver of the notification, where the receiver isn't implied by `notificationType` value. So, when `notificationType` is:   *  `User` The `parameter` is the user account ID.  *  `Group` The `parameter` is the group name.  *  `ProjectRole` The `parameter` is the project role ID.  *  `UserCustomField` The `parameter` is the ID of the custom field.  *  `GroupCustomField` The `parameter` is the ID of the custom field.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// The specified project role.
    #[serde(rename = "projectRole", skip_serializing_if = "Option::is_none")]
    pub project_role: Option<Box<models::ProjectRole>>,
    /// The identifier associated with the `notificationType` value that defines the receiver of the notification, where the receiver isn't implied by the `notificationType` value. So, when `notificationType` is:   *  `User`, `recipient` is the user account ID.  *  `Group`, `recipient` is the group ID.  *  `ProjectRole`, `recipient` is the project role ID.  *  `UserCustomField`, `recipient` is the ID of the custom field.  *  `GroupCustomField`, `recipient` is the ID of the custom field.
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The specified user.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::UserDetails>>,
}

impl EventNotification {
    /// Details about a notification associated with an event.
    pub fn new() -> EventNotification {
        EventNotification {
            email_address: None,
            expand: None,
            field: None,
            group: None,
            id: None,
            notification_type: None,
            parameter: None,
            project_role: None,
            recipient: None,
            user: None,
        }
    }
}
/// Identifies the recipients of the notification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "CurrentAssignee")]
    CurrentAssignee,
    #[serde(rename = "Reporter")]
    Reporter,
    #[serde(rename = "CurrentUser")]
    CurrentUser,
    #[serde(rename = "ProjectLead")]
    ProjectLead,
    #[serde(rename = "ComponentLead")]
    ComponentLead,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Group")]
    Group,
    #[serde(rename = "ProjectRole")]
    ProjectRole,
    #[serde(rename = "EmailAddress")]
    EmailAddress,
    #[serde(rename = "AllWatchers")]
    AllWatchers,
    #[serde(rename = "UserCustomField")]
    UserCustomField,
    #[serde(rename = "GroupCustomField")]
    GroupCustomField,
}

impl Default for NotificationType {
    fn default() -> NotificationType {
        Self::CurrentAssignee
    }
}

