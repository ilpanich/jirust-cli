# WorkflowCapabilities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connect_rules** | Option<[**Vec<models::AvailableWorkflowConnectRule>**](AvailableWorkflowConnectRule.md)> | The Connect provided ecosystem rules available. | [optional]
**editor_scope** | Option<**String**> | The scope of the workflow capabilities. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects. | [optional]
**forge_rules** | Option<[**Vec<models::AvailableWorkflowForgeRule>**](AvailableWorkflowForgeRule.md)> | The Forge provided ecosystem rules available. | [optional]
**project_types** | Option<**Vec<String>**> | The types of projects that this capability set is available for. | [optional]
**system_rules** | Option<[**Vec<models::AvailableWorkflowSystemRule>**](AvailableWorkflowSystemRule.md)> | The Atlassian provided system rules available. | [optional]
**trigger_rules** | Option<[**Vec<models::AvailableWorkflowTriggers>**](AvailableWorkflowTriggers.md)> | The trigger rules available. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


