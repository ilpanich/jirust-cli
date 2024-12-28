# IssueBulkTransitionForWorkflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_transitions_filtered** | Option<**bool**> | Indicates whether all the transitions of this workflow are available in the transitions list or not. | [optional][readonly]
**issues** | Option<**Vec<String>**> | List of issue keys from the request which are associated with this workflow. | [optional][readonly]
**transitions** | Option<[**Vec<models::SimplifiedIssueTransition>**](SimplifiedIssueTransition.md)> | List of transitions available for issues from the request which are associated with this workflow.   **This list includes only those transitions that are common across the issues in this workflow and do not involve any additional field updates.**  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


