# BulkIssueResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issue_errors** | Option<[**Vec<models::IssueError>**](IssueError.md)> | When Jira can't return an issue enumerated in a request due to a retriable error or payload constraint, we'll return the respective issue ID with a corresponding error message. This list is empty when there are no errors Issues which aren't found or that the user doesn't have permission to view won't be returned in this list. | [optional][readonly]
**issues** | Option<[**Vec<models::IssueBean>**](IssueBean.md)> | The list of issues. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


