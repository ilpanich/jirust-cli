# IssueLimitReportResponseBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issues_approaching_limit** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, i64>>**](std::collections::HashMap.md)> | A list of ids of issues approaching the limit and their field count | [optional]
**issues_breaching_limit** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, i64>>**](std::collections::HashMap.md)> | A list of ids of issues breaching the limit and their field count | [optional]
**limits** | Option<**std::collections::HashMap<String, i32>**> | The fields and their defined limits | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


