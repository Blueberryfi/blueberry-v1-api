use actix_web::{get, web, HttpResponse};
use gql_client::Client;

use crate::models::position::PositionData;

/// TODO: Add detailed error handling

/// Get all open positionss
#[get("/positions")]
pub async fn get_open_positions() -> HttpResponse {
    let query: &str = "
        query Positions {
            positions {
                id
                owner
                underlyingToken
                underlyingAmount
                debtToken
                collateralId
                collateralToken
            }
        }
    ";

    let positions: PositionData = call_and_unwrap(query).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(positions)
}

/// Get a specific position based on its id
#[get("/positions/{id}")]
pub async fn get_position(id: web::Path<String>) -> HttpResponse {
    let query: String = format!(
        "
        query Positions {{
            positions(where: {{id: {}}}) {{
                id
                owner
                underlyingToken
                underlyingAmount
                debtToken
                collateralId
                collateralToken
            }}
        }}
        ",
        id
    );

    let positions: PositionData = call_and_unwrap(&query).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(positions)
}

/// Get all open positions for a specific user
#[get("/positions/user/{user}")]
pub async fn get_users_positions(user: web::Path<String>) -> HttpResponse {
    let query: String = format!(
        "
        query Positions {{
            positions(where: {{owner: \"{}\"}}) {{
                id
                owner
                underlyingToken
                underlyingAmount
                debtToken
                collateralId
                collateralToken
            }}
        }}
        ",
        user
    );

    let positions: PositionData = call_and_unwrap(&query).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(positions)
}

/// Calls the database using a provided GraphQl query and formats data that can be used
/// as an appropriate response.
async fn call_and_unwrap(graph_ql_query: &str) -> PositionData {
    let endpoint: &str = "https://blueberry-backend-mainnet.up.railway.app/";
    let client: Client = Client::new(endpoint);

    let response: Option<PositionData> =
        client.query::<PositionData>(graph_ql_query).await.unwrap();

    match response {
        Some(x) => x,
        None => panic!("Server Error"),
    }
}
