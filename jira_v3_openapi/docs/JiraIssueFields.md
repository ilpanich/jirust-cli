# JiraIssueFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cascading_select_fields** | Option<[**Vec<models::JiraCascadingSelectField>**](JiraCascadingSelectField.md)> | Add or clear a cascading select field:   *  To add, specify `optionId` for both parent and child.  *  To clear the child, set its `optionId` to null.  *  To clear both, set the parent's `optionId` to null. | [optional]
**clearable_number_fields** | Option<[**Vec<models::JiraNumberField>**](JiraNumberField.md)> | Add or clear a number field:   *  To add, specify a numeric `value`.  *  To clear, set `value` to `null`. | [optional]
**color_fields** | Option<[**Vec<models::JiraColorField>**](JiraColorField.md)> | Add or clear a color field:   *  To add, specify the color `name`. Available colors are: `purple`, `blue`, `green`, `teal`, `yellow`, `orange`, `grey`, `dark purple`, `dark blue`, `dark green`, `dark teal`, `dark yellow`, `dark orange`, `dark grey`.  *  To clear, set the color `name` to an empty string. | [optional]
**date_picker_fields** | Option<[**Vec<models::JiraDateField>**](JiraDateField.md)> | Add or clear a date picker field:   *  To add, specify the date in `d/mmm/yy` format or ISO format `dd-mm-yyyy`.  *  To clear, set `formattedDate` to an empty string. | [optional]
**date_time_picker_fields** | Option<[**Vec<models::JiraDateTimeField>**](JiraDateTimeField.md)> | Add or clear the planned start date and time:   *  To add, specify the date and time in ISO format for `formattedDateTime`.  *  To clear, provide an empty string for `formattedDateTime`. | [optional]
**issue_type** | Option<[**models::JiraIssueTypeField**](JiraIssueTypeField.md)> | Set the issue type field by providing an `issueTypeId`. | [optional]
**labels_fields** | Option<[**Vec<models::JiraLabelsField>**](JiraLabelsField.md)> | Edit a labels field:   *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.  *  To clear labels, use the `REMOVE_ALL` option with an empty `labels` array. | [optional]
**multiple_group_picker_fields** | Option<[**Vec<models::JiraMultipleGroupPickerField>**](JiraMultipleGroupPickerField.md)> | Add or clear a multi-group picker field:   *  To add groups, provide an array of groups with `groupName`s.  *  To clear all groups, use an empty `groups` array. | [optional]
**multiple_select_clearable_user_picker_fields** | Option<[**Vec<models::JiraMultipleSelectUserPickerField>**](JiraMultipleSelectUserPickerField.md)> | Assign or unassign multiple users to/from a field:   *  To assign, provide an array of user `accountId`s.  *  To clear, set `users` to `null`. | [optional]
**multiple_select_fields** | Option<[**Vec<models::JiraMultipleSelectField>**](JiraMultipleSelectField.md)> | Add or clear a multi-select field:   *  To add, provide an array of options with `optionId`s.  *  To clear, use an empty `options` array. | [optional]
**multiple_version_picker_fields** | Option<[**Vec<models::JiraMultipleVersionPickerField>**](JiraMultipleVersionPickerField.md)> | Edit a multi-version picker field like Fix Versions/Affects Versions:   *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.  *  To clear the field, use the `REMOVE_ALL` option with an empty `versions` array. | [optional]
**multiselect_components** | Option<[**models::JiraMultiSelectComponentField**](JiraMultiSelectComponentField.md)> | Edit a multi select components field:   *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.  *  To clear, use the `REMOVE_ALL` option with an empty `components` array. | [optional]
**original_estimate_field** | Option<[**models::JiraDurationField**](JiraDurationField.md)> | Edit the original estimate field. | [optional]
**priority** | Option<[**models::JiraPriorityField**](JiraPriorityField.md)> | Set the priority of an issue by specifying a `priorityId`. | [optional]
**rich_text_fields** | Option<[**Vec<models::JiraRichTextField>**](JiraRichTextField.md)> | Add or clear a rich text field:   *  To add, provide `adfValue`. Note that rich text fields only support ADF values.  *  To clear, use an empty `richText` object.  For ADF format details, refer to: [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure). | [optional]
**single_group_picker_fields** | Option<[**Vec<models::JiraSingleGroupPickerField>**](JiraSingleGroupPickerField.md)> | Add or clear a single group picker field:   *  To add, specify the group with `groupName`.  *  To clear, set `groupName` to an empty string. | [optional]
**single_line_text_fields** | Option<[**Vec<models::JiraSingleLineTextField>**](JiraSingleLineTextField.md)> | Add or clear a single line text field:   *  To add, provide the `text` value.  *  To clear, set `text` to an empty string. | [optional]
**single_select_clearable_user_picker_fields** | Option<[**Vec<models::JiraSingleSelectUserPickerField>**](JiraSingleSelectUserPickerField.md)> | Edit assignment for single select user picker fields like Assignee/Reporter:   *  To assign an issue, specify the user's `accountId`.  *  To unassign an issue, set `user` to `null`.  *  For automatic assignment, set `accountId` to `-1`. | [optional]
**single_select_fields** | Option<[**Vec<models::JiraSingleSelectField>**](JiraSingleSelectField.md)> | Add or clear a single select field:   *  To add, specify the option with an `optionId`.  *  To clear, pass an option with `optionId` as `-1`. | [optional]
**single_version_picker_fields** | Option<[**Vec<models::JiraSingleVersionPickerField>**](JiraSingleVersionPickerField.md)> | Add or clear a single version picker field:   *  To add, specify the version with a `versionId`.  *  To clear, set `versionId` to `-1`. | [optional]
**time_tracking_field** | Option<[**models::JiraTimeTrackingField**](JiraTimeTrackingField.md)> | Edit the time tracking field. | [optional]
**url_fields** | Option<[**Vec<models::JiraUrlField>**](JiraUrlField.md)> | Add or clear a URL field:   *  To add, provide the `url` with the desired URL value.  *  To clear, set `url` to an empty string. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


