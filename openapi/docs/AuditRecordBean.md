# AuditRecordBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**associated_items** | Option<[**Vec<models::AssociatedItemBean>**](AssociatedItemBean.md)> | The list of items associated with the changed record. | [optional][readonly]
**author_key** | Option<**String**> | Deprecated, use `authorAccountId` instead. The key of the user who created the audit record. | [optional][readonly]
**category** | Option<**String**> | The category of the audit record. For a list of these categories, see the help article [Auditing in Jira applications](https://confluence.atlassian.com/x/noXKM). | [optional][readonly]
**changed_values** | Option<[**Vec<models::ChangedValueBean>**](ChangedValueBean.md)> | The list of values changed in the record event. | [optional][readonly]
**created** | Option<**String**> | The date and time on which the audit record was created. | [optional][readonly]
**description** | Option<**String**> | The description of the audit record. | [optional][readonly]
**event_source** | Option<**String**> | The event the audit record originated from. | [optional][readonly]
**id** | Option<**i64**> | The ID of the audit record. | [optional][readonly]
**object_item** | Option<[**models::AssociatedItemBean**](AssociatedItemBean.md)> |  | [optional]
**remote_address** | Option<**String**> | The URL of the computer where the creation of the audit record was initiated. | [optional][readonly]
**summary** | Option<**String**> | The summary of the audit record. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


