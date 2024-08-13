# IssueLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the issue link. | [optional][readonly]
**inward_issue** | [**models::LinkedIssue**](LinkedIssue.md) | Provides details about the linked issue. If presenting this link in a user interface, use the `inward` field of the issue link type to label the link. | 
**outward_issue** | [**models::LinkedIssue**](LinkedIssue.md) | Provides details about the linked issue. If presenting this link in a user interface, use the `outward` field of the issue link type to label the link. | 
**param_self** | Option<**String**> | The URL of the issue link. | [optional][readonly]
**r#type** | [**models::IssueLinkType**](IssueLinkType.md) | The type of link between the issues. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


