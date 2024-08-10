# RemoteIssueLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application** | Option<[**models::Application**](Application.md)> | Details of the remote application the linked item is in. For example, trello. | [optional]
**global_id** | Option<**String**> | An identifier for the remote item in the remote system. For example, the global ID for a remote item in Confluence would consist of the app ID and page ID, like this: `appId=456&pageId=123`.  Setting this field enables the remote issue link details to be updated or deleted using remote system and item details as the record identifier, rather than using the record's Jira ID.  The maximum length is 255 characters. | [optional]
**object** | [**models::RemoteObject**](RemoteObject.md) | Details of the item linked to. | 
**relationship** | Option<**String**> | Description of the relationship between the issue and the linked item. If not set, the relationship description \"links to\" is used in Jira. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


