# IssueTransition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expand** | Option<**String**> | Expand options that include additional transition details in the response. | [optional][readonly]
**fields** | Option<[**std::collections::HashMap<String, models::FieldMetadata>**](FieldMetadata.md)> | Details of the fields associated with the issue transition screen. Use this information to populate `fields` and `update` in a transition request. | [optional][readonly]
**has_screen** | Option<**bool**> | Whether there is a screen associated with the issue transition. | [optional][readonly]
**id** | Option<**String**> | The ID of the issue transition. Required when specifying a transition to undertake. | [optional]
**is_available** | Option<**bool**> | Whether the transition is available to be performed. | [optional][readonly]
**is_conditional** | Option<**bool**> | Whether the issue has to meet criteria before the issue transition is applied. | [optional][readonly]
**is_global** | Option<**bool**> | Whether the issue transition is global, that is, the transition is applied to issues regardless of their status. | [optional][readonly]
**is_initial** | Option<**bool**> | Whether this is the initial issue transition for the workflow. | [optional][readonly]
**looped** | Option<**bool**> |  | [optional]
**name** | Option<**String**> | The name of the issue transition. | [optional][readonly]
**to** | Option<[**models::StatusDetails**](StatusDetails.md)> | Details of the issue status after the transition. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


