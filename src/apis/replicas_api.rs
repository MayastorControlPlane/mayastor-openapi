#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::web::{self, Json, Path, Query};

#[async_trait::async_trait]
pub trait ReplicasApi {
    async fn del_node_pool_replica(web::Path((node_id, pool_id, replica_id)): web::Path<(String, String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_node_pool_replica_shares(web::Path((node_id, pool_id, replica_id)): web::Path<(String, String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_pool_replica(web::Path((pool_id, replica_id)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_pool_replica_shares(web::Path((pool_id, replica_id)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_pool_replica(web::Path((node_id, pool_id, replica_id)): web::Path<(String, String, String)>) -> Result<web::Json<crate::models::Replica>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_pool_replicas(web::Path((node_id, pool_id)): web::Path<(String, String)>) -> Result<web::Json<Vec<crate::models::Replica>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_replicas(web::Path(id): web::Path<String>) -> Result<web::Json<Vec<crate::models::Replica>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_replica(web::Path(id): web::Path<String>) -> Result<web::Json<crate::models::Replica>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_replicas() -> Result<web::Json<Vec<crate::models::Replica>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_node_pool_replica(web::Path((node_id, pool_id, replica_id)): web::Path<(String, String, String)>, web::Json(create_replica_body): web::Json<crate::models::CreateReplicaBody>) -> Result<web::Json<crate::models::Replica>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_node_pool_replica_share(web::Path((node_id, pool_id, replica_id, protocol)): web::Path<(String, String, String, String)>) -> Result<web::Json<String>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_pool_replica(web::Path((pool_id, replica_id)): web::Path<(String, String)>, web::Json(create_replica_body): web::Json<crate::models::CreateReplicaBody>) -> Result<web::Json<crate::models::Replica>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_pool_replica_share(web::Path((pool_id, replica_id, protocol)): web::Path<(String, String, String)>) -> Result<web::Json<String>, crate::apis::RestError<crate::models::RestJsonError>>;
}
