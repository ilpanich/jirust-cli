# \TeamsInPlanApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_atlassian_team**](TeamsInPlanApi.md#add_atlassian_team) | **POST** /rest/api/3/plans/plan/{planId}/team/atlassian | Add Atlassian team to plan
[**create_plan_only_team**](TeamsInPlanApi.md#create_plan_only_team) | **POST** /rest/api/3/plans/plan/{planId}/team/planonly | Create plan-only team
[**delete_plan_only_team**](TeamsInPlanApi.md#delete_plan_only_team) | **DELETE** /rest/api/3/plans/plan/{planId}/team/planonly/{planOnlyTeamId} | Delete plan-only team
[**get_atlassian_team**](TeamsInPlanApi.md#get_atlassian_team) | **GET** /rest/api/3/plans/plan/{planId}/team/atlassian/{atlassianTeamId} | Get Atlassian team in plan
[**get_plan_only_team**](TeamsInPlanApi.md#get_plan_only_team) | **GET** /rest/api/3/plans/plan/{planId}/team/planonly/{planOnlyTeamId} | Get plan-only team
[**get_teams**](TeamsInPlanApi.md#get_teams) | **GET** /rest/api/3/plans/plan/{planId}/team | Get teams in plan paginated
[**remove_atlassian_team**](TeamsInPlanApi.md#remove_atlassian_team) | **DELETE** /rest/api/3/plans/plan/{planId}/team/atlassian/{atlassianTeamId} | Remove Atlassian team from plan
[**update_atlassian_team**](TeamsInPlanApi.md#update_atlassian_team) | **PUT** /rest/api/3/plans/plan/{planId}/team/atlassian/{atlassianTeamId} | Update Atlassian team in plan
[**update_plan_only_team**](TeamsInPlanApi.md#update_plan_only_team) | **PUT** /rest/api/3/plans/plan/{planId}/team/planonly/{planOnlyTeamId} | Update plan-only team



## add_atlassian_team

> serde_json::Value add_atlassian_team(plan_id, add_atlassian_team_request)
Add Atlassian team to plan

Adds an existing Atlassian team to a plan and configures their plannning settings.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**add_atlassian_team_request** | [**AddAtlassianTeamRequest**](AddAtlassianTeamRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_plan_only_team

> i64 create_plan_only_team(plan_id, create_plan_only_team_request)
Create plan-only team

Creates a plan-only team and configures their planning settings.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**create_plan_only_team_request** | [**CreatePlanOnlyTeamRequest**](CreatePlanOnlyTeamRequest.md) |  | [required] |

### Return type

**i64**

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_plan_only_team

> serde_json::Value delete_plan_only_team(plan_id, plan_only_team_id)
Delete plan-only team

Deletes a plan-only team and their planning settings.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**plan_only_team_id** | **i64** | The ID of the plan-only team. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_atlassian_team

> models::GetAtlassianTeamResponse get_atlassian_team(plan_id, atlassian_team_id)
Get Atlassian team in plan

Returns planning settings for an Atlassian team in a plan.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**atlassian_team_id** | **String** | The ID of the Atlassian team. | [required] |

### Return type

[**models::GetAtlassianTeamResponse**](GetAtlassianTeamResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plan_only_team

> models::GetPlanOnlyTeamResponse get_plan_only_team(plan_id, plan_only_team_id)
Get plan-only team

Returns planning settings for a plan-only team.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**plan_only_team_id** | **i64** | The ID of the plan-only team. | [required] |

### Return type

[**models::GetPlanOnlyTeamResponse**](GetPlanOnlyTeamResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams

> models::PageWithCursorGetTeamResponseForPage get_teams(plan_id, cursor, max_results)
Get teams in plan paginated

Returns a [paginated](#pagination) list of plan-only and Atlassian teams in a plan.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**cursor** | Option<**String**> | The cursor to start from. If not provided, the first page will be returned. |  |[default to ]
**max_results** | Option<**i32**> | The maximum number of plan teams to return per page. The maximum value is 50. The default value is 50. |  |[default to 50]

### Return type

[**models::PageWithCursorGetTeamResponseForPage**](PageWithCursorGetTeamResponseForPage.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_atlassian_team

> serde_json::Value remove_atlassian_team(plan_id, atlassian_team_id)
Remove Atlassian team from plan

Removes an Atlassian team from a plan and deletes their planning settings.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**atlassian_team_id** | **String** | The ID of the Atlassian team. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_atlassian_team

> serde_json::Value update_atlassian_team(plan_id, atlassian_team_id, body)
Update Atlassian team in plan

Updates any of the following planning settings of an Atlassian team in a plan using [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902).   *  planningStyle  *  issueSourceId  *  sprintLength  *  capacity  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *Note that \"add\" operations do not respect array indexes in target locations. Call the \"Get Atlassian team in plan\" endpoint to find out the order of array elements.*

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**atlassian_team_id** | **String** | The ID of the Atlassian team. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plan_only_team

> serde_json::Value update_plan_only_team(plan_id, plan_only_team_id, body)
Update plan-only team

Updates any of the following planning settings of a plan-only team using [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902).   *  name  *  planningStyle  *  issueSourceId  *  sprintLength  *  capacity  *  memberAccountIds  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *Note that \"add\" operations do not respect array indexes in target locations. Call the \"Get plan-only team\" endpoint to find out the order of array elements.*

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**plan_only_team_id** | **i64** | The ID of the plan-only team. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

