# FieldLayoutSchemePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_field_layout** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**description** | Option<**String**> | The description of the field layout scheme | [optional]
**explicit_mappings** | Option<[**std::collections::HashMap<String, models::ProjectCreateResourceIdentifier>**](ProjectCreateResourceIdentifier.md)> | There is a default configuration \"fieldlayout\" that is applied to all issue types using this scheme that don't have an explicit mapping users can create (or re-use existing) configurations for other issue types and map them to this scheme | [optional]
**name** | Option<**String**> | The name of the field layout scheme | [optional]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


