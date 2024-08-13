# ComponentWithIssueCount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignee** | Option<[**models::User**](User.md)> | The details of the user associated with `assigneeType`, if any. See `realAssignee` for details of the user assigned to issues created with this component. | [optional]
**assignee_type** | Option<**String**> | The nominal user type used to determine the assignee for issues created with this component. See `realAssigneeType` for details on how the type of the user, and hence the user, assigned to issues is determined. Takes the following values:   *  `PROJECT_LEAD` the assignee to any issues created with this component is nominally the lead for the project the component is in.  *  `COMPONENT_LEAD` the assignee to any issues created with this component is nominally the lead for the component.  *  `UNASSIGNED` an assignee is not set for issues created with this component.  *  `PROJECT_DEFAULT` the assignee to any issues created with this component is nominally the default assignee for the project that the component is in. | [optional][readonly]
**description** | Option<**String**> | The description for the component. | [optional][readonly]
**id** | Option<**String**> | The unique identifier for the component. | [optional][readonly]
**is_assignee_type_valid** | Option<**bool**> | Whether a user is associated with `assigneeType`. For example, if the `assigneeType` is set to `COMPONENT_LEAD` but the component lead is not set, then `false` is returned. | [optional][readonly]
**issue_count** | Option<**i64**> | Count of issues for the component. | [optional][readonly]
**lead** | Option<[**models::User**](User.md)> | The user details for the component's lead user. | [optional]
**name** | Option<**String**> | The name for the component. | [optional][readonly]
**project** | Option<**String**> | The key of the project to which the component is assigned. | [optional][readonly]
**project_id** | Option<**i64**> | Not used. | [optional][readonly]
**real_assignee** | Option<[**models::User**](User.md)> | The user assigned to issues created with this component, when `assigneeType` does not identify a valid assignee. | [optional]
**real_assignee_type** | Option<**String**> | The type of the assignee that is assigned to issues created with this component, when an assignee cannot be set from the `assigneeType`. For example, `assigneeType` is set to `COMPONENT_LEAD` but no component lead is set. This property is set to one of the following values:   *  `PROJECT_LEAD` when `assigneeType` is `PROJECT_LEAD` and the project lead has permission to be assigned issues in the project that the component is in.  *  `COMPONENT_LEAD` when `assignee`Type is `COMPONENT_LEAD` and the component lead has permission to be assigned issues in the project that the component is in.  *  `UNASSIGNED` when `assigneeType` is `UNASSIGNED` and Jira is configured to allow unassigned issues.  *  `PROJECT_DEFAULT` when none of the preceding cases are true. | [optional][readonly]
**param_self** | Option<**String**> | The URL for this count of the issues contained in the component. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


