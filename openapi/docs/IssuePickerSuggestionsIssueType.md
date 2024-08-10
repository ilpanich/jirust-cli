# IssuePickerSuggestionsIssueType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the type of issues suggested for use in auto-completion. | [optional][readonly]
**issues** | Option<[**Vec<models::SuggestedIssue>**](SuggestedIssue.md)> | A list of issues suggested for use in auto-completion. | [optional][readonly]
**label** | Option<**String**> | The label of the type of issues suggested for use in auto-completion. | [optional][readonly]
**msg** | Option<**String**> | If no issue suggestions are found, returns a message indicating no suggestions were found, | [optional][readonly]
**sub** | Option<**String**> | If issue suggestions are found, returns a message indicating the number of issues suggestions found and returned. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


