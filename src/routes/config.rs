use actix_web::web;
use actix_web::web::service;
use super::enumeration::{save_enumeration,save_user};


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
        .service(save_enumeration)
        .service(save_user);
        // .service(get_game_by_id)
        // .service(create_game)
        // .service(update_game)
        // .service(delete_game);
    conf.service(scope);
}