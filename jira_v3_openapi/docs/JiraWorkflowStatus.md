# JiraWorkflowStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the status. | [optional]
**id** | Option<**String**> | The ID of the status. | [optional]
**name** | Option<**String**> | The name of the status. | [optional]
**scope** | Option<[**models::WorkflowScope**](WorkflowScope.md)> |  | [optional]
**status_category** | Option<**String**> | The category of the status. | [optional]
**status_reference** | Option<**String**> | The reference of the status. | [optional]
**usages** | Option<[**Vec<models::ProjectIssueTypes>**](ProjectIssueTypes.md)> | Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  The `statuses.usages` expand is an optional parameter that can be used when reading and updating statuses in Jira. It provides additional information about the projects and issue types associated with the requested statuses. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


