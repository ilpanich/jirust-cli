# ParsedJqlQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<**Vec<String>**> | The list of syntax or validation errors. | [optional]
**query** | **String** | The JQL query that was parsed and validated. | 
**structure** | Option<[**models::JqlQuery**](JqlQuery.md)> | The syntax tree of the query. Empty if the query was invalid. | [optional]
**warnings** | Option<**Vec<String>**> | The list of warning messages | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


