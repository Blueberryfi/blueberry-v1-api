use actix_web::{get, web, HttpResponse};
use gql_client::Client;

use crate::models::{config::GlobalConfig, position::PositionData};

/// Get all open positionss
#[get("/positions")]
pub async fn get_open_positions(config: web::Data<GlobalConfig>) -> HttpResponse {
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

    call_and_unwrap(&config.ponder_client, query).await
}

/// Get a specific position based on its id
#[get("/positions/{id}")]
pub async fn get_position(config: web::Data<GlobalConfig>, id: web::Path<String>) -> HttpResponse {
    let position_id: i32 = id.parse().unwrap_or(0);

    if position_id == 0 {
        return HttpResponse::NoContent().body("No Content: Invalid position Id.");
    }

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
        position_id
    );

    call_and_unwrap(&config.ponder_client, &query).await
}

/// Get all open positions for a specific user
#[get("/positions/users/{user}")]
pub async fn get_users_positions(
    config: web::Data<GlobalConfig>,
    user: web::Path<String>,
) -> HttpResponse {
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

    call_and_unwrap(&config.ponder_client, &query).await
}

/// Calls the database using a provided GraphQl query, formats the data and returns a
/// a valid HTTPResponse
async fn call_and_unwrap(ponder_client: &Client, graph_ql_query: &str) -> HttpResponse {
    match ponder_client.query::<PositionData>(graph_ql_query).await {
        Ok(Some(response)) => HttpResponse::Ok()
            .content_type("application/json")
            .json(response),
        Ok(None) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Query failed: {:?}", e);
            HttpResponse::InternalServerError().body("Internal Server Error: Query failed")
        }
    }
}
