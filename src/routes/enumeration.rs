use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use serde_json::json;
use crate::models::enumeration::Enumeration;
use crate::models::user::User;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct APIErrorResponse {
    error: String,
}

#[post("/save-enumeration")]
pub async fn save_enumeration(data: web::Json<Enumeration>) -> impl Responder {
    let deviceID = Enumeration::save_enumeration(data.into_inner());
    if deviceID == 0 {
        HttpResponse::InternalServerError().json(json!({"deviceID":deviceID}))
    }

    HttpResponse::Created().json(data)
}

#[post("/save-user")]
pub async fn save_user(data: web::Json<User>) -> impl Responder {
    let mut save_user = data.into_inner();
    save_user.email = String::from("");

    let mut error: Vec<APIErrorResponse> = Vec::new();
    let new_user = data.into_inner();
    if new_user.username.is_empty() {
        error.push(APIErrorResponse { error: "username cannot be null".to_string() });
    } else if new_user.email.is_empty() {
        error.push(APIErrorResponse { error: "email cannot be null".to_string() });
    }
    if !error.is_empty() {
        println!("Length: {:?}", error);
        return HttpResponse::UnprocessableEntity().json(error);
    }
    HttpResponse::Created().json(new_user)
}

