# UpdatePrioritySchemeRequestBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_priority_id** | Option<**i64**> | The default priority of the scheme. | [optional]
**description** | Option<**String**> | The description of the priority scheme. | [optional]
**mappings** | Option<[**models::PriorityMapping**](PriorityMapping.md)> | Instructions to migrate the priorities of issues.  `in` mappings are used to migrate the priorities of issues to priorities used within the priority scheme.  `out` mappings are used to migrate the priorities of issues to priorities not used within the priority scheme.   *  When **priorities** are **added** to the priority scheme, no mapping needs to be provided as the new priorities are not used by any issues.  *  When **priorities** are **removed** from the priority scheme, issues that are using those priorities must be migrated to new priorities used by the priority scheme.           *  An `in` mapping must be provided for each of these priorities.  *  When **projects** are **added** to the priority scheme, the priorities of issues in those projects might need to be migrated to new priorities used by the priority scheme. This can occur when the current scheme does not use all the priorities in the project(s)' priority scheme(s).           *  An `in` mapping must be provided for each of these priorities.  *  When **projects** are **removed** from the priority scheme, the priorities of issues in those projects might need to be migrated to new priorities within the **Default Priority Scheme** that are not used by the priority scheme. This can occur when the **Default Priority Scheme** does not use all the priorities within the current scheme.           *  An `out` mapping must be provided for each of these priorities.  For more information on `in` and `out` mappings, see the child properties documentation for the `PriorityMapping` object below. | [optional]
**name** | Option<**String**> | The name of the priority scheme. Must be unique. | [optional]
**priorities** | Option<[**models::UpdatePrioritiesInSchemeRequestBean**](UpdatePrioritiesInSchemeRequestBean.md)> | The priorities in the scheme. | [optional]
**projects** | Option<[**models::UpdateProjectsInSchemeRequestBean**](UpdateProjectsInSchemeRequestBean.md)> | The projects in the scheme. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


