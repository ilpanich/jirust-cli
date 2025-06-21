# StatusPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the status | [optional]
**name** | Option<**String**> | The name of the status | [optional]
**on_conflict** | Option<**String**> | The conflict strategy for the status already exists. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters; NEW - Create a new entity | [optional]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**status_category** | Option<**String**> | The status category of the status. The value is case-sensitive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


