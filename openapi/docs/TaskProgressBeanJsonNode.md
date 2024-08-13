# TaskProgressBeanJsonNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the task. | [optional]
**elapsed_runtime** | **i64** | The execution time of the task, in milliseconds. | 
**finished** | Option<**i64**> | A timestamp recording when the task was finished. | [optional]
**id** | **String** | The ID of the task. | 
**last_update** | **i64** | A timestamp recording when the task progress was last updated. | 
**message** | Option<**String**> | Information about the progress of the task. | [optional]
**progress** | **i64** | The progress of the task, as a percentage complete. | 
**result** | Option<[**models::JsonNode**](JsonNode.md)> | The result of the task execution. | [optional]
**param_self** | **String** | The URL of the task. | 
**started** | Option<**i64**> | A timestamp recording when the task was started. | [optional]
**status** | **String** | The status of the task. | 
**submitted** | **i64** | A timestamp recording when the task was submitted. | 
**submitted_by** | **i64** | The ID of the user who submitted the task. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


