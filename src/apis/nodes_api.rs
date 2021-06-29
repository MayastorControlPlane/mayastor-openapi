#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::web::{self, Json, Path, Query};

#[async_trait::async_trait]
pub trait NodesApi {
    async fn get_node(web::Path(id): web::Path<String>) -> Result<web::Json<crate::models::Node>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_nodes() -> Result<web::Json<Vec<crate::models::Node>>, crate::apis::RestError<crate::models::RestJsonError>>;
}
