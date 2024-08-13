# JiraExpressionForAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_variables** | Option<**std::collections::HashMap<String, String>**> | Context variables and their types. The type checker assumes that [common context variables](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/#context-variables), such as `issue` or `project`, are available in context and sets their type. Use this property to override the default types or provide details of new variables. | [optional]
**expressions** | **Vec<String>** | The list of Jira expressions to analyse. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


