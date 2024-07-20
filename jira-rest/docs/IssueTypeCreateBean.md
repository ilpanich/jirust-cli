# IssueTypeCreateBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the issue type. | [optional]
**hierarchy_level** | Option<**i32**> | The hierarchy level of the issue type. Use:   *  `-1` for Subtask.  *  `0` for Base.  Defaults to `0`. | [optional]
**name** | **String** | The unique name for the issue type. The maximum length is 60 characters. | 
**r#type** | Option<**String**> | Deprecated. Use `hierarchyLevel` instead. See the [deprecation notice](https://community.developer.atlassian.com/t/deprecation-of-the-epic-link-parent-link-and-other-related-fields-in-rest-apis-and-webhooks/54048) for details.  Whether the issue type is `subtype` or `standard`. Defaults to `standard`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


