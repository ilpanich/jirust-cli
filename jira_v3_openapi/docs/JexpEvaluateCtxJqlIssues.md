# JexpEvaluateCtxJqlIssues

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_results** | Option<**i32**> | The maximum number of issues to return from the JQL query. max results value considered may be lower than the number specific here. | [optional]
**next_page_token** | Option<**String**> | The token for a page to fetch that is not the first page. The first page has a `nextPageToken` of `null`. Use the `nextPageToken` to fetch the next page of issues. | [optional]
**query** | Option<**String**> | The JQL query, required to be bounded. Additionally, `orderBy` clause can contain a maximum of 7 fields | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


