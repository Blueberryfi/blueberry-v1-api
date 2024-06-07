use actix_web::{get, web, HttpResponse};
use alloy::primitives::{Uint, U160, U256};
use alloy::rpc::types::eth::BlockId;
use std::str::FromStr;

use crate::api::positions::query_user_positions;
use crate::contracts::blueberry_bank::IBlueberryBank;
use crate::contracts::erc20::IERC20;
use crate::models::position::PositionData;
use crate::models::{
    config::GlobalConfig,
    points::{EtherfiPoints, UserBalance},
};

/// Get all open positionss
#[get("/points-etherfi/{block}/addresses={addresses}")]
pub async fn get_weeth_effective_balances(
    config: web::Data<GlobalConfig>,
    block: web::Path<u64>,
    addresses: web::Path<Vec<String>>,
) -> HttpResponse {
    let mut user_balances: Vec<UserBalance> = vec![];
    let weeth_addr: &str = "0xcd5fe23c85820f7b72d0926fc9b05b43e359b7ee";
    let blueberry_bank_addr: &str = "0xb96f3016DE8265612809c3d4FF2DEf5D4397F86d";
    let werc20: String = String::from("0x1a1f8ed1a73d62F08a74C0e7ebdF50937153e9E5");
    let weeth = IERC20::new(weeth_addr.parse().unwrap(), config.provider.clone());
    let blueberry_bank = IBlueberryBank::new(
        blueberry_bank_addr.parse().unwrap(),
        config.provider.clone(),
    );

    let block_id: BlockId = BlockId::from(block.to_be());
    // Get the balance of weETH in the Blueberry Bank
    let protocol_balance: Uint<256, 4> = weeth
        .balanceOf(
            "0xa34F59F634d48E2c3606048f2367326c46a4B5fA"
                .parse()
                .unwrap(),
        )
        .block(block_id.clone())
        .call()
        .await
        .unwrap()
        .balance;

    // For each address passed, get the weETH position size
    for addr in addresses.iter() {
        let mut user_balance: Uint<256, 4> = "0".parse().unwrap();
        let position_data: PositionData =
            query_user_positions(config.clone(), addr.to_string().into()).await;
        for position in position_data.positions.items.into_iter() {
            if position.collateralToken == werc20 {
                if position.collateralId == get_werc20_id_from_token(weeth_addr.into()).to_string()
                {
                    let collateral_size: Uint<256, 4> = blueberry_bank
                        .getPositionInfo(U256::from_str(&position.id).unwrap())
                        .block(block_id)
                        .call()
                        .await
                        .unwrap()
                        ._0
                        .collateralSize;

                    user_balance += collateral_size;
                }
            }
        }
        let _ = protocol_balance.checked_sub(user_balance);

        user_balances.push(UserBalance {
            address: addr.to_string(),
            effective_balance: user_balance.to_string(),
        });
    }

    if !protocol_balance.is_zero() {
        user_balances.push(UserBalance {
            address: String::from("0xE4D701c6E3bFbA3e50D1045A3cef4797b6165119"), // Blueberry Treasury
            effective_balance: protocol_balance.to_string(),
        });
    }

    let etherfi_balances: EtherfiPoints = EtherfiPoints {
        result: user_balances,
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(etherfi_balances)
}

fn get_werc20_id_from_token(token_addr: String) -> U256 {
    return U160::from_str(&token_addr).unwrap().to::<U256>();
}
