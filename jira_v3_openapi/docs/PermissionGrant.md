# PermissionGrant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**holder** | Option<[**models::PermissionHolder**](PermissionHolder.md)> | The user or group being granted the permission. It consists of a `type`, a type-dependent `parameter` and a type-dependent `value`. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information. | [optional]
**id** | Option<**i64**> | The ID of the permission granted details. | [optional][readonly]
**permission** | Option<**String**> | The permission to grant. This permission can be one of the built-in permissions or a custom permission added by an app. See [Built-in permissions](../api-group-permission-schemes/#built-in-permissions) in *Get all permission schemes* for more information about the built-in permissions. See the [project permission](https://developer.atlassian.com/cloud/jira/platform/modules/project-permission/) and [global permission](https://developer.atlassian.com/cloud/jira/platform/modules/global-permission/) module documentation for more information about custom permissions. | [optional]
**param_self** | Option<**String**> | The URL of the permission granted details. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


