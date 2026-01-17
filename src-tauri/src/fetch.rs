pub async fn analyze(replay_url: &str) -> Result<String, String> {
    let response = reqwest::get(replay_url)
        .await
        .map_err(|e| format!("Failed to fetch URL: {}", e))?;

    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(content)
}
