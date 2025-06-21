# IssueTypeScreenSchemePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_screen_scheme** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**description** | Option<**String**> | The description of the issue type screen scheme | [optional]
**explicit_mappings** | Option<[**std::collections::HashMap<String, models::ProjectCreateResourceIdentifier>**](ProjectCreateResourceIdentifier.md)> | The IDs of the screen schemes for the issue type IDs and default. A default entry is required to create an issue type screen scheme, it defines the mapping for all issue types without a screen scheme. | [optional]
**name** | Option<**String**> | The name of the issue type screen scheme | [optional]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


