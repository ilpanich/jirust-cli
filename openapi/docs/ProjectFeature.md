# ProjectFeature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**feature** | Option<**String**> | The key of the feature. | [optional]
**image_uri** | Option<**String**> | URI for the image representing the feature. | [optional]
**localised_description** | Option<**String**> | Localized display description for the feature. | [optional]
**localised_name** | Option<**String**> | Localized display name for the feature. | [optional]
**prerequisites** | Option<**Vec<String>**> | List of keys of the features required to enable the feature. | [optional]
**project_id** | Option<**i64**> | The ID of the project. | [optional]
**state** | Option<**String**> | The state of the feature. When updating the state of a feature, only ENABLED and DISABLED are supported. Responses can contain all values | [optional]
**toggle_locked** | Option<**bool**> | Whether the state of the feature can be updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


