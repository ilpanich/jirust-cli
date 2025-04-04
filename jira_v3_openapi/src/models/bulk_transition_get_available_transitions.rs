/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BulkTransitionGetAvailableTransitions : Bulk Transition Get Available Transitions Response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkTransitionGetAvailableTransitions {
    /// List of available transitions for bulk transition operation for requested issues grouped by workflow
    #[serde(rename = "availableTransitions", skip_serializing_if = "Option::is_none")]
    pub available_transitions: Option<Vec<models::IssueBulkTransitionForWorkflow>>,
    /// The end cursor for use in pagination.
    #[serde(rename = "endingBefore", skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// The start cursor for use in pagination.
    #[serde(rename = "startingAfter", skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl BulkTransitionGetAvailableTransitions {
    /// Bulk Transition Get Available Transitions Response.
    pub fn new() -> BulkTransitionGetAvailableTransitions {
        BulkTransitionGetAvailableTransitions {
            available_transitions: None,
            ending_before: None,
            starting_after: None,
        }
    }
}

