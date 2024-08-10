# ReorderIssuePriorities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**after** | Option<**String**> | The ID of the priority. Required if `position` isn't provided. | [optional]
**ids** | **Vec<String>** | The list of issue IDs to be reordered. Cannot contain duplicates nor after ID. | 
**position** | Option<**String**> | The position for issue priorities to be moved to. Required if `after` isn't provided. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


