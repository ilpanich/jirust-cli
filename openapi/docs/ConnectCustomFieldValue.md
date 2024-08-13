# ConnectCustomFieldValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The type of custom field. | 
**field_id** | **i32** | The custom field ID. | 
**issue_id** | **i32** | The issue ID. | 
**number** | Option<**f64**> | The value of number type custom field when `_type` is `NumberIssueField`. | [optional]
**option_id** | Option<**String**> | The value of single select and multiselect custom field type when `_type` is `SingleSelectIssueField` or `MultiSelectIssueField`. | [optional]
**rich_text** | Option<**String**> | The value of richText type custom field when `_type` is `RichTextIssueField`. | [optional]
**string** | Option<**String**> | The value of string type custom field when `_type` is `StringIssueField`. | [optional]
**text** | Option<**String**> | The value of of text custom field type when `_type` is `TextIssueField`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


