use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;

use crate::{WebsiteState, auth::auth_handler};

pub fn router() -> Router<WebsiteState> {
    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_methods(["POST".parse().unwrap()])
        .allow_headers(["content-type".parse().unwrap()])
        .allow_credentials(true);

    Router::new()
        .route("/", post(auth_handler::login))
        .layer(cors)
}
