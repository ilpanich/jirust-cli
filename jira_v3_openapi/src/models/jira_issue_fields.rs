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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraIssueFields {
    /// Add or clear a cascading select field:   *  To add, specify `optionId` for both parent and child.  *  To clear the child, set its `optionId` to null.  *  To clear both, set the parent's `optionId` to null.
    #[serde(rename = "cascadingSelectFields", skip_serializing_if = "Option::is_none")]
    pub cascading_select_fields: Option<Vec<models::JiraCascadingSelectField>>,
    /// Add or clear a number field:   *  To add, specify a numeric `value`.  *  To clear, set `value` to `null`.
    #[serde(rename = "clearableNumberFields", skip_serializing_if = "Option::is_none")]
    pub clearable_number_fields: Option<Vec<models::JiraNumberField>>,
    /// Add or clear a color field:   *  To add, specify the color `name`. Available colors are: `purple`, `blue`, `green`, `teal`, `yellow`, `orange`, `grey`, `dark purple`, `dark blue`, `dark green`, `dark teal`, `dark yellow`, `dark orange`, `dark grey`.  *  To clear, set the color `name` to an empty string.
    #[serde(rename = "colorFields", skip_serializing_if = "Option::is_none")]
    pub color_fields: Option<Vec<models::JiraColorField>>,
    /// Add or clear a date picker field:   *  To add, specify the date in `d/mmm/yy` format or ISO format `dd-mm-yyyy`.  *  To clear, set `formattedDate` to an empty string.
    #[serde(rename = "datePickerFields", skip_serializing_if = "Option::is_none")]
    pub date_picker_fields: Option<Vec<models::JiraDateField>>,
    /// Add or clear the planned start date and time:   *  To add, specify the date and time in ISO format for `formattedDateTime`.  *  To clear, provide an empty string for `formattedDateTime`.
    #[serde(rename = "dateTimePickerFields", skip_serializing_if = "Option::is_none")]
    pub date_time_picker_fields: Option<Vec<models::JiraDateTimeField>>,
    /// Set the issue type field by providing an `issueTypeId`.
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<Box<models::JiraIssueTypeField>>,
    /// Edit a labels field:   *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.  *  To clear labels, use the `REMOVE_ALL` option with an empty `labels` array.
    #[serde(rename = "labelsFields", skip_serializing_if = "Option::is_none")]
    pub labels_fields: Option<Vec<models::JiraLabelsField>>,
    /// Add or clear a multi-group picker field:   *  To add groups, provide an array of groups with `groupName`s.  *  To clear all groups, use an empty `groups` array.
    #[serde(rename = "multipleGroupPickerFields", skip_serializing_if = "Option::is_none")]
    pub multiple_group_picker_fields: Option<Vec<models::JiraMultipleGroupPickerField>>,
    /// Assign or unassign multiple users to/from a field:   *  To assign, provide an array of user `accountId`s.  *  To clear, set `users` to `null`.
    #[serde(rename = "multipleSelectClearableUserPickerFields", skip_serializing_if = "Option::is_none")]
    pub multiple_select_clearable_user_picker_fields: Option<Vec<models::JiraMultipleSelectUserPickerField>>,
    /// Add or clear a multi-select field:   *  To add, provide an array of options with `optionId`s.  *  To clear, use an empty `options` array.
    #[serde(rename = "multipleSelectFields", skip_serializing_if = "Option::is_none")]
    pub multiple_select_fields: Option<Vec<models::JiraMultipleSelectField>>,
    /// Edit a multi-version picker field like Fix Versions/Affects Versions:   *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.  *  To clear the field, use the `REMOVE_ALL` option with an empty `versions` array.
    #[serde(rename = "multipleVersionPickerFields", skip_serializing_if = "Option::is_none")]
    pub multiple_version_picker_fields: Option<Vec<models::JiraMultipleVersionPickerField>>,
    /// Edit a multi select components field:   *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.  *  To clear, use the `REMOVE_ALL` option with an empty `components` array.
    #[serde(rename = "multiselectComponents", skip_serializing_if = "Option::is_none")]
    pub multiselect_components: Option<Box<models::JiraMultiSelectComponentField>>,
    /// Set the priority of an issue by specifying a `priorityId`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Box<models::JiraPriorityField>>,
    /// Add or clear a rich text field:   *  To add, provide `adfValue`. Note that rich text fields only support ADF values.  *  To clear, use an empty `richText` object.  For ADF format details, refer to: [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure).
    #[serde(rename = "richTextFields", skip_serializing_if = "Option::is_none")]
    pub rich_text_fields: Option<Vec<models::JiraRichTextField>>,
    /// Add or clear a single group picker field:   *  To add, specify the group with `groupName`.  *  To clear, set `groupName` to an empty string.
    #[serde(rename = "singleGroupPickerFields", skip_serializing_if = "Option::is_none")]
    pub single_group_picker_fields: Option<Vec<models::JiraSingleGroupPickerField>>,
    /// Add or clear a single line text field:   *  To add, provide the `text` value.  *  To clear, set `text` to an empty string.
    #[serde(rename = "singleLineTextFields", skip_serializing_if = "Option::is_none")]
    pub single_line_text_fields: Option<Vec<models::JiraSingleLineTextField>>,
    /// Edit assignment for single select user picker fields like Assignee/Reporter:   *  To assign an issue, specify the user's `accountId`.  *  To unassign an issue, set `user` to `null`.  *  For automatic assignment, set `accountId` to `-1`.
    #[serde(rename = "singleSelectClearableUserPickerFields", skip_serializing_if = "Option::is_none")]
    pub single_select_clearable_user_picker_fields: Option<Vec<models::JiraSingleSelectUserPickerField>>,
    /// Add or clear a single select field:   *  To add, specify the option with an `optionId`.  *  To clear, pass an option with `optionId` as `-1`.
    #[serde(rename = "singleSelectFields", skip_serializing_if = "Option::is_none")]
    pub single_select_fields: Option<Vec<models::JiraSingleSelectField>>,
    /// Add or clear a single version picker field:   *  To add, specify the version with a `versionId`.  *  To clear, set `versionId` to `-1`.
    #[serde(rename = "singleVersionPickerFields", skip_serializing_if = "Option::is_none")]
    pub single_version_picker_fields: Option<Vec<models::JiraSingleVersionPickerField>>,
    /// Add or clear a URL field:   *  To add, provide the `url` with the desired URL value.  *  To clear, set `url` to an empty string.
    #[serde(rename = "urlFields", skip_serializing_if = "Option::is_none")]
    pub url_fields: Option<Vec<models::JiraUrlField>>,
}

impl JiraIssueFields {
    pub fn new() -> JiraIssueFields {
        JiraIssueFields {
            cascading_select_fields: None,
            clearable_number_fields: None,
            color_fields: None,
            date_picker_fields: None,
            date_time_picker_fields: None,
            issue_type: None,
            labels_fields: None,
            multiple_group_picker_fields: None,
            multiple_select_clearable_user_picker_fields: None,
            multiple_select_fields: None,
            multiple_version_picker_fields: None,
            multiselect_components: None,
            priority: None,
            rich_text_fields: None,
            single_group_picker_fields: None,
            single_line_text_fields: None,
            single_select_clearable_user_picker_fields: None,
            single_select_fields: None,
            single_version_picker_fields: None,
            url_fields: None,
        }
    }
}
