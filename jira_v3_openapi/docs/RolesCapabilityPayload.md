# RolesCapabilityPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role_to_project_actors** | Option<[**std::collections::HashMap<String, Vec<models::ProjectCreateResourceIdentifier>>**](Vec.md)> | A map of role PCRI (can be ID or REF) to a list of user or group PCRI IDs to associate with the role and project. | [optional]
**roles** | Option<[**Vec<models::RolePayload>**](RolePayload.md)> | The list of roles to create. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


