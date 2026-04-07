mod fetch;
mod analyze;
mod handlers;
mod schema;

use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::{StatusCode, header}
};
use lambda_http::{run, Error};


use fetch::fetch_replay;
use analyze::analyze;

async fn index() -> &'static str {
    "Hello, world!"
}

async fn analyze_replay(body: String) -> impl IntoResponse {
    let lines = fetch_replay(&body).await.unwrap_or_default();
    analyze(lines).await;
    
    (
        StatusCode::OK,
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://czajkub.pl")],
        "Analysis complete"
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new()
        .route("/", get(index))
        .route("/analyze", post(analyze_replay));
    run(app).await
}