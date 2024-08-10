/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NotificationRecipients : Details of the users and groups to receive the notification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRecipients {
    /// Whether the notification should be sent to the issue's assignees.
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<bool>,
    /// List of groupIds to receive the notification.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of groups to receive the notification.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<models::GroupName>>,
    /// Whether the notification should be sent to the issue's reporter.
    #[serde(rename = "reporter", skip_serializing_if = "Option::is_none")]
    pub reporter: Option<bool>,
    /// List of users to receive the notification.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::UserDetails>>,
    /// Whether the notification should be sent to the issue's voters.
    #[serde(rename = "voters", skip_serializing_if = "Option::is_none")]
    pub voters: Option<bool>,
    /// Whether the notification should be sent to the issue's watchers.
    #[serde(rename = "watchers", skip_serializing_if = "Option::is_none")]
    pub watchers: Option<bool>,
}

impl NotificationRecipients {
    /// Details of the users and groups to receive the notification.
    pub fn new() -> NotificationRecipients {
        NotificationRecipients {
            assignee: None,
            group_ids: None,
            groups: None,
            reporter: None,
            users: None,
            voters: None,
            watchers: None,
        }
    }
}

