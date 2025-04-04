# WorkflowSearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_last** | Option<**bool**> | Whether this is the last page. | [optional]
**max_results** | Option<**i32**> | The maximum number of items that could be returned. | [optional]
**next_page** | Option<**String**> | If there is another page of results, the URL of the next page. | [optional]
**param_self** | Option<**String**> | The URL of the page. | [optional]
**start_at** | Option<**i64**> | The index of the first item returned. | [optional]
**statuses** | Option<[**Vec<models::JiraWorkflowStatus>**](JiraWorkflowStatus.md)> | List of statuses. | [optional]
**total** | Option<**i64**> | The number of items returned. | [optional]
**values** | Option<[**Vec<models::JiraWorkflow>**](JiraWorkflow.md)> | List of workflows. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


