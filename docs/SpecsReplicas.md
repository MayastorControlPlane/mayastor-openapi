# SpecsReplicas

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**managed** | **bool** | Managed by our control plane | 
**operation** | Option<[**crate::models::SpecsOperation2**](Specs_operation_2.md)> |  | [optional]
**owners** | [**crate::models::SpecsOwners**](Specs_owners.md) |  | 
**pool** | **String** | The pool that the replica should live on. | 
**share** | **String** | Protocol used for exposing the replica. | 
**size** | **i64** | The size that the replica should be. | 
**state** | **String** | The state that the replica should eventually achieve. | 
**thin** | **bool** | Thin provisioning. | 
**uuid** | **String** | uuid of the replica | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


