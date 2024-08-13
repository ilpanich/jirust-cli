# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**events** | **Vec<String>** | The Jira events that trigger the webhook. | 
**expiration_date** | Option<**i64**> | The date after which the webhook is no longer sent. Use [Extend webhook life](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-webhooks/#api-rest-api-3-webhook-refresh-put) to extend the date. | [optional][readonly]
**field_ids_filter** | Option<**Vec<String>**> | A list of field IDs. When the issue changelog contains any of the fields, the webhook `jira:issue_updated` is sent. If this parameter is not present, the app is notified about all field updates. | [optional]
**id** | **i64** | The ID of the webhook. | 
**issue_property_keys_filter** | Option<**Vec<String>**> | A list of issue property keys. A change of those issue properties triggers the `issue_property_set` or `issue_property_deleted` webhooks. If this parameter is not present, the app is notified about all issue property updates. | [optional]
**jql_filter** | **String** | The JQL filter that specifies which issues the webhook is sent for. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


