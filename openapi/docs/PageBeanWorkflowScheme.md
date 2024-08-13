# PageBeanWorkflowScheme

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_last** | Option<**bool**> | Whether this is the last page. | [optional][readonly]
**max_results** | Option<**i32**> | The maximum number of items that could be returned. | [optional][readonly]
**next_page** | Option<**String**> | If there is another page of results, the URL of the next page. | [optional][readonly]
**param_self** | Option<**String**> | The URL of the page. | [optional][readonly]
**start_at** | Option<**i64**> | The index of the first item returned. | [optional][readonly]
**total** | Option<**i64**> | The number of items returned. | [optional][readonly]
**values** | Option<[**Vec<models::WorkflowScheme>**](WorkflowScheme.md)> | The list of items. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


