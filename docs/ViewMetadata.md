# ViewMetadata

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**view_uuid** | **String** |  | 
**format_version** | **u8** |  | 
**location** | **String** |  | 
**current_version_id** | **i32** |  | 
**versions** | [**Vec<models::ViewVersion>**](ViewVersion.md) |  | 
**version_log** | [**Vec<models::ViewHistoryEntry>**](ViewHistoryEntry.md) |  | 
**schemas** | [**Vec<models::Schema>**](Schema.md) |  | 
**properties** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


