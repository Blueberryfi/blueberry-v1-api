use std::str::FromStr;

use actix_web::{get, web, HttpResponse};
use alloy::primitives::{utils::format_units, Address, U256};

use crate::{
    contracts::{addresses::BLB_ADDRESS, erc20::IERC20},
    models::{blb::Blb, config::GlobalConfig, singleton_return::SingletonReturn},
};

/// Get all open positionss
#[get("/blb/circulating_supply")]
pub async fn get_blb_circulating_supply(config: web::Data<GlobalConfig>) -> HttpResponse {
    let blb: Blb = Blb::new();

    let burn_accounts: &Vec<Address> = &blb.non_circulating_accounts;
    let mut non_circulating_supply: U256 = U256::from_str("0").unwrap();

    let blb_instance = IERC20::new(BLB_ADDRESS, &config.provider);
    for account in burn_accounts.iter() {
        let balance_call = blb_instance.balanceOf(*account).call().await;

        let user_balance = match balance_call {
            Ok(x) => x.balance,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Ethereum Call Error: {}", e))
            }
        };
        non_circulating_supply += user_balance;
    }

    let circulating_supply: U256 = blb.total_supply - non_circulating_supply;

    let circulating_supply_scaled: String =
        match format_units(circulating_supply, blb.token_decimals) {
            Ok(x) => x,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Ethereum Call Error: {}", e))
            }
        };

    let res: SingletonReturn = SingletonReturn {
        result: circulating_supply_scaled,
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(res)
}

#[get("/blb/total_supply")]
pub async fn get_blb_total_supply() -> HttpResponse {
    let blb: Blb = Blb::new();

    let total_supply_scaled: String =
        match format_units(blb.total_supply, blb.token_decimals) {
            Ok(x) => x,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Ethereum Call Error: {}", e))
            }
        };

    let res: SingletonReturn = SingletonReturn {
        result: total_supply_scaled,
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(res)
}
