# IssueTypePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatar_id** | Option<**i64**> | The avatar ID of the issue type. Go to https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-avatars/\\#api-rest-api-3-avatar-type-system-get to choose an avatarId existing in Jira | [optional]
**description** | Option<**String**> | The description of the issue type | [optional]
**hierarchy_level** | Option<**i32**> | The hierarchy level of the issue type. 0, 1, 2, 3 .. n; Negative values for subtasks | [optional]
**name** | Option<**String**> | The name of the issue type | [optional]
**on_conflict** | Option<**String**> | The conflict strategy to use when the issue type already exists. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters | [optional]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


