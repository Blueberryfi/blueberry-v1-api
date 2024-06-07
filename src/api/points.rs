use actix_web::{get, web, HttpResponse};
use alloy::primitives::{Uint, U160, U256};
use alloy::rpc::types::eth::BlockId;
use std::str::FromStr;

use crate::api::positions::query_user_positions;
use crate::contracts::addresses::{BLUEBERRY_BANK, BLUEBERRY_DEPLOYER_MULTISIG, WEETH, WERC20};
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
    let block_id: BlockId = BlockId::from(block.to_be());
    let weeth = IERC20::new(WEETH, config.provider.clone());
    let balance_call = weeth
        .balanceOf(BLUEBERRY_BANK)
        .block(block_id.clone())
        .call()
        .await;

    let protocol_balance: Uint<256, 4> = match balance_call {
        Ok(x) => x.balance,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Ethereum Call Error: {}", e))
        }
    };

    for addr in addresses.iter() {
        let mut user_balance: Uint<256, 4> = "0".parse().unwrap();
        let position_data: PositionData =
            query_user_positions(config.clone(), addr.to_string().into()).await;
        for position in position_data.positions.items.into_iter() {
            let blueberry_bank = IBlueberryBank::new(BLUEBERRY_BANK, config.provider.clone());
            if position.collateralToken == WERC20.to_string() {
                if position.collateralId == get_werc20_id_from_token(WEETH.to_string()).to_string()
                {
                    let collateral_size_call = blueberry_bank
                        .getPositionInfo(U256::from_str(&position.id).unwrap())
                        .block(block_id)
                        .call()
                        .await;

                    let collateral_size = match collateral_size_call {
                        Ok(x) => x._0.collateralSize,
                        Err(e) => {
                            return HttpResponse::InternalServerError()
                                .body(format!("Ethereum Call Error: {}", e))
                        }
                    };

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
            address: BLUEBERRY_DEPLOYER_MULTISIG.to_string(),
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
