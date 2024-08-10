# IssueUpdateDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fields** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | List of issue screen fields to update, specifying the sub-field to update and its value for each field. This field provides a straightforward option when setting a sub-field. When multiple sub-fields or other operations are required, use `update`. Fields included in here cannot be included in `update`. | [optional]
**history_metadata** | Option<[**models::HistoryMetadata**](HistoryMetadata.md)> | Additional issue history details. | [optional]
**properties** | Option<[**Vec<models::EntityProperty>**](EntityProperty.md)> | Details of issue properties to be add or update. | [optional]
**transition** | Option<[**models::IssueTransition**](IssueTransition.md)> | Details of a transition. Required when performing a transition, optional when creating or editing an issue. | [optional]
**update** | Option<[**std::collections::HashMap<String, Vec<models::FieldUpdateOperation>>**](Vec.md)> | A Map containing the field field name and a list of operations to perform on the issue screen field. Note that fields included in here cannot be included in `fields`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


