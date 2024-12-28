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

/// RemoteObject : The linked item.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteObject {
    /// Details of the icon for the item. If no icon is defined, the default link icon is used in Jira.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<models::Icon>,
    /// The status of the item.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::Status>,
    /// The summary details of the item.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The title of the item.
    #[serde(rename = "title")]
    pub title: String,
    /// The URL of the item.
    #[serde(rename = "url")]
    pub url: String,
}

impl RemoteObject {
    /// The linked item.
    pub fn new(title: String, url: String) -> RemoteObject {
        RemoteObject {
            icon: None,
            status: None,
            summary: None,
            title,
            url,
        }
    }
}

