#![allow(
dead_code,
unused_crate_dependencies,
unused_variables,
unused_assignments,
unused,
non_snake_case,
non_camel_case_types,
warnings
)]

mod models;
mod routes;
//mod database;

use std::sync::Mutex;
use models::enumeration::Enumeration;
use models::user::User;
use routes::{config::config};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Logger;


struct AppState {
    user_list: Mutex<Vec<User>>,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// #[post("/save-user")]
// async fn create_info(data: web::Data<AppState>,payload: web::Json<User>) -> impl Responder {
//     // HttpResponse::Ok().body("Hey there!")
//     let mut userlist = data.user_list.lock().unwrap();
//     userlist.push(User{ id: "".to_string(), username: None, email: None });
//     HttpResponse::Created().json(userlist.to_vec())
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    println!("🚀 Server started successfully");

    let app_data = web::Data::new(AppState {
        user_list: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            //.service(create_info)
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
