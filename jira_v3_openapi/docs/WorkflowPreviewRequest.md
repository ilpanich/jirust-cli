# WorkflowPreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issue_type_ids** | Option<**Vec<String>**> | The list of issue type IDs. At most 25 issue type IDs can be specified. | [optional]
**project_id** | **String** | The projectId parameter is required and will be used for permission checks. In addition, you must supply at least one of the following lookup terms: *workflowNames*, *workflowIds*, or *issueTypeIds*. The specified workflows must be associated with the given project. | 
**workflow_ids** | Option<**Vec<String>**> | The list of workflow IDs to be returned. At most 25 workflow IDs can be specified. | [optional]
**workflow_names** | Option<**Vec<String>**> | The list of workflow names to be returned. At most 25 workflow names can be specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


