use actix_web::{web, Responder, HttpResponse };

pub async fn get_user_by_id(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let response_text = format!("get user by id handler, id: {}", id);
    HttpResponse::Ok().body(response_text)
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
    let scope = web::scope("/user");
    cfg.service(
        scope
            .route("", web::get().to(get_users))
            .route("/{id}", web::get().to(get_user_by_id))
            .route("/{id}", web::delete().to(delete_user_by_id))
    );
}

