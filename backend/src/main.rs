mod fetch;
mod analyze;
mod handlers;
mod schema;

use fetch::fetch_replay;
use analyze::analyze;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/analyze", data = "<replay_uri>")]
async fn analyze_replay(replay_uri: String) -> String {
    let lines = fetch_replay(&replay_uri).await.unwrap_or_default();
    analyze(lines).await;
    format!("Analysis complete")

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, analyze_replay])
}