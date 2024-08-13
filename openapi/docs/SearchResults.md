# SearchResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expand** | Option<**String**> | Expand options that include additional search result details in the response. | [optional][readonly]
**issues** | Option<[**Vec<models::IssueBean>**](IssueBean.md)> | The list of issues found by the search. | [optional][readonly]
**max_results** | Option<**i32**> | The maximum number of results that could be on the page. | [optional][readonly]
**names** | Option<**std::collections::HashMap<String, String>**> | The ID and name of each field in the search results. | [optional][readonly]
**schema** | Option<[**std::collections::HashMap<String, models::JsonTypeBean>**](JsonTypeBean.md)> | The schema describing the field types in the search results. | [optional][readonly]
**start_at** | Option<**i32**> | The index of the first item returned on the page. | [optional][readonly]
**total** | Option<**i32**> | The number of results on the page. | [optional][readonly]
**warning_messages** | Option<**Vec<String>**> | Any warnings related to the JQL query. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


