# CreatePrioritySchemeDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_priority_id** | **i64** | The ID of the default priority for the priority scheme. | 
**description** | Option<**String**> | The description of the priority scheme. | [optional]
**mappings** | Option<[**models::PriorityMapping**](PriorityMapping.md)> | Mappings of issue priorities for issues being migrated in and out of this priority scheme. | [optional]
**name** | **String** | The name of the priority scheme. Must be unique. | 
**priority_ids** | **Vec<i64>** | The IDs of priorities in the scheme. | 
**project_ids** | Option<**Vec<i64>**> | The IDs of projects that will use the priority scheme. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


