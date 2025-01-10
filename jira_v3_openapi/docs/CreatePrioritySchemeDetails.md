# CreatePrioritySchemeDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_priority_id** | **i64** | The ID of the default priority for the priority scheme. | 
**description** | Option<**String**> | The description of the priority scheme. | [optional]
**mappings** | Option<[**models::PriorityMapping**](PriorityMapping.md)> | Instructions to migrate the priorities of issues.  `in` mappings are used to migrate the priorities of issues to priorities used within the priority scheme.  `out` mappings are used to migrate the priorities of issues to priorities not used within the priority scheme.   *  When **priorities** are **added** to the new priority scheme, no mapping needs to be provided as the new priorities are not used by any issues.  *  When **priorities** are **removed** from the new priority scheme, no mapping needs to be provided as the removed priorities are not used by any issues.  *  When **projects** are **added** to the priority scheme, the priorities of issues in those projects might need to be migrated to new priorities used by the priority scheme. This can occur when the current scheme does not use all the priorities in the project(s)' priority scheme(s).           *  An `in` mapping must be provided for each of these priorities.  *  When **projects** are **removed** from the priority scheme, no mapping needs to be provided as the removed projects are not using the priorities of the new priority scheme.  For more information on `in` and `out` mappings, see the child properties documentation for the `PriorityMapping` object below. | [optional]
**name** | **String** | The name of the priority scheme. Must be unique. | 
**priority_ids** | **Vec<i64>** | The IDs of priorities in the scheme. | 
**project_ids** | Option<**Vec<i64>**> | The IDs of projects that will use the priority scheme. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


