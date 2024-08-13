# WebhookDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**events** | **Vec<String>** | The Jira events that trigger the webhook. | 
**field_ids_filter** | Option<**Vec<String>**> | A list of field IDs. When the issue changelog contains any of the fields, the webhook `jira:issue_updated` is sent. If this parameter is not present, the app is notified about all field updates. | [optional]
**issue_property_keys_filter** | Option<**Vec<String>**> | A list of issue property keys. A change of those issue properties triggers the `issue_property_set` or `issue_property_deleted` webhooks. If this parameter is not present, the app is notified about all issue property updates. | [optional]
**jql_filter** | **String** | The JQL filter that specifies which issues the webhook is sent for. Only a subset of JQL can be used. The supported elements are:   *  Fields: `issueKey`, `project`, `issuetype`, `status`, `assignee`, `reporter`, `issue.property`, and `cf[id]`. For custom fields (`cf[id]`), only the epic label custom field is supported.\".  *  Operators: `=`, `!=`, `IN`, and `NOT IN`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


