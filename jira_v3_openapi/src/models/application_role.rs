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

/// ApplicationRole : Details of an application role.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRole {
    /// The groups that are granted default access for this application role. As a group's name can change, use of `defaultGroupsDetails` is recommended to identify a groups.
    #[serde(rename = "defaultGroups", skip_serializing_if = "Option::is_none")]
    pub default_groups: Option<Vec<String>>,
    /// The groups that are granted default access for this application role.
    #[serde(rename = "defaultGroupsDetails", skip_serializing_if = "Option::is_none")]
    pub default_groups_details: Option<Vec<models::GroupName>>,
    /// Deprecated.
    #[serde(rename = "defined", skip_serializing_if = "Option::is_none")]
    pub defined: Option<bool>,
    /// The groups associated with the application role.
    #[serde(rename = "groupDetails", skip_serializing_if = "Option::is_none")]
    pub group_details: Option<Vec<models::GroupName>>,
    /// The groups associated with the application role. As a group's name can change, use of `groupDetails` is recommended to identify a groups.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "hasUnlimitedSeats", skip_serializing_if = "Option::is_none")]
    pub has_unlimited_seats: Option<bool>,
    /// The key of the application role.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The display name of the application role.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The maximum count of users on your license.
    #[serde(rename = "numberOfSeats", skip_serializing_if = "Option::is_none")]
    pub number_of_seats: Option<i32>,
    /// Indicates if the application role belongs to Jira platform (`jira-core`).
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<bool>,
    /// The count of users remaining on your license.
    #[serde(rename = "remainingSeats", skip_serializing_if = "Option::is_none")]
    pub remaining_seats: Option<i32>,
    /// Determines whether this application role should be selected by default on user creation.
    #[serde(rename = "selectedByDefault", skip_serializing_if = "Option::is_none")]
    pub selected_by_default: Option<bool>,
    /// The number of users counting against your license.
    #[serde(rename = "userCount", skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
    /// The [type of users](https://confluence.atlassian.com/x/lRW3Ng) being counted against your license.
    #[serde(rename = "userCountDescription", skip_serializing_if = "Option::is_none")]
    pub user_count_description: Option<String>,
}

impl ApplicationRole {
    /// Details of an application role.
    pub fn new() -> ApplicationRole {
        ApplicationRole {
            default_groups: None,
            default_groups_details: None,
            defined: None,
            group_details: None,
            groups: None,
            has_unlimited_seats: None,
            key: None,
            name: None,
            number_of_seats: None,
            platform: None,
            remaining_seats: None,
            selected_by_default: None,
            user_count: None,
            user_count_description: None,
        }
    }
}

