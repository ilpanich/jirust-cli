/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// VersionApprover : Contains details about a version approver.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionApprover {
    /// The Atlassian account ID of the approver.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A description of why the user is declining the approval.
    #[serde(rename = "declineReason", skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<String>,
    /// A description of what the user is approving within the specified version.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The status of the approval, which can be *PENDING*, *APPROVED*, or *DECLINED*
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl VersionApprover {
    /// Contains details about a version approver.
    pub fn new() -> VersionApprover {
        VersionApprover {
            account_id: None,
            decline_reason: None,
            description: None,
            status: None,
        }
    }
}

