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

/// IssueBulkEditPayload : Issue Bulk Edit Payload
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueBulkEditPayload {
    /// An object that defines the values to be updated in specified fields of an issue. The structure and content of this parameter vary depending on the type of field being edited. Although the order is not significant, ensure that field IDs align with those in selectedActions.
    #[serde(rename = "editedFieldsInput")]
    pub edited_fields_input: Box<models::JiraIssueFields>,
    /// List of all the field IDs that are to be bulk edited. Each field ID in this list corresponds to a specific attribute of an issue that is set to be modified in the bulk edit operation. The relevant field ID can be obtained by calling the Bulk Edit Get Fields REST API (documentation available on this page itself).
    #[serde(rename = "selectedActions")]
    pub selected_actions: Vec<String>,
    /// List of issue IDs or keys which are to be bulk edited. These IDs or keys can be from different projects and issue types.
    #[serde(rename = "selectedIssueIdsOrKeys")]
    pub selected_issue_ids_or_keys: Vec<String>,
    /// A boolean value that indicates whether to send a bulk change notification when the issues are being edited.  If `true`, dispatches a bulk notification email to users about the updates.
    #[serde(rename = "sendBulkNotification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub send_bulk_notification: Option<Option<bool>>,
}

impl IssueBulkEditPayload {
    /// Issue Bulk Edit Payload
    pub fn new(edited_fields_input: models::JiraIssueFields, selected_actions: Vec<String>, selected_issue_ids_or_keys: Vec<String>) -> IssueBulkEditPayload {
        IssueBulkEditPayload {
            edited_fields_input: Box::new(edited_fields_input),
            selected_actions,
            selected_issue_ids_or_keys,
            send_bulk_notification: None,
        }
    }
}

