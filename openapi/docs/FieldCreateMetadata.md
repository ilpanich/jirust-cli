# FieldCreateMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_values** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | The list of values allowed in the field. | [optional][readonly]
**auto_complete_url** | Option<**String**> | The URL that can be used to automatically complete the field. | [optional][readonly]
**configuration** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The configuration properties. | [optional][readonly]
**default_value** | Option<[**serde_json::Value**](.md)> | The default value of the field. | [optional][readonly]
**field_id** | **String** | The field id. | [readonly]
**has_default_value** | Option<**bool**> | Whether the field has a default value. | [optional][readonly]
**key** | **String** | The key of the field. | [readonly]
**name** | **String** | The name of the field. | [readonly]
**operations** | **Vec<String>** | The list of operations that can be performed on the field. | [readonly]
**required** | **bool** | Whether the field is required. | [readonly]
**schema** | [**models::JsonTypeBean**](JsonTypeBean.md) | The data type of the field. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


