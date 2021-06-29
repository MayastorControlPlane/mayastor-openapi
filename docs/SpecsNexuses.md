# SpecsNexuses

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**children** | **Vec<String>** | List of children. | 
**managed** | **bool** | Managed by our control plane | 
**node** | **String** | Node where the nexus should live. | 
**operation** | Option<[**crate::models::SpecsOperation**](Specs_operation.md)> |  | [optional]
**owner** | Option<**String**> | Volume which owns this nexus, if any | [optional]
**share** | **String** | Share Protocol | 
**size** | **i64** | Size of the nexus. | 
**state** | **String** | The state the nexus should eventually reach. | 
**uuid** | **String** | Nexus Id | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


