# IssueTypeScreenSchemeDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the issue type screen scheme. The maximum length is 255 characters. | [optional]
**issue_type_mappings** | [**Vec<models::IssueTypeScreenSchemeMapping>**](IssueTypeScreenSchemeMapping.md) | The IDs of the screen schemes for the issue type IDs and *default*. A *default* entry is required to create an issue type screen scheme, it defines the mapping for all issue types without a screen scheme. | 
**name** | **String** | The name of the issue type screen scheme. The name must be unique. The maximum length is 255 characters. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


