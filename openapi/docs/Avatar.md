# Avatar

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_name** | Option<**String**> | The file name of the avatar icon. Returned for system avatars. | [optional][readonly]
**id** | **String** | The ID of the avatar. | 
**is_deletable** | Option<**bool**> | Whether the avatar can be deleted. | [optional][readonly]
**is_selected** | Option<**bool**> | Whether the avatar is used in Jira. For example, shown as a project's avatar. | [optional][readonly]
**is_system_avatar** | Option<**bool**> | Whether the avatar is a system avatar. | [optional][readonly]
**owner** | Option<**String**> | The owner of the avatar. For a system avatar the owner is null (and nothing is returned). For non-system avatars this is the appropriate identifier, such as the ID for a project or the account ID for a user. | [optional][readonly]
**urls** | Option<**std::collections::HashMap<String, String>**> | The list of avatar icon URLs. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


