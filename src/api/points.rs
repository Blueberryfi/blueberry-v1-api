use actix_web::{get, web, HttpResponse};
use alloy::primitives::{Uint, U160, U256};
use alloy::rpc::types::eth::BlockId;
use gql_client::GraphQLError;
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

/// Get all open positions
#[get("/points-etherfi/{block}/addresses={addresses}")]
pub async fn get_weeth_effective_balances(
    config: web::Data<GlobalConfig>,
    block: web::Path<u64>,
    addresses: web::Path<Vec<String>>,
) -> HttpResponse {
    let mut user_balances: Vec<UserBalance> = vec![];
    let block_id: BlockId = BlockId::from(block.to_be());

    // Get protocol balance
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

    // Get the next position ID
    let blueberry_bank = IBlueberryBank::new(BLUEBERRY_BANK, config.provider.clone());

    let next_position_id_call = blueberry_bank
        .getNextPositionId()
        .block(block_id)
        .call()
        .await;
    let next_position_id = match next_position_id_call {
        Ok(x) => x._0,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ethereum Call Error: {}", e))
        }
    };

    // Get the user balance
    for addr in addresses.iter() {
        let mut user_balance: Uint<256, 4> = "0".parse().unwrap();
        let position_data: Result<std::option::Option<PositionData>, GraphQLError> =
            query_user_positions(&config.ponder_client, addr.to_string().into()).await;

        let positions = match position_data {
            Ok(Some(x)) => x.positions,
            Ok(None) => {
                return HttpResponse::NoContent().finish();
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Internal Server Error: {}", e));
            }
        };

        for position in positions.items.into_iter() {
            if U256::from_str(&position.id).unwrap() >= next_position_id {
                break;
            }

            if position.collateralToken == WERC20.to_string() {
                if position.collateralId == get_werc20_id_from_token(WEETH.to_string()).to_string()
                {
                    let collateral_size_call = blueberry_bank
                        .getPositionInfo(U256::from_str(&position.id).unwrap())
                        .block(block_id)
                        .call()
                        .await;

                    let collateral_size: Uint<256, 4> = match collateral_size_call {
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
