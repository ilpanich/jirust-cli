# JiraExpressionResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | Option<[**models::JiraExpressionEvaluationMetaDataBean**](JiraExpressionEvaluationMetaDataBean.md)> | Contains various characteristics of the performed expression evaluation. | [optional]
**value** | Option<[**serde_json::Value**](.md)> | The value of the evaluated expression. It may be a primitive JSON value or a Jira REST API object. (Some expressions do not produce any meaningful results—for example, an expression that returns a lambda function—if that's the case a simple string representation is returned. These string representations should not be relied upon and may change without notice.) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


