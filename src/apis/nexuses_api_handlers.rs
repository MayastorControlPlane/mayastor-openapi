#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::{
    web::{self, Json, Path, Query},
    FromRequest,
};

/// Configure handlers for the NexusesApi resource
pub fn configure<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
       .service(
            actix_web::web::resource("/nexuses/{nexus_id}")
                .name("del_nexus")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_nexus::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}")
                .name("del_node_nexus")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_node_nexus::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}/share")
                .name("del_node_nexus_shares")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_node_nexus_shares::<T, A>))
       )
       .service(
            actix_web::web::resource("/nexuses/{nexus_id}")
                .name("get_nexus")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_nexus::<T, A>))
       )
       .service(
            actix_web::web::resource("/nexuses")
                .name("get_nexuses")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_nexuses::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}")
                .name("get_node_nexus")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_nexus::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{id}/nexuses")
                .name("get_node_nexuses")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_nexuses::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}")
                .name("put_node_nexus")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_nexus::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}/share/{protocol}")
                .name("put_node_nexus_share")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_nexus_share::<T, A>))
       );
}

async fn del_nexus<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(nexus_id): web::Path<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_nexus(web::Path(nexus_id)).await
}
async fn del_node_nexus<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((node_id, nexus_id)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_node_nexus(web::Path((node_id, nexus_id))).await
}
async fn del_node_nexus_shares<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((node_id, nexus_id)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_node_nexus_shares(web::Path((node_id, nexus_id))).await
}
async fn get_nexus<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(nexus_id): web::Path<String>) -> Result<web::Json<crate::models::Nexus>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_nexus(web::Path(nexus_id)).await
}
async fn get_nexuses<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A) -> Result<web::Json<Vec<crate::models::Nexus>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_nexuses().await
}
async fn get_node_nexus<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((node_id, nexus_id)): web::Path<(String, String)>) -> Result<web::Json<crate::models::Nexus>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_node_nexus(web::Path((node_id, nexus_id))).await
}
async fn get_node_nexuses<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(id): web::Path<String>) -> Result<web::Json<Vec<crate::models::Nexus>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_node_nexuses(web::Path(id)).await
}
async fn put_node_nexus<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((node_id, nexus_id)): web::Path<(String, String)>, web::Json(create_nexus_body): web::Json<crate::models::CreateNexusBody>) -> Result<web::Json<crate::models::Nexus>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::put_node_nexus(web::Path((node_id, nexus_id)), web::Json(create_nexus_body)).await
}
async fn put_node_nexus_share<T: crate::apis::NexusesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((node_id, nexus_id, protocol)): web::Path<(String, String, String)>) -> Result<web::Json<String>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::put_node_nexus_share(web::Path((node_id, nexus_id, protocol))).await
}
