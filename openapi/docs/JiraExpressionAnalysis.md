# JiraExpressionAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**complexity** | Option<[**models::JiraExpressionComplexity**](JiraExpressionComplexity.md)> |  | [optional]
**errors** | Option<[**Vec<models::JiraExpressionValidationError>**](JiraExpressionValidationError.md)> | A list of validation errors. Not included if the expression is valid. | [optional]
**expression** | **String** | The analysed expression. | 
**r#type** | Option<**String**> | EXPERIMENTAL. The inferred type of the expression. | [optional]
**valid** | **bool** | Whether the expression is valid and the interpreter will evaluate it. Note that the expression may fail at runtime (for example, if it executes too many expensive operations). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


