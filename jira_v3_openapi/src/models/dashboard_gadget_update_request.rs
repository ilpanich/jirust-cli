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

/// DashboardGadgetUpdateRequest : The details of the gadget to update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardGadgetUpdateRequest {
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The position of the gadget.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<models::DashboardGadgetPosition>>,
    /// The title of the gadget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl DashboardGadgetUpdateRequest {
    /// The details of the gadget to update.
    pub fn new() -> DashboardGadgetUpdateRequest {
        DashboardGadgetUpdateRequest {
            color: None,
            position: None,
            title: None,
        }
    }
}

