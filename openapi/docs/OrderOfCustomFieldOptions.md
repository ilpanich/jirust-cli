# OrderOfCustomFieldOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**after** | Option<**String**> | The ID of the custom field option or cascading option to place the moved options after. Required if `position` isn't provided. | [optional]
**custom_field_option_ids** | **Vec<String>** | A list of IDs of custom field options to move. The order of the custom field option IDs in the list is the order they are given after the move. The list must contain custom field options or cascading options, but not both. | 
**position** | Option<**String**> | The position the custom field options should be moved to. Required if `after` isn't provided. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


