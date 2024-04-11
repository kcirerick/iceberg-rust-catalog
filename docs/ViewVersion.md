# ViewVersion

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version_id** | **i32** |  | 
**timestamp_ms** | **i64** |  | 
**schema_id** | **i32** | Schema ID to set as current, or -1 to set last added schema | 
**summary** | **std::collections::HashMap<String, String>** |  | 
**representations** | [**Vec<models::ViewRepresentation>**](ViewRepresentation.md) |  | 
**default_catalog** | **String** |  | [optional] [default to None]
**default_namespace** | **Vec<String>** | Reference to one or more levels of a namespace | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


