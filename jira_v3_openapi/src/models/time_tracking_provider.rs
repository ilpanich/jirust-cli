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

/// TimeTrackingProvider : Details about the time tracking provider.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeTrackingProvider {
    /// The key for the time tracking provider. For example, *JIRA*.
    #[serde(rename = "key")]
    pub key: String,
    /// The name of the time tracking provider. For example, *JIRA provided time tracking*.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the configuration page for the time tracking provider app. For example, *_/example/config/url*. This property is only returned if the `adminPageKey` property is set in the module descriptor of the time tracking provider app.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TimeTrackingProvider {
    /// Details about the time tracking provider.
    pub fn new(key: String) -> TimeTrackingProvider {
        TimeTrackingProvider {
            key,
            name: None,
            url: None,
        }
    }
}

