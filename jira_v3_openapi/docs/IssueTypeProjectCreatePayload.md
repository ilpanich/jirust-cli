# IssueTypeProjectCreatePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issue_type_hierarchy** | Option<[**Vec<models::IssueTypeHierarchyPayload>**](IssueTypeHierarchyPayload.md)> | Defines the issue type hierarhy to be created and used during this project creation. This will only add new levels if there isn't an existing level | [optional]
**issue_type_scheme** | Option<[**models::IssueTypeSchemePayload**](IssueTypeSchemePayload.md)> |  | [optional]
**issue_types** | Option<[**Vec<models::IssueTypePayload>**](IssueTypePayload.md)> | Only needed if you want to create issue types, you can otherwise use the ids of issue types in the scheme configuration | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


