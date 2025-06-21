# ScreenSchemePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_screen** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**description** | Option<**String**> | The description of the screen scheme | [optional]
**name** | Option<**String**> | The name of the screen scheme | [optional]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**screens** | Option<[**std::collections::HashMap<String, models::ProjectCreateResourceIdentifier>**](ProjectCreateResourceIdentifier.md)> | Similar to the field layout scheme those mappings allow users to set different screens for different operations: default - always there, applied to all operations that don't have an explicit mapping `create`, `view`, `edit` - specific operations that are available and users can assign a different screen for each one of them https://support.atlassian.com/jira-cloud-administration/docs/manage-screen-schemes/\\#Associating-a-screen-with-an-issue-operation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


