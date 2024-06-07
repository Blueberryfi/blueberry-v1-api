use actix_web::{get, web, HttpResponse};
use gql_client::{Client, GraphQLError};

use crate::models::{config::GlobalConfig, position::PositionData};

/// Get all open positionss
#[get("/positions")]
pub async fn get_open_positions(config: web::Data<GlobalConfig>) -> HttpResponse {
    let res: Result<Option<PositionData>, GraphQLError> =
        query_all_open_positions(&config.ponder_client).await;
    unwrap_position_query(res)
}

/// Get a specific position based on its id
#[get("/positions/{id}")]
pub async fn get_position(config: web::Data<GlobalConfig>, id: web::Path<String>) -> HttpResponse {
    let position_id: u32 = match id.parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    if position_id == 0 {
        return HttpResponse::NoContent().body("No Content: Invalid position Id.");
    }

    let res: Result<Option<PositionData>, GraphQLError> =
        query_position_id(&config.ponder_client, position_id).await;
    unwrap_position_query(res)
}

/// Get all open positions for a specific user
#[get("/positions/users/{user}")]
pub async fn get_users_positions(
    config: web::Data<GlobalConfig>,
    user: web::Path<String>,
) -> HttpResponse {
    let res: Result<Option<PositionData>, GraphQLError> =
        query_user_positions(&config.ponder_client, user).await;
    unwrap_position_query(res)
}

pub async fn query_all_open_positions(
    ponder_client: &Client,
) -> Result<Option<PositionData>, GraphQLError> {
    let query: &str = "
        query Positions {
            positions(where: {isOpen: true}) {
                items {
                    id
                    owner
                    underlyingToken
                    underlyingAmount
                    debtToken
                    debtShare
                    collateralId
                    collateralToken
                    collateralSize
                    isOpen
                }
            }
        }
    ";
    query_position(&ponder_client, &query).await
}

pub async fn query_position_id(
    ponder_client: &Client,
    id: u32,
) -> Result<Option<PositionData>, GraphQLError> {
    let query: String = format!(
        "
        query Positions {{
            positions(where: {{id: \"{}\"}}) {{
                items {{
                    id
                    owner
                    underlyingToken
                    underlyingAmount
                    debtToken
                    debtShare
                    collateralId
                    collateralToken
                    collateralSize
                    isOpen
                }}
            }}
        }}
        ",
        id
    );
    query_position(&ponder_client, &query).await
}

pub async fn query_user_positions(
    ponder_client: &Client,
    user: web::Path<String>,
) -> Result<Option<PositionData>, GraphQLError> {
    let query: String = format!(
        "
        query Positions {{
            positions(where: {{owner: \"{}\"}}) {{
                items {{
                    id
                    owner
                    underlyingToken
                    underlyingAmount
                    debtToken
                    debtShare
                    collateralId
                    collateralToken
                    collateralSize
                    isOpen
                }}
            }}
        }}
        ",
        user
    );
    query_position(&ponder_client, &query).await
}

/// Calls the database using a provided GraphQl query
async fn query_position(
    ponder_client: &Client,
    graph_ql_query: &str,
) -> Result<Option<PositionData>, GraphQLError> {
    ponder_client.query::<PositionData>(graph_ql_query).await
}

/// Formats data into a correct HTTP Response
fn unwrap_position_query(query_res: Result<Option<PositionData>, GraphQLError>) -> HttpResponse {
    match query_res {
        Ok(Some(response)) => HttpResponse::Ok()
            .content_type("application/json")
            .json(response),
        Ok(None) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", e)),
    }
}
