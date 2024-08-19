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

/// FilterSubscription : Details of a user or group subscribing to a filter.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterSubscription {
    /// The group subscribing to filter.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<models::GroupName>>,
    /// The ID of the filter subscription.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The user subscribing to filter.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
}

impl FilterSubscription {
    /// Details of a user or group subscribing to a filter.
    pub fn new() -> FilterSubscription {
        FilterSubscription {
            group: None,
            id: None,
            user: None,
        }
    }
}

