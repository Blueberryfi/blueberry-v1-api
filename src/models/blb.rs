use alloy::primitives::{Address, U256};

use crate::contracts::addresses::{
    BLB_ADDRESS, BLB_TREASURY, BLUEBERRY_STAKING, CHAINLINK_WALLET, MAGNA_CLAIM_CONTRACT, MAGNA_VESTING_CONTRACT,
};
const ONE_BILLION: u64 = 1_000_000_000;
const BLB_DECIMALS: u8 = 18;

pub struct Blb {
    pub address: Address,
    pub total_supply: U256,
    pub token_decimals: u8,
    pub non_circulating_accounts: Vec<Address>,
}

impl Blb {
    pub fn new() -> Blb {
        Blb {
            address: BLB_ADDRESS,
            total_supply: U256::from(ONE_BILLION) * U256::from(10).pow(U256::from(BLB_DECIMALS)),
            token_decimals: BLB_DECIMALS,
            non_circulating_accounts: vec![
                BLB_TREASURY,
                BLUEBERRY_STAKING,
                MAGNA_CLAIM_CONTRACT,
                MAGNA_VESTING_CONTRACT,
                CHAINLINK_WALLET,
            ],
        }
    }
}
