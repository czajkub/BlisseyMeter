mod fetch;
mod analyze;
mod handlers;
mod schema;
mod constants;

use std::env;
use std::sync::OnceLock;
use dotenv::dotenv;
use serde::Serialize;
use schema::state::LuckEvent;

use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::{StatusCode, header},
    Json
};
use lambda_http::{run, Error};
use tokio::net::TcpListener;

use fetch::fetch_replay;
use analyze::analyze;

static API_URL: OnceLock<String> = OnceLock::new();

fn get_api_url() -> &'static String {
    API_URL.get_or_init(|| {
        dotenv().ok();
        env::var("API_URL").expect("API_URL must be set")
    })
}

#[derive(Serialize)]
struct AnalyzeResponse {
    p1_luck_events: Vec<LuckEvent>,
    p2_luck_events: Vec<LuckEvent>,
}

async fn index() -> &'static str {
    "Hello, world!\n"
}

async fn analyze_replay(body: String) -> impl IntoResponse {
    let lines = fetch_replay(&body).await.unwrap_or_default();
    let game_state = analyze(lines).await;
    
    let response_data = AnalyzeResponse {
        p1_luck_events: game_state.p1.luck_events,
        p2_luck_events: game_state.p2.luck_events,
    };

    (
        StatusCode::OK,
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, get_api_url().as_str())],
        Json(response_data)
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let app = Router::new()
        .route("/", get(index))
        .route("/analyze", post(analyze_replay));

    let is_lambda = env::var("AWS_LAMBDA_RUNTIME_API").is_ok() || env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok();

    if is_lambda {
        run(app).await?;
    } else {
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let addr = format!("localhost:{}", port);
        println!("Running locally on http://{}", addr);
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    
    Ok(())
}