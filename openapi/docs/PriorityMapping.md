# PriorityMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#in** | Option<**std::collections::HashMap<String, i64>**> | The mapping of priorities for issues being migrated **into** this priority scheme. Key is the old priority ID, value is the new priority ID (must exist in this priority scheme). | [optional]
**out** | Option<**std::collections::HashMap<String, i64>**> | The mapping of priorities for issues being migrated **out of** this priority scheme. Key is the old priority ID (must exist in this priority scheme), value is the new priority ID (must exist in the default priority scheme). Required for updating an existing priority scheme. Not used when creating a new priority scheme. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


