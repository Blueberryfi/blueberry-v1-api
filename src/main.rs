use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api::blb::get_blb_circulating_supply;

use crate::{
    api::positions::{get_open_positions, get_position, get_users_positions},
    models::config::GlobalConfig,
    utils::config::{generate_cors, generate_global_config, generate_server_address, parse_env},
};

mod api;
mod contracts;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address: String = generate_server_address();
    let port: u16 = parse_env("PORT");
    let global_config: GlobalConfig = generate_global_config();

    println!("Server starting on http://{}:{}", address, port);
    HttpServer::new(move || {
        let cors: Cors = generate_cors();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(global_config.clone()))
            .service(get_open_positions)
            .service(get_position)
            .service(get_users_positions)
            .service(get_blb_circulating_supply)
    })
    .bind((address, port))?
    .workers(4)
    .run()
    .await
}
