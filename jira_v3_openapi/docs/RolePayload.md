# RolePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_actors** | Option<[**Vec<models::ProjectCreateResourceIdentifier>**](ProjectCreateResourceIdentifier.md)> | The default actors for the role. By adding default actors, the role will be added to any future projects created | [optional]
**description** | Option<**String**> | The description of the role | [optional]
**name** | Option<**String**> | The name of the role | [optional]
**on_conflict** | Option<**String**> | The strategy to use when there is a conflict with an existing project role. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters | [optional][default to Use]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**r#type** | Option<**String**> | The type of the role. Only used by project-scoped project | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


