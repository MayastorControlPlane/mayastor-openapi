#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::web::{self, Json, Path, Query};

#[async_trait::async_trait]
pub trait PoolsApi {
    async fn del_node_pool(web::Path((node_id, pool_id)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_pool(web::Path(pool_id): web::Path<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_pool(web::Path((node_id, pool_id)): web::Path<(String, String)>) -> Result<web::Json<crate::models::Pool>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_pools(web::Path(id): web::Path<String>) -> Result<web::Json<Vec<crate::models::Pool>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_pool(web::Path(pool_id): web::Path<String>) -> Result<web::Json<crate::models::Pool>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_pools() -> Result<web::Json<Vec<crate::models::Pool>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_node_pool(web::Path((node_id, pool_id)): web::Path<(String, String)>, web::Json(create_pool_body): web::Json<crate::models::CreatePoolBody>) -> Result<web::Json<crate::models::Pool>, crate::apis::RestError<crate::models::RestJsonError>>;
}
