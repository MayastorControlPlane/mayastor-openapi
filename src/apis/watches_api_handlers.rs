#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::{
    web::{self, Json, Path, Query},
    FromRequest,
};

/// Configure handlers for the WatchesApi resource
pub fn configure<T: crate::apis::WatchesApi + 'static, A: FromRequest + 'static>(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
       .service(
            actix_web::web::resource("/watches/volumes/{volume_id}")
                .name("del_watch_volume")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_watch_volume::<T, A>))
       )
       .service(
            actix_web::web::resource("/watches/volumes/{volume_id}")
                .name("get_watch_volume")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_watch_volume::<T, A>))
       )
       .service(
            actix_web::web::resource("/watches/volumes/{volume_id}")
                .name("put_watch_volume")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_watch_volume::<T, A>))
       );
}

async fn del_watch_volume<T: crate::apis::WatchesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(volume_id): web::Path<String>, web::Query(callback): web::Query<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_watch_volume(web::Path(volume_id), web::Query(callback)).await
}
async fn get_watch_volume<T: crate::apis::WatchesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(volume_id): web::Path<String>) -> Result<web::Json<Vec<crate::models::RestWatch>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_watch_volume(web::Path(volume_id)).await
}
async fn put_watch_volume<T: crate::apis::WatchesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(volume_id): web::Path<String>, web::Query(callback): web::Query<String>) -> Result<web::Json<serde_json::Value>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::put_watch_volume(web::Path(volume_id), web::Query(callback)).await
}
