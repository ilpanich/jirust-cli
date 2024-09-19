# SearchAndReconcileResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issues** | Option<[**Vec<models::IssueBean>**](IssueBean.md)> | The list of issues found by the search or reconsiliation. | [optional][readonly]
**names** | Option<**std::collections::HashMap<String, String>**> | The ID and name of each field in the search results. | [optional][readonly]
**next_page_token** | Option<**String**> | Continuation token to fetch the next page. If this result represents the last or the only page this token will be null. This token will expire in 7 days. | [optional][readonly]
**schema** | Option<[**std::collections::HashMap<String, models::JsonTypeBean>**](JsonTypeBean.md)> | The schema describing the field types in the search results. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


