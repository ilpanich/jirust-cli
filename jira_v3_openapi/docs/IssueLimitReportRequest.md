# IssueLimitReportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issues_approaching_limit_params** | Option<**std::collections::HashMap<String, i32>**> | A list of fields and their respective approaching limit threshold. Required for querying issues approaching limits. Optional for querying issues breaching limits. Accepted fields are: `comment`, `worklog`, `attachment`, `remoteIssueLinks`, and `issuelinks`. Example: `{\"issuesApproachingLimitParams\": {\"comment\": 4500, \"attachment\": 1800}}` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


