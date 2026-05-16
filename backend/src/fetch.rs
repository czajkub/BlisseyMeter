pub async fn fetch_replay(replay_url: &str) -> Result<Vec<String>, String> {
    let response = reqwest::get(replay_url)
        .await
        .map_err(|e| format!("Failed to fetch URL: {e}"))?;

    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {e}"))?;
    
    Ok(content.split('\n').map(|s| s.to_string()).collect())
}
