# FieldReferenceData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto** | Option<**String**> | Whether the field provide auto-complete suggestions. | [optional]
**cfid** | Option<**String**> | If the item is a custom field, the ID of the custom field. | [optional]
**deprecated** | Option<**String**> | Whether this field has been deprecated. | [optional]
**deprecated_searcher_key** | Option<**String**> | The searcher key of the field, only passed when the field is deprecated. | [optional]
**display_name** | Option<**String**> | The display name contains the following:   *  for system fields, the field name. For example, `Summary`.  *  for collapsed custom fields, the field name followed by a hyphen and then the field name and field type. For example, `Component - Component[Dropdown]`.  *  for other custom fields, the field name followed by a hyphen and then the custom field ID. For example, `Component - cf[10061]`. | [optional]
**operators** | Option<**Vec<String>**> | The valid search operators for the field. | [optional]
**orderable** | Option<**String**> | Whether the field can be used in a query's `ORDER BY` clause. | [optional]
**searchable** | Option<**String**> | Whether the content of this field can be searched. | [optional]
**types** | Option<**Vec<String>**> | The data types of items in the field. | [optional]
**value** | Option<**String**> | The field identifier. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


