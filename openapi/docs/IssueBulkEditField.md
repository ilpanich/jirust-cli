# IssueBulkEditField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of the field. | [optional]
**field_options** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | A list of options related to the field, applicable in contexts where multiple selections are allowed. | [optional]
**id** | Option<**String**> | The unique ID of the field. | [optional]
**is_required** | Option<**bool**> | Indicates whether the field is mandatory for the operation. | [optional]
**multi_select_field_options** | Option<**Vec<String>**> | Specifies supported actions (like add, replace, remove) on multi-select fields via an enum. | [optional]
**name** | Option<**String**> | The display name of the field. | [optional]
**search_url** | Option<**String**> | A URL to fetch additional data for the field | [optional]
**r#type** | Option<**String**> | The type of the field. | [optional]
**unavailable_message** | Option<**String**> | A message indicating why the field is unavailable for editing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


