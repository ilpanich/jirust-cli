# RemoveOptionFromIssuesResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**models::SimpleErrorCollection**](SimpleErrorCollection.md)> | A collection of errors related to unchanged issues. The collection size is limited, which means not all errors may be returned. | [optional]
**modified_issues** | Option<**Vec<i64>**> | The IDs of the modified issues. | [optional]
**unmodified_issues** | Option<**Vec<i64>**> | The IDs of the unchanged issues, those issues where errors prevent modification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


