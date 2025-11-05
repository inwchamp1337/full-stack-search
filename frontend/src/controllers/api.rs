use crate::models::*;
use crate::config::Config;

pub async fn submit_review(review: ReviewInput) -> Result<String, String> {
    let backend_url = Config::get_full_backend_url();
    let url = format!("{}/reviews", backend_url);

    let response = gloo_net::http::Request::post(&url)
        .json(&review)
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        Ok("Review submitted successfully".to_string())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(format!("Failed with status {}: {}", status, error_text))
    }
}

pub async fn search_reviews(query: String, k: usize) -> Result<SearchResponse, String> {
    let backend_url = Config::get_full_backend_url();
    let url = format!("{}/reviews/search", backend_url);

    let search_query = SearchQuery { query, k };

    let response = gloo_net::http::Request::post(&url)
        .json(&search_query)
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        response
            .json::<SearchResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(format!("Search failed with status {}: {}", status, error_text))
    }
}