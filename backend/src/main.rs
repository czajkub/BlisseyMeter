mod fetch;
mod analyze;
mod handlers;
mod schema;
mod constants;

use std::env;
use dotenv::dotenv;
use serde::Serialize;
use schema::state::LuckEvent;

use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::StatusCode,
    Json
};
use tower_http::cors::{CorsLayer, Any};
use lambda_http::{run, Error};
use tokio::net::TcpListener;

use fetch::fetch_replay;
use analyze::analyze;

#[derive(Serialize)]
struct PlayerData {
    name: String,
    avatar: String,
    events: Vec<LuckEvent>,
}

#[derive(Serialize)]
struct AnalyzeResponse {
    p1: PlayerData,
    p2: PlayerData,
}

async fn index() -> &'static str {
    "Hello, world!\n"
}

async fn analyze_lines(lines: Vec<String>) -> AnalyzeResponse {
    let game_state = analyze(lines).await;
    AnalyzeResponse {
        p1: PlayerData {
            name: game_state.p1.name.clone(),
            avatar: game_state.p1.avatar.clone(),
            events: game_state.p1.luck_events,
        },
        p2: PlayerData {
            name: game_state.p2.name.clone(),
            avatar: game_state.p2.avatar.clone(),
            events: game_state.p2.luck_events,
        },
    }
}

async fn analyze_replay(body: String) -> impl IntoResponse {
    match fetch_replay(&body).await {
        Ok(lines) => (StatusCode::OK, Json(analyze_lines(lines).await)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

async fn analyze_raw(body: String) -> impl IntoResponse {
    let lines: Vec<String> = body.split('\n').map(|s| s.to_string()).collect();
    (StatusCode::OK, Json(analyze_lines(lines).await))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    
    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap_or(axum::http::HeaderValue::from_static("*")))
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/analyze", post(analyze_replay))
        .route("/analyze-raw", post(analyze_raw))
        .layer(cors);

    let is_lambda = env::var("AWS_LAMBDA_RUNTIME_API").is_ok() || env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok();

    if is_lambda {
        run(app).await?;
    } else {
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let addr = format!("127.0.0.1:{port}");
        println!("Running locally on http://{addr}");
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    
    Ok(())
}
