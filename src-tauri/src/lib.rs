mod fetch;
mod analysis;
pub mod handlers;


#[tauri::command]
async fn analyze(replay_url: &str) -> Result<String, String> {
    let lines = fetch::fetch_replay(replay_url)
        .await
        .unwrap_or_default();
    analysis::analysis(lines);
    Ok("ok".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![analyze])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
