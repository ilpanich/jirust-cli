# JiraWorkflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | The creation date of the workflow. | [optional]
**description** | Option<**String**> | The description of the workflow. | [optional]
**id** | Option<**String**> | The ID of the workflow. | [optional]
**is_editable** | Option<**bool**> | Indicates if the workflow can be edited. | [optional]
**name** | Option<**String**> | The name of the workflow. | [optional]
**scope** | Option<[**models::WorkflowScope**](WorkflowScope.md)> |  | [optional]
**start_point_layout** | Option<[**models::WorkflowLayout**](WorkflowLayout.md)> |  | [optional]
**statuses** | Option<[**Vec<models::WorkflowReferenceStatus>**](WorkflowReferenceStatus.md)> | The statuses referenced in this workflow. | [optional]
**task_id** | Option<**String**> | If there is a current [asynchronous task](#async-operations) operation for this workflow. | [optional]
**transitions** | Option<[**Vec<models::WorkflowTransitions>**](WorkflowTransitions.md)> | The transitions of the workflow. Note that a transition can have either the deprecated `to`/`from` fields or the `toStatusReference`/`links` fields, but never both nor a combination. | [optional]
**updated** | Option<**String**> | The last edited date of the workflow. | [optional]
**usages** | Option<[**Vec<models::ProjectIssueTypes>**](ProjectIssueTypes.md)> | Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  Use the optional `workflows.usages` expand to get additional information about the projects and issue types associated with the requested workflows. | [optional]
**version** | Option<[**models::DocumentVersion**](DocumentVersion.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


