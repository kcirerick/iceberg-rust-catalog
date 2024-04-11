# ListNamespacesResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_page_token** | **String** | An opaque token that allows clients to make use of pagination for list APIs (e.g. ListTables). Clients may initiate the first paginated request by sending an empty query parameter `pageToken` to the server. Servers that support pagination should identify the `pageToken` parameter and return a `next-page-token` in the response if there are more results available.  After the initial request, the value of `next-page-token` from each response must be used as the `pageToken` parameter value for the next request. The server must return `null` value for the `next-page-token` in the last response. Servers that support pagination must return all results in a single response with the value of `next-page-token` set to `null` if the query parameter `pageToken` is not set in the request. Servers that do not support pagination should ignore the `pageToken` parameter and return all results in a single response. The `next-page-token` must be omitted from the response. Clients must interpret either `null` or missing response value of `next-page-token` as the end of the listing results. | [optional] [default to None]
**namespaces** | [**Vec<models::Namespace>**](array.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


