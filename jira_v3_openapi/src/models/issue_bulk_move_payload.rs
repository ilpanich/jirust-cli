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

/// IssueBulkMovePayload : Issue Bulk Move Payload
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueBulkMovePayload {
    /// A boolean value that indicates whether to send a bulk change notification when the issues are being moved.  If `true`, dispatches a bulk notification email to users about the updates.
    #[serde(rename = "sendBulkNotification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub send_bulk_notification: Option<Option<bool>>,
    /// An object representing the mapping of issues and data related to destination entities, like fields and statuses, that are required during a bulk move.  The key is a string that is created by concatenating the following three entities in order, separated by commas. The format is `<project ID or key>,<issueType ID>,<parent ID or key>`. It should be unique across mappings provided in the payload. If you provide multiple mappings for the same key, only one will be processed. However, the operation won't fail, so the error may be hard to track down.   *  ***Destination project*** (Required): ID or key of the project to which the issues are being moved.  *  ***Destination issueType*** (Required): ID of the issueType to which the issues are being moved.  *  ***Destination parent ID or key*** (Optional): ID or key of the issue which will become the parent of the issues being moved. Only required when the destination issueType is a subtask.
    #[serde(rename = "targetToSourcesMapping", skip_serializing_if = "Option::is_none")]
    pub target_to_sources_mapping: Option<std::collections::HashMap<String, models::TargetToSourcesMapping>>,
}

impl IssueBulkMovePayload {
    /// Issue Bulk Move Payload
    pub fn new() -> IssueBulkMovePayload {
        IssueBulkMovePayload {
            send_bulk_notification: None,
            target_to_sources_mapping: None,
        }
    }
}

