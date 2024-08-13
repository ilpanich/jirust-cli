# ProjectIssueTypesHierarchyLevel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the issue type hierarchy level. This property is deprecated, see [Change notice: Removing hierarchy level IDs from next-gen APIs](https://developer.atlassian.com/cloud/jira/platform/change-notice-removing-hierarchy-level-ids-from-next-gen-apis/). | [optional][readonly]
**issue_types** | Option<[**Vec<models::IssueTypeInfo>**](IssueTypeInfo.md)> | The list of issue types in the hierarchy level. | [optional][readonly]
**level** | Option<**i32**> | The level of the issue type hierarchy level. | [optional][readonly]
**name** | Option<**String**> | The name of the issue type hierarchy level. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


