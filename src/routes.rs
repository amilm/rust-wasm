use crate::models::{User};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let users = User::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/user/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/user/{id}")]
async fn update(id: web::Path<i32>, user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let user = User::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/user/")]
async fn create(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
 let user = User::create(user.into_inner())?;
 Ok(HttpResponse::Ok().json(user))
}

#[delete("/user/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_user = User::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_user })))

}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find); 
    config.service(update);
    config.service(create);
    config.service(delete);

}

