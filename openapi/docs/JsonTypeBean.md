# JsonTypeBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**configuration** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | If the field is a custom field, the configuration of the field. | [optional][readonly]
**custom** | Option<**String**> | If the field is a custom field, the URI of the field. | [optional][readonly]
**custom_id** | Option<**i64**> | If the field is a custom field, the custom ID of the field. | [optional][readonly]
**items** | Option<**String**> | When the data type is an array, the name of the field items within the array. | [optional][readonly]
**system** | Option<**String**> | If the field is a system field, the name of the field. | [optional][readonly]
**r#type** | **String** | The data type of the field. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


