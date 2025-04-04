# \PlansApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_plan**](PlansApi.md#archive_plan) | **PUT** /rest/api/3/plans/plan/{planId}/archive | Archive plan
[**create_plan**](PlansApi.md#create_plan) | **POST** /rest/api/3/plans/plan | Create plan
[**duplicate_plan**](PlansApi.md#duplicate_plan) | **POST** /rest/api/3/plans/plan/{planId}/duplicate | Duplicate plan
[**get_plan**](PlansApi.md#get_plan) | **GET** /rest/api/3/plans/plan/{planId} | Get plan
[**get_plans**](PlansApi.md#get_plans) | **GET** /rest/api/3/plans/plan | Get plans paginated
[**trash_plan**](PlansApi.md#trash_plan) | **PUT** /rest/api/3/plans/plan/{planId}/trash | Trash plan
[**update_plan**](PlansApi.md#update_plan) | **PUT** /rest/api/3/plans/plan/{planId} | Update plan



## archive_plan

> serde_json::Value archive_plan(plan_id)
Archive plan

Archives a plan.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_plan

> i64 create_plan(create_plan_request, use_group_id)
Create plan

Creates a plan.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_plan_request** | [**CreatePlanRequest**](CreatePlanRequest.md) |  | [required] |
**use_group_id** | Option<**bool**> | Whether to accept group IDs instead of group names. Group names are deprecated. |  |[default to false]

### Return type

**i64**

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## duplicate_plan

> i64 duplicate_plan(plan_id, duplicate_plan_request)
Duplicate plan

Duplicates a plan.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**duplicate_plan_request** | [**DuplicatePlanRequest**](DuplicatePlanRequest.md) |  | [required] |

### Return type

**i64**

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plan

> models::GetPlanResponse get_plan(plan_id, use_group_id)
Get plan

Returns a plan.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**use_group_id** | Option<**bool**> | Whether to return group IDs instead of group names. Group names are deprecated. |  |[default to false]

### Return type

[**models::GetPlanResponse**](GetPlanResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plans

> models::PageWithCursorGetPlanResponseForPage get_plans(include_trashed, include_archived, cursor, max_results)
Get plans paginated

Returns a [paginated](#pagination) list of plans.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_trashed** | Option<**bool**> | Whether to include trashed plans in the results. |  |[default to false]
**include_archived** | Option<**bool**> | Whether to include archived plans in the results. |  |[default to false]
**cursor** | Option<**String**> | The cursor to start from. If not provided, the first page will be returned. |  |[default to ]
**max_results** | Option<**i32**> | The maximum number of plans to return per page. The maximum value is 50. The default value is 50. |  |[default to 50]

### Return type

[**models::PageWithCursorGetPlanResponseForPage**](PageWithCursorGetPlanResponseForPage.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trash_plan

> serde_json::Value trash_plan(plan_id)
Trash plan

Moves a plan to trash.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plan

> serde_json::Value update_plan(plan_id, body, use_group_id)
Update plan

Updates any of the following details of a plan using [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902).   *  name  *  leadAccountId  *  scheduling           *  estimation with StoryPoints, Days or Hours as possible values      *  startDate                   *  type with DueDate, TargetStartDate, TargetEndDate or DateCustomField as possible values          *  dateCustomFieldId      *  endDate                   *  type with DueDate, TargetStartDate, TargetEndDate or DateCustomField as possible values          *  dateCustomFieldId      *  inferredDates with None, SprintDates or ReleaseDates as possible values      *  dependencies with Sequential or Concurrent as possible values  *  issueSources           *  type with Board, Project or Filter as possible values      *  value  *  exclusionRules           *  numberOfDaysToShowCompletedIssues      *  issueIds      *  workStatusIds      *  workStatusCategoryIds      *  issueTypeIds      *  releaseIds  *  crossProjectReleases           *  name      *  releaseIds  *  customFields           *  customFieldId      *  filter  *  permissions           *  type with View or Edit as possible values      *  holder                   *  type with Group or AccountId as possible values          *  value  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *Note that \"add\" operations do not respect array indexes in target locations. Call the \"Get plan\" endpoint to find out the order of array elements.*

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i64** | The ID of the plan. | [required] |
**body** | **serde_json::Value** |  | [required] |
**use_group_id** | Option<**bool**> | Whether to accept group IDs instead of group names. Group names are deprecated. |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

