# DeleteAndReplaceVersionBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_field_replacement_list** | Option<[**Vec<models::CustomFieldReplacement>**](CustomFieldReplacement.md)> | An array of custom field IDs (`customFieldId`) and version IDs (`moveTo`) to update when the fields contain the deleted version. | [optional]
**move_affected_issues_to** | Option<**i64**> | The ID of the version to update `affectedVersion` to when the field contains the deleted version. | [optional]
**move_fix_issues_to** | Option<**i64**> | The ID of the version to update `fixVersion` to when the field contains the deleted version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


