# PositionDeleteFile

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **String** |  | 
**file_path** | **String** |  | 
**file_format** | [***models::FileFormat**](FileFormat.md) |  | 
**spec_id** | **i32** |  | 
**partition** | [**Vec<models::PrimitiveTypeValue>**](PrimitiveTypeValue.md) | A list of partition field values ordered based on the fields of the partition spec specified by the `spec-id` | [optional] [default to None]
**file_size_in_bytes** | **i64** | Total file size in bytes | 
**record_count** | **i64** | Number of records in the file | 
**key_metadata** | **String** | Binary type values are stored and serialized as an uppercase hexadecimal string | [optional] [default to None]
**split_offsets** | **Vec<i64>** | List of splittable offsets | [optional] [default to None]
**sort_order_id** | **i32** |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


