# DetailedErrorCollection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**details** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Map of objects representing additional details for an error | [optional]
**error_messages** | Option<**Vec<String>**> | The list of error messages produced by this operation. For example, \"input parameter 'key' must be provided\" | [optional]
**errors** | Option<**std::collections::HashMap<String, String>**> | The list of errors by parameter returned by the operation. For example,\"projectKey\": \"Project keys must start with an uppercase letter, followed by one or more uppercase alphanumeric characters.\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


