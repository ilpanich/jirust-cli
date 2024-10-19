/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RemoteIssueLinkRequest : Details of a remote issue link.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteIssueLinkRequest {
    /// Details of the remote application the linked item is in. For example, trello.
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<models::Application>,
    /// An identifier for the remote item in the remote system. For example, the global ID for a remote item in Confluence would consist of the app ID and page ID, like this: `appId=456&pageId=123`.  Setting this field enables the remote issue link details to be updated or deleted using remote system and item details as the record identifier, rather than using the record's Jira ID.  The maximum length is 255 characters.
    #[serde(rename = "globalId", skip_serializing_if = "Option::is_none")]
    pub global_id: Option<String>,
    /// Details of the item linked to.
    #[serde(rename = "object")]
    pub object: models::RemoteObject,
    /// Description of the relationship between the issue and the linked item. If not set, the relationship description \"links to\" is used in Jira.
    #[serde(rename = "relationship", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
}

impl RemoteIssueLinkRequest {
    /// Details of a remote issue link.
    pub fn new(object: models::RemoteObject) -> RemoteIssueLinkRequest {
        RemoteIssueLinkRequest {
            application: None,
            global_id: None,
            object,
            relationship: None,
        }
    }
}

