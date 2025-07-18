/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ScreenSchemePayload : Defines the payload for the screen schemes. See https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-screen-schemes/\\#api-rest-api-3-screenscheme-post
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenSchemePayload {
    #[serde(rename = "defaultScreen", skip_serializing_if = "Option::is_none")]
    pub default_screen: Option<Box<models::ProjectCreateResourceIdentifier>>,
    /// The description of the screen scheme
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen scheme
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pcri", skip_serializing_if = "Option::is_none")]
    pub pcri: Option<Box<models::ProjectCreateResourceIdentifier>>,
    /// Similar to the field layout scheme those mappings allow users to set different screens for different operations: default - always there, applied to all operations that don't have an explicit mapping `create`, `view`, `edit` - specific operations that are available and users can assign a different screen for each one of them https://support.atlassian.com/jira-cloud-administration/docs/manage-screen-schemes/\\#Associating-a-screen-with-an-issue-operation
    #[serde(rename = "screens", skip_serializing_if = "Option::is_none")]
    pub screens: Option<std::collections::HashMap<String, models::ProjectCreateResourceIdentifier>>,
}

impl ScreenSchemePayload {
    /// Defines the payload for the screen schemes. See https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-screen-schemes/\\#api-rest-api-3-screenscheme-post
    pub fn new() -> ScreenSchemePayload {
        ScreenSchemePayload {
            default_screen: None,
            description: None,
            name: None,
            pcri: None,
            screens: None,
        }
    }
}

