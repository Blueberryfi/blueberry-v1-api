use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    id: u128,
    owner: String,
    underlyingToken: String,
    underlyingAmount: String,
    debtToken: String,
    collateralId: String,
    collateralToken: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionData {
    positions: Vec<Position>,
}