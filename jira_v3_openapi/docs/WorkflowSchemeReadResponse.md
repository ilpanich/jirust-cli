# WorkflowSchemeReadResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_workflow** | Option<[**models::WorkflowMetadataRestModel**](WorkflowMetadataRestModel.md)> |  | [optional]
**description** | Option<**String**> | The description of the workflow scheme. | [optional]
**id** | **String** | The ID of the workflow scheme. | 
**name** | **String** | The name of the workflow scheme. | 
**project_ids_using_scheme** | Option<**Vec<String>**> | Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  The IDs of projects using the workflow scheme. | [optional]
**scope** | [**models::WorkflowScope**](WorkflowScope.md) |  | 
**task_id** | Option<**String**> | Indicates if there's an [asynchronous task](#async-operations) for this workflow scheme. | [optional]
**version** | [**models::DocumentVersion**](DocumentVersion.md) |  | 
**workflows_for_issue_types** | [**Vec<models::WorkflowMetadataAndIssueTypeRestModel>**](WorkflowMetadataAndIssueTypeRestModel.md) | Mappings from workflows to issue types. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


