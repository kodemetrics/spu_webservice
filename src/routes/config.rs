use actix_web::web;
use actix_web::web::service;
use super::enumeration_routes::{save_enumeration};
use super::user_routes::{save_user, login};


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
        .service(save_enumeration)
        .service(save_user)
        .service(login);
        // .service(get_game_by_id)
        // .service(create_game)
        // .service(update_game)
        // .service(delete_game);
    conf.service(scope);
}