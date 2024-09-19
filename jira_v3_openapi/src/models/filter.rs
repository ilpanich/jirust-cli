/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Filter : Details about a filter.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    /// \\[Experimental\\] Approximate last used time. Returns the date and time when the filter was last used. Returns `null` if the filter hasn't been used after tracking was enabled. For performance reasons, timestamps aren't updated in real time and therefore may not be exactly accurate.
    #[serde(rename = "approximateLastUsed", skip_serializing_if = "Option::is_none")]
    pub approximate_last_used: Option<String>,
    /// A description of the filter.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The groups and projects that can edit the filter.
    #[serde(rename = "editPermissions", skip_serializing_if = "Option::is_none")]
    pub edit_permissions: Option<Vec<models::SharePermission>>,
    /// Whether the filter is selected as a favorite.
    #[serde(rename = "favourite", skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    /// The count of how many users have selected this filter as a favorite, including the filter owner.
    #[serde(rename = "favouritedCount", skip_serializing_if = "Option::is_none")]
    pub favourited_count: Option<i64>,
    /// The unique identifier for the filter.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The JQL query for the filter. For example, *project = SSP AND issuetype = Bug*.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// The name of the filter. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
    /// The user who owns the filter. This is defaulted to the creator of the filter, however Jira administrators can change the owner of a shared filter in the admin settings.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::User>>,
    /// A URL to view the filter results in Jira, using the [Search for issues using JQL](#api-rest-api-3-filter-search-get) operation with the filter's JQL string to return the filter results. For example, *https://your-domain.atlassian.net/rest/api/3/search?jql=project+%3D+SSP+AND+issuetype+%3D+Bug*.
    #[serde(rename = "searchUrl", skip_serializing_if = "Option::is_none")]
    pub search_url: Option<String>,
    /// The URL of the filter.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The groups and projects that the filter is shared with.
    #[serde(rename = "sharePermissions", skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<models::SharePermission>>,
    /// A paginated list of the users that the filter is shared with. This includes users that are members of the groups or can browse the projects that the filter is shared with.
    #[serde(rename = "sharedUsers", skip_serializing_if = "Option::is_none")]
    pub shared_users: Option<Box<models::UserList>>,
    /// A paginated list of the users that are subscribed to the filter.
    #[serde(rename = "subscriptions", skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Box<models::FilterSubscriptionsList>>,
    /// A URL to view the filter results in Jira, using the ID of the filter. For example, *https://your-domain.atlassian.net/issues/?filter=10100*.
    #[serde(rename = "viewUrl", skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}

impl Filter {
    /// Details about a filter.
    pub fn new(name: String) -> Filter {
        Filter {
            approximate_last_used: None,
            description: None,
            edit_permissions: None,
            favourite: None,
            favourited_count: None,
            id: None,
            jql: None,
            name,
            owner: None,
            search_url: None,
            param_self: None,
            share_permissions: None,
            shared_users: None,
            subscriptions: None,
            view_url: None,
        }
    }
}

