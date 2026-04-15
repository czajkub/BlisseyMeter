mod fetch;
mod analyze;
mod handlers;
mod schema;
mod constants;

use std::env;
use std::sync::OnceLock;
use dotenv::dotenv;

use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::{StatusCode, header}
};
use lambda_http::{run, Error};

use fetch::fetch_replay;
use analyze::analyze;

static API_URL: OnceLock<String> = OnceLock::new();

fn get_api_url() -> &'static String {
    API_URL.get_or_init(|| {
        dotenv().ok();
        env::var("API_URL").expect("API_URL must be set")
    })
}

async fn index() -> &'static str {
    "Hello, world!\n"
}

async fn analyze_replay(body: String) -> impl IntoResponse {
    let lines = fetch_replay(&body).await.unwrap_or_default();
    analyze(lines).await;
    
    (
        StatusCode::OK,
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, get_api_url().as_str())],
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