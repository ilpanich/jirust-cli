# PermissionPayloadDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_addon_role** | Option<**bool**> | Configuration to generate addon role. Default is false if null. Only applies to GLOBAL-scoped permission scheme | [optional]
**description** | Option<**String**> | The description of the permission scheme | [optional]
**grants** | Option<[**Vec<models::PermissionGrantDto>**](PermissionGrantDTO.md)> | List of permission grants | [optional]
**name** | Option<**String**> | The name of the permission scheme | [optional]
**on_conflict** | Option<**String**> | The strategy to use when there is a conflict with an existing permission scheme. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters; NEW - If the entity exist, try and create a new one with a different name | [optional][default to Fail]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


