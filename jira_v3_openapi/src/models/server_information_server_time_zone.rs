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

/// ServerInformationServerTimeZone : The default timezone of the Jira server. In a format known as Olson Time Zones, IANA Time Zones or TZ Database Time Zones.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerInformationServerTimeZone {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "dstsavings", skip_serializing_if = "Option::is_none")]
    pub dstsavings: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "rawOffset", skip_serializing_if = "Option::is_none")]
    pub raw_offset: Option<i32>,
}

impl ServerInformationServerTimeZone {
    /// The default timezone of the Jira server. In a format known as Olson Time Zones, IANA Time Zones or TZ Database Time Zones.
    pub fn new() -> ServerInformationServerTimeZone {
        ServerInformationServerTimeZone {
            display_name: None,
            dstsavings: None,
            id: None,
            raw_offset: None,
        }
    }
}

