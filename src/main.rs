use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

use crate::api::positions::{get_open_positions, get_position, get_users_positions};

mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let address: String = match env::var("LOCAL_DEV") {
        Ok(x) => {
            println!("Local Dev: {}", x);
            if x == "true" {
                String::from("127.0.0.1")
            } else {
                match env::var("SERVER_ADDRESS") {
                    Ok(x) => x,
                    Err(_) => panic!("Config Error: Invalid Endpoint Url"),
                }
            }
        }
        Err(_) => panic!("Config Error: Invalid Endpoint Url"),
    };

    let port: u16 = match env::var("PORT") {
        Ok(x) => x
            .parse()
            .expect("Config Error: Failed to parse port number"),
        Err(_) => panic!("Config Error: Invalid Port Number"),
    };

    println!("Server starting on http://{}:{}", address, port);
    HttpServer::new(|| {
        App::new()
            .service(get_open_positions)
            .service(get_position)
            .service(get_users_positions)
    })
    .bind((address, port))?
    .workers(4)
    .run()
    .await
}
