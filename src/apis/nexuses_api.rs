#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::web::{self, Json, Path, Query};

#[async_trait::async_trait]
pub trait NexusesApi {
    async fn del_nexus(web::Path(nexus_id): web::Path<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_node_nexus(web::Path((node_id, nexus_id)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_node_nexus_shares(web::Path((node_id, nexus_id)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_nexus(web::Path(nexus_id): web::Path<String>) -> Result<web::Json<crate::models::Nexus>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_nexuses() -> Result<web::Json<Vec<crate::models::Nexus>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_nexus(web::Path((node_id, nexus_id)): web::Path<(String, String)>) -> Result<web::Json<crate::models::Nexus>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_nexuses(web::Path(id): web::Path<String>) -> Result<web::Json<Vec<crate::models::Nexus>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_node_nexus(web::Path((node_id, nexus_id)): web::Path<(String, String)>, web::Json(create_nexus_body): web::Json<crate::models::CreateNexusBody>) -> Result<web::Json<crate::models::Nexus>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_node_nexus_share(web::Path((node_id, nexus_id, protocol)): web::Path<(String, String, String)>) -> Result<web::Json<String>, crate::apis::RestError<crate::models::RestJsonError>>;
}
