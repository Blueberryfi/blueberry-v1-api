use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub id: String,
    pub owner: String,
    pub underlyingToken: String,
    pub underlyingAmount: String,
    pub debtToken: String,
    pub debtShare: String,
    pub collateralId: String,
    pub collateralToken: String,
    pub collateralSize: String,
    pub isOpen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionItems {
    pub items: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionData {
    pub positions: PositionItems,
}
