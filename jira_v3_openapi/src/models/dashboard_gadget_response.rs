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

/// DashboardGadgetResponse : The list of gadgets on the dashboard.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardGadgetResponse {
    /// The list of gadgets.
    #[serde(rename = "gadgets")]
    pub gadgets: Vec<models::DashboardGadget>,
}

impl DashboardGadgetResponse {
    /// The list of gadgets on the dashboard.
    pub fn new(gadgets: Vec<models::DashboardGadget>) -> DashboardGadgetResponse {
        DashboardGadgetResponse {
            gadgets,
        }
    }
}

