use std::collections::HashMap;
use actix_web::{get, post, put, web, HttpResponse, Responder, delete, Result, Error};
use serde_json::{json, Value};
use crate::models::enumeration::Enumeration;
use crate::utils;


#[post("/save-enumeration")]
pub async fn save_enumeration(data: web::Json<Enumeration>) -> impl Responder {
    let new_enumeration = data.clone();
    let deviceID = Enumeration::save_enumeration(new_enumeration);
    if deviceID == 0 {
        HttpResponse::InternalServerError().json(json!({"deviceID":deviceID}));
    }
    HttpResponse::Created().json(data)
}


