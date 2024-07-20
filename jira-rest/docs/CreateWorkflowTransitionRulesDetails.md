# CreateWorkflowTransitionRulesDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | Option<[**models::CreateWorkflowCondition**](CreateWorkflowCondition.md)> | The workflow conditions. | [optional]
**post_functions** | Option<[**Vec<models::CreateWorkflowTransitionRule>**](CreateWorkflowTransitionRule.md)> | The workflow post functions.  **Note:** The default post functions are always added to the *initial* transition, as in:      \"postFunctions\": [         {             \"type\": \"IssueCreateFunction\"         },         {             \"type\": \"IssueReindexFunction\"         },         {             \"type\": \"FireIssueEventFunction\",             \"configuration\": {                 \"event\": {                     \"id\": \"1\",                     \"name\": \"issue_created\"                 }             }         }     ]  **Note:** The default post functions are always added to the *global* and *directed* transitions, as in:      \"postFunctions\": [         {             \"type\": \"UpdateIssueStatusFunction\"         },         {             \"type\": \"CreateCommentFunction\"         },         {             \"type\": \"GenerateChangeHistoryFunction\"         },         {             \"type\": \"IssueReindexFunction\"         },         {             \"type\": \"FireIssueEventFunction\",             \"configuration\": {                 \"event\": {                     \"id\": \"13\",                     \"name\": \"issue_generic\"                 }             }         }     ] | [optional]
**validators** | Option<[**Vec<models::CreateWorkflowTransitionRule>**](CreateWorkflowTransitionRule.md)> | The workflow validators.  **Note:** The default permission validator is always added to the *initial* transition, as in:      \"validators\": [         {             \"type\": \"PermissionValidator\",             \"configuration\": {                 \"permissionKey\": \"CREATE_ISSUES\"             }         }     ] | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


