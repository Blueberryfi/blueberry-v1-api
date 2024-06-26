use actix_cors::Cors;
use actix_web::http;
use alloy::providers::ProviderBuilder;
use dotenv::dotenv;
use gql_client::Client;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

use crate::models::config::GlobalConfig;

pub fn parse_env<T: FromStr>(variable: &str) -> T {
    dotenv().ok();
    match env::var(variable) {
        Ok(x) => match x.parse::<T>() {
            Ok(x) => x,
            Err(_) => panic!("Config Error: Invalid {}", variable),
        },
        Err(_) => panic!("Config Error: Invalid {}", variable),
    }
}

pub fn generate_server_address() -> String {
    dotenv().ok();
    match env::var("LOCAL_DEV") {
        Ok(x) => {
            println!("Local Dev: {}", x);
            if x == "true" {
                String::from("127.0.0.1")
            } else {
                String::from("0.0.0.0")
            }
        }
        Err(_) => panic!("Config Error: Invalid server address"),
    }
}

pub fn generate_cors() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".blueberry.garden"))
        .allowed_methods(vec!["GET"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}

pub fn generate_global_config() -> GlobalConfig {
    dotenv().ok();
    let rpc_url: String = parse_env("MAINNET_RPC_URL");
    let ponder_url: String = parse_env("PONDER_URL");
    let provider = Arc::new(ProviderBuilder::new().on_http(rpc_url.parse().unwrap()));

    GlobalConfig {
        provider,
        ponder_client: Client::new(ponder_url),
    }
}
