use std::collections::HashMap;
use actix_web::{get, post, put, web, HttpResponse, Responder, delete, Result, Error};
use actix_web::error::{JsonPayloadError, ErrorBadRequest};
use actix_web::http::StatusCode;
use serde::de::Unexpected::Option;
use serde_json::{json, Value};
use crate::models::enumeration::Enumeration;
use crate::models::user::{Login, User};
use validator::Validate;
use std::fmt;
use crate::utils::{api_utils,error_utils::APIError};


#[post("/login")]
pub async fn login(data: web::Json<Login>) -> impl Responder {

    let mut errors: Vec<APIError> = Vec::new();
    if data.username.is_none() {
        errors.push(APIError { error: "username is missing or empty" });
    }else if data.password.is_none()  {
        errors.push(APIError { error: "password is missing or empty" });
    }
    if !errors.is_empty() {
        return HttpResponse::UnprocessableEntity().json(errors.get(0));
    }

    let response = api_utils::process_login(data.clone()).await;
    //let info = response.iter().enumerate();
    //for (index,(key, value)) in info{}
    println!("{:?}", response.get_key_value(&0));
    match response.iter().next() {
        Some((key, value)) => {
            if *key == 200 {
                let json_value: Value = serde_json::from_str(value).unwrap();
                HttpResponse::Ok().json(json!(json_value))
            } else {
                HttpResponse::Ok().json(json!({"error":value}))
            }
        }
        None => {
            HttpResponse::InternalServerError().json(json!({"error":""}))
        }
    }

}

#[post("/save-user")]
pub async fn save_user(data: web::Json<User>) -> impl Responder {
    // let mut save_user = data.clone().into_inner();
    // save_user.email = String::from("");

    let mut error: Vec<APIError> = Vec::new();
    let new_user = data.into_inner();
    if new_user.username.is_empty() {
        error.push(APIError { error: "username cannot be null" });
    } else if new_user.email.is_empty() {
        error.push(APIError { error: "email cannot be null" });
    }
    if !error.is_empty() {
        println!("Length: {:?}", error);
        return HttpResponse::UnprocessableEntity().json(error);
    }
    User::save_user(new_user.clone());
    HttpResponse::Created().json(&new_user)
}

