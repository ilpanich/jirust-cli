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

/// NotificationRecipientsRestrictions : Details of the group membership or permissions needed to receive the notification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRecipientsRestrictions {
    /// List of groupId memberships required to receive the notification.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of group memberships required to receive the notification.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<models::GroupName>>,
    /// List of permissions required to receive the notification.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<models::RestrictedPermission>>,
}

impl NotificationRecipientsRestrictions {
    /// Details of the group membership or permissions needed to receive the notification.
    pub fn new() -> NotificationRecipientsRestrictions {
        NotificationRecipientsRestrictions {
            group_ids: None,
            groups: None,
            permissions: None,
        }
    }
}

