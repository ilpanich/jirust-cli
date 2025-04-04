# JiraStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the status. | [optional]
**id** | Option<**String**> | The ID of the status. | [optional]
**name** | Option<**String**> | The name of the status. | [optional]
**scope** | Option<[**models::StatusScope**](StatusScope.md)> |  | [optional]
**status_category** | Option<**String**> | The category of the status. | [optional]
**usages** | Option<[**Vec<models::ProjectIssueTypes>**](ProjectIssueTypes.md)> | Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  Projects and issue types where the status is used. Only available if the `usages` expand is requested. | [optional]
**workflow_usages** | Option<[**Vec<models::WorkflowUsages>**](WorkflowUsages.md)> | Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  The workflows that use this status. Only available if the `workflowUsages` expand is requested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


