# SpecsVolumes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**labels** | **Vec<String>** | Volume labels. | 
**num_paths** | **i32** | Number of front-end paths. | 
**num_replicas** | **i32** | Number of children the volume should have. | 
**operation** | Option<[**crate::models::SpecsOperation3**](Specs_operation_3.md)> |  | [optional]
**protocol** | **String** | Protocol that the volume should be shared over. | 
**size** | **i64** | Size that the volume should be. | 
**state** | **String** | State that the volume should eventually achieve. | 
**target_node** | Option<**String**> | The node where front-end IO will be sent to | [optional]
**uuid** | **String** | Volume Id | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


