/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AvailableDashboardGadget : The details of the available dashboard gadget.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableDashboardGadget {
    /// The module key of the gadget type.
    #[serde(rename = "moduleKey", skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    /// The title of the gadget.
    #[serde(rename = "title")]
    pub title: String,
    /// The URI of the gadget type.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl AvailableDashboardGadget {
    /// The details of the available dashboard gadget.
    pub fn new(title: String) -> AvailableDashboardGadget {
        AvailableDashboardGadget {
            module_key: None,
            title,
            uri: None,
        }
    }
}

