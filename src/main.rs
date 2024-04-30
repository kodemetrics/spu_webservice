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
mod utils;
//mod database;
//mod schema;

use std::sync::Mutex;
use models::enumeration::Enumeration;
use models::user::User;
use routes::{config::config};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Logger;

use utoipa_swagger_ui::SwaggerUi;

struct AppState {
    user_list: Mutex<Vec<User>>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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

    // struct ApiDoc;
    // let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            //.service(create_info)
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
