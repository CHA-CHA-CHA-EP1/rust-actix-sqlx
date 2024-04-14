use actix_web::{web, Responder, HttpResponse };

use crate::services::user_service::UserService;

pub async fn get_user_by_id(
    service: web::Data<crate::domain::user::AppState>,
    path: web::Path<i32>,
) -> impl Responder {

    let id = path.into_inner();
    let name = service.user_service.get_user_by_id(id).await.unwrap();

    HttpResponse::Ok().body(format!("get user by id handler, id: {}, name: {}", id, name))
}

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("get users handler")
}

pub async fn delete_user_by_id(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let response_text = format!("delete user by id handler, id: {}", id);
    HttpResponse::Ok().body(response_text)
}
 
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/{id}", web::get().to(get_user_by_id))
        .route("/", web::get().to(get_users))
        .route("/{id}", web::delete().to(delete_user_by_id));
}

