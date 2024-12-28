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

/// AnnouncementBannerConfigurationUpdate : Configuration of the announcement banner.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnouncementBannerConfigurationUpdate {
    /// Flag indicating if the announcement banner can be dismissed by the user.
    #[serde(rename = "isDismissible", skip_serializing_if = "Option::is_none")]
    pub is_dismissible: Option<bool>,
    /// Flag indicating if the announcement banner is enabled or not.
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// The text on the announcement banner.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Visibility of the announcement banner. Can be public or private.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl AnnouncementBannerConfigurationUpdate {
    /// Configuration of the announcement banner.
    pub fn new() -> AnnouncementBannerConfigurationUpdate {
        AnnouncementBannerConfigurationUpdate {
            is_dismissible: None,
            is_enabled: None,
            message: None,
            visibility: None,
        }
    }
}

