use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use crate::models::enumeration::Enumeration;
use crate::models::user::User;

#[post("/save-enumeration")]
pub async fn save_enumeration(data: web::Json<Enumeration>) -> impl Responder {
    HttpResponse::Created().json(data)
}

#[post("/save-user")]
pub async fn save_user(data: web::Json<User>) -> impl Responder {
    let mut error: Vec<&str> = Vec::new();
    if data.username.unwrap().is_empty() {
        error.push(&"username cannot be null".to_string());
    }else if data.email.unwrap().is_empty() {
        error.push(&"email cannot be null".to_string());
    }
    if error.len() > 0 {
        HttpResponse::UnprocessableEntity().body(&error[0]);
    }

    HttpResponse::Created().json(data)
}

