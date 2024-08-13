# MandatoryFieldValueForAdf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retain** | Option<**bool**> | If `true`, will try to retain original non-null issue field values on move. | [optional][default to true]
**r#type** | **String** | Will treat as `MandatoryFieldValueForADF` if type is `adf` | [default to Raw]
**value** | [**serde_json::Value**](.md) | Value for each field. Accepts Atlassian Document Format (ADF) for rich text fields like `description`, `environments`. For ADF format details, refer to: [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


