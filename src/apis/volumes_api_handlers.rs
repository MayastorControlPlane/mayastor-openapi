#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::{
    web::{self, Json, Path, Query},
    FromRequest,
};

/// Configure handlers for the VolumesApi resource
pub fn configure<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
       .service(
            actix_web::web::resource("/volumes{volume_id}/share")
                .name("del_shares")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_shares::<T, A>))
       )
       .service(
            actix_web::web::resource("/volumes/{volume_id}")
                .name("del_volume")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_volume::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{node_id}/volumes/{volume_id}")
                .name("get_node_volume")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_volume::<T, A>))
       )
       .service(
            actix_web::web::resource("/nodes/{node_id}/volumes")
                .name("get_node_volumes")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_volumes::<T, A>))
       )
       .service(
            actix_web::web::resource("/volumes/{volume_id}")
                .name("get_volume")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_volume::<T, A>))
       )
       .service(
            actix_web::web::resource("/volumes")
                .name("get_volumes")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_volumes::<T, A>))
       )
       .service(
            actix_web::web::resource("/volumes/{volume_id}")
                .name("put_volume")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_volume::<T, A>))
       )
       .service(
            actix_web::web::resource("/volumes/{volume_id}/share/{protocol}")
                .name("put_volume_share")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_volume_share::<T, A>))
       );
}

async fn del_shares<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(volume_id): web::Path<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_shares(web::Path(volume_id)).await
}
async fn del_volume<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(volume_id): web::Path<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_volume(web::Path(volume_id)).await
}
async fn get_node_volume<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((node_id, volume_id)): web::Path<(String, String)>) -> Result<web::Json<crate::models::Volume>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_node_volume(web::Path((node_id, volume_id))).await
}
async fn get_node_volumes<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(node_id): web::Path<String>) -> Result<web::Json<Vec<crate::models::Volume>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_node_volumes(web::Path(node_id)).await
}
async fn get_volume<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(volume_id): web::Path<String>) -> Result<web::Json<crate::models::Volume>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_volume(web::Path(volume_id)).await
}
async fn get_volumes<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A) -> Result<web::Json<Vec<crate::models::Volume>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_volumes().await
}
async fn put_volume<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(volume_id): web::Path<String>, web::Json(create_volume_body): web::Json<crate::models::CreateVolumeBody>) -> Result<web::Json<crate::models::Volume>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::put_volume(web::Path(volume_id), web::Json(create_volume_body)).await
}
async fn put_volume_share<T: crate::apis::VolumesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((volume_id, protocol)): web::Path<(String, String)>) -> Result<web::Json<String>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::put_volume_share(web::Path((volume_id, protocol))).await
}
