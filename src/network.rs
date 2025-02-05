pub async fn check_internet_connection() -> bool {
    reqwest::get("https://www.google.com")
        .await
        .map(|resp| resp.status().is_success())
        .unwrap_or(false)
}
