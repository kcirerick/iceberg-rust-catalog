# ScanReport

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table_name** | **String** |  | 
**snapshot_id** | **i64** |  | 
**filter** | [***models::Expression**](Expression.md) |  | 
**schema_id** | **i32** |  | 
**projected_field_ids** | **Vec<i32>** |  | 
**projected_field_names** | **Vec<String>** |  | 
**metrics** | [**std::collections::HashMap<String, models::MetricResult>**](MetricResult.md) |  | 
**metadata** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


