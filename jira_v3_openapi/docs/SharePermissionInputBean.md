# SharePermissionInputBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The user account ID that the filter is shared with. For a request, specify the `accountId` property for the user. | [optional]
**group_id** | Option<**String**> | The ID of the group, which uniquely identifies the group across all Atlassian products.For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*. Cannot be provided with `groupname`. | [optional]
**groupname** | Option<**String**> | The name of the group to share the filter with. Set `type` to `group`. Please note that the name of a group is mutable, to reliably identify a group use `groupId`. | [optional]
**project_id** | Option<**String**> | The ID of the project to share the filter with. Set `type` to `project`. | [optional]
**project_role_id** | Option<**String**> | The ID of the project role to share the filter with. Set `type` to `projectRole` and the `projectId` for the project that the role is in. | [optional]
**rights** | Option<**i32**> | The rights for the share permission. | [optional]
**r#type** | **String** | The type of the share permission.Specify the type as follows:   *  `user` Share with a user.  *  `group` Share with a group. Specify `groupname` as well.  *  `project` Share with a project. Specify `projectId` as well.  *  `projectRole` Share with a project role in a project. Specify `projectId` and `projectRoleId` as well.  *  `global` Share globally, including anonymous users. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.  *  `authenticated` Share with all logged-in users. This shows as `loggedin` in the response. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


