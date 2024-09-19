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

/// TargetToSourcesMapping : An object representing the mapping of issues and data related to destination entities, like fields and statuses, that are required during a bulk move.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetToSourcesMapping {
    /// If `true`, when issues are moved into this target group, they will adopt the target project's default classification, if they don't have a classification already. If they do have a classification, it will be kept the same even after the move. Leave `targetClassification` empty when using this.  If `false`, you must provide a `targetClassification` mapping for each classification associated with the selected issues.  [Benefit from data classification](https://support.atlassian.com/security-and-access-policies/docs/what-is-data-classification/)
    #[serde(rename = "inferClassificationDefaults")]
    pub infer_classification_defaults: bool,
    /// If `true`, values from the source issues will be retained for the mandatory fields in the field configuration of the destination project. The `targetMandatoryFields` property shouldn't be defined.  If `false`, the user is required to set values for mandatory fields present in the field configuration of the destination project. Provide input by defining the `targetMandatoryFields` property
    #[serde(rename = "inferFieldDefaults")]
    pub infer_field_defaults: bool,
    /// If `true`, the statuses of issues being moved in this target group that are not present in the target workflow will be changed to the default status of the target workflow (see below). Leave `targetStatus` empty when using this.  If `false`, you must provide a `targetStatus` for each status not present in the target workflow.  The default status in a workflow is referred to as the \"initial status\". Each workflow has its own unique initial status. When an issue is created, it is automatically assigned to this initial status. Read more about configuring initial statuses: [Configure the initial status | Atlassian Support.](https://support.atlassian.com/jira-cloud-administration/docs/configure-the-initial-status/)
    #[serde(rename = "inferStatusDefaults")]
    pub infer_status_defaults: bool,
    /// When an issue is moved, its subtasks (if there are any) need to be moved with it. `inferSubtaskTypeDefault` helps with moving the subtasks by picking a random subtask type in the target project.  If `true`, subtasks will automatically move to the same project as their parent.  When they move:   *  Their `issueType` will be set to the default for subtasks in the target project.  *  Values for mandatory fields will be retained from the source issues  *  Specifying separate mapping for implicit subtasks won’t be allowed.  If `false`, you must manually move the subtasks. They will retain the parent which they had in the current project after being moved.
    #[serde(rename = "inferSubtaskTypeDefault")]
    pub infer_subtask_type_default: bool,
    /// List of issue IDs or keys to be moved. These issues must be from the same project, have the same issue type, and be from the same parent (if they’re subtasks).
    #[serde(rename = "issueIdsOrKeys", skip_serializing_if = "Option::is_none")]
    pub issue_ids_or_keys: Option<Vec<String>>,
    /// List of the objects containing classifications in the source issues and their new values which need to be set during the bulk move operation.   *  **You should only define this property when `inferClassificationDefaults` is `false`.**  *  **In order to provide mapping for issues which don't have a classification, use `\"-1\"`.**
    #[serde(rename = "targetClassification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_classification: Option<Option<Vec<models::TargetClassification>>>,
    /// List of objects containing mandatory fields in the target field configuration and new values that need to be set during the bulk move operation.  The new values will only be applied if the field is mandatory in the target project and at least one issue from the source has that field empty, or if the field context is different in the target project (e.g. project-scoped version fields).  **You should only define this property when `inferFieldDefaults` is `false`.**
    #[serde(rename = "targetMandatoryFields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_mandatory_fields: Option<Option<Vec<models::TargetMandatoryFields>>>,
    /// List of the objects containing statuses in the source workflow and their new values which need to be set during the bulk move operation.  The new values will only be applied if the source status is invalid for the target project and issue type.  **You should only define this property when `inferStatusDefaults` is `false`.**
    #[serde(rename = "targetStatus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_status: Option<Option<Vec<models::TargetStatus>>>,
}

impl TargetToSourcesMapping {
    /// An object representing the mapping of issues and data related to destination entities, like fields and statuses, that are required during a bulk move.
    pub fn new(infer_classification_defaults: bool, infer_field_defaults: bool, infer_status_defaults: bool, infer_subtask_type_default: bool) -> TargetToSourcesMapping {
        TargetToSourcesMapping {
            infer_classification_defaults,
            infer_field_defaults,
            infer_status_defaults,
            infer_subtask_type_default,
            issue_ids_or_keys: None,
            target_classification: None,
            target_mandatory_fields: None,
            target_status: None,
        }
    }
}

