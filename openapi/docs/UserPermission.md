# UserPermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deprecated_key** | Option<**bool**> | Indicate whether the permission key is deprecated. Note that deprecated keys cannot be used in the `permissions parameter of Get my permissions. Deprecated keys are not returned by Get all permissions.` | [optional]
**description** | Option<**String**> | The description of the permission. | [optional]
**have_permission** | Option<**bool**> | Whether the permission is available to the user in the queried context. | [optional]
**id** | Option<**String**> | The ID of the permission. Either `id` or `key` must be specified. Use [Get all permissions](#api-rest-api-3-permissions-get) to get the list of permissions. | [optional]
**key** | Option<**String**> | The key of the permission. Either `id` or `key` must be specified. Use [Get all permissions](#api-rest-api-3-permissions-get) to get the list of permissions. | [optional]
**name** | Option<**String**> | The name of the permission. | [optional]
**r#type** | Option<**String**> | The type of the permission. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


