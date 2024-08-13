# IssueTypesWorkflowMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_mapping** | Option<**bool**> | Whether the workflow is the default workflow for the workflow scheme. | [optional]
**issue_types** | Option<**Vec<String>**> | The list of issue type IDs. | [optional]
**update_draft_if_needed** | Option<**bool**> | Whether a draft workflow scheme is created or updated when updating an active workflow scheme. The draft is updated with the new workflow-issue types mapping. Defaults to `false`. | [optional]
**workflow** | Option<**String**> | The name of the workflow. Optional if updating the workflow-issue types mapping. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


