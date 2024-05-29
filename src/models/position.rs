use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    id: String,
    owner: String,
    underlyingToken: String,
    underlyingAmount: String,
    debtToken: String,
    debtShare: String,
    collateralId: String,
    collateralToken: String,
    collateralSize: String,
    isOpen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionItems {
    items: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionData {
    positions: PositionItems,
}
