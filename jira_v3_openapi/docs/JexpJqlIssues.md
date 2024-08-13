# JexpJqlIssues

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_results** | Option<**i32**> | The maximum number of issues to return from the JQL query. Inspect `meta.issues.jql.maxResults` in the response to ensure the maximum value has not been exceeded. | [optional]
**query** | Option<**String**> | The JQL query. | [optional]
**start_at** | Option<**i64**> | The index of the first issue to return from the JQL query. | [optional]
**validation** | Option<**String**> | Determines how to validate the JQL query and treat the validation results. | [optional][default to Strict]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


