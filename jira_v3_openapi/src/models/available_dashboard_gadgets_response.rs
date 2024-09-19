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

/// AvailableDashboardGadgetsResponse : The list of available gadgets.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableDashboardGadgetsResponse {
    /// The list of available gadgets.
    #[serde(rename = "gadgets")]
    pub gadgets: Vec<models::AvailableDashboardGadget>,
}

impl AvailableDashboardGadgetsResponse {
    /// The list of available gadgets.
    pub fn new(gadgets: Vec<models::AvailableDashboardGadget>) -> AvailableDashboardGadgetsResponse {
        AvailableDashboardGadgetsResponse {
            gadgets,
        }
    }
}

