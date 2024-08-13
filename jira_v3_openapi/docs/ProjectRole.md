# ProjectRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actors** | Option<[**Vec<models::RoleActor>**](RoleActor.md)> | The list of users who act in this role. | [optional][readonly]
**admin** | Option<**bool**> | Whether this role is the admin role for the project. | [optional][readonly]
**current_user_role** | Option<**bool**> | Whether the calling user is part of this role. | [optional]
**default** | Option<**bool**> | Whether this role is the default role for the project | [optional][readonly]
**description** | Option<**String**> | The description of the project role. | [optional][readonly]
**id** | Option<**i64**> | The ID of the project role. | [optional][readonly]
**name** | Option<**String**> | The name of the project role. | [optional]
**role_configurable** | Option<**bool**> | Whether the roles are configurable for this project. | [optional][readonly]
**scope** | Option<[**models::Scope**](Scope.md)> | The scope of the role. Indicated for roles associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO). | [optional][readonly]
**param_self** | Option<**String**> | The URL the project role details. | [optional][readonly]
**translated_name** | Option<**String**> | The translated name of the project role. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


