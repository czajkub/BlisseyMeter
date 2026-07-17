const ALLOWED_PREFIX: &str = "https://replay.pokemonshowdown.com/";

pub async fn fetch_replay(replay_url: &str) -> Result<Vec<String>, String> {
    let url = replay_url.trim();
    if !url.starts_with(ALLOWED_PREFIX) {
        return Err(format!(
            "Invalid replay URL: only {ALLOWED_PREFIX}... links are supported"
        ));
    }

    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch URL: {e}"))?;

    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {e}"))?;
    
    Ok(content.split('\n').map(|s| s.to_string()).collect())
}
