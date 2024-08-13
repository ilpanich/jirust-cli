# VersionRelatedWork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **String** | The category of the related work | 
**issue_id** | Option<**i64**> | The ID of the issue associated with the related work (if there is one). Cannot be updated via the Rest API. | [optional][readonly]
**related_work_id** | Option<**String**> | The id of the related work. For the native release note related work item, this will be null, and Rest API does not support updating it. | [optional][readonly]
**title** | Option<**String**> | The title of the related work | [optional]
**url** | Option<**String**> | The URL of the related work. Will be null for the native release note related work item, but is otherwise required. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


