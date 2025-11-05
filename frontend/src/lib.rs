use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const BACKEND_URL: &str = "http://localhost:8000";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReviewData {
    review_title: String,
    review_body: String,
    product_id: String,
    review_rating: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SearchRequest {
    query: String,
    k: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SearchResult {
    review_title: Option<String>,
    review_body: Option<String>,
    product_id: Option<String>,
    review_rating: Option<u8>,
    score: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="SPFresh Review System"/>
        <Style>
            {r#"
                * { margin: 0; padding: 0; box-sizing: border-box; }
                body { 
                    font-family: Arial, sans-serif; 
                    padding: 20px; 
                    max-width: 1200px; 
                    margin: 0 auto;
                    background: #f5f5f5;
                }
                .container { 
                    background: white; 
                    padding: 20px; 
                    margin-bottom: 20px; 
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }
                h1, h2 { margin-bottom: 15px; color: #333; }
                h1 { font-size: 24px; }
                h2 { font-size: 18px; color: #666; }
                .form-group { margin-bottom: 15px; }
                label { 
                    display: block; 
                    margin-bottom: 5px; 
                    font-weight: bold;
                    color: #555;
                }
                input, textarea { 
                    width: 100%; 
                    padding: 10px; 
                    border: 1px solid #ddd; 
                    border-radius: 4px;
                    font-size: 14px;
                }
                textarea { 
                    min-height: 100px; 
                    resize: vertical;
                    font-family: Arial, sans-serif;
                }
                button { 
                    background: #007bff; 
                    color: white; 
                    padding: 10px 20px; 
                    border: none; 
                    border-radius: 4px; 
                    cursor: pointer;
                    font-size: 14px;
                    font-weight: bold;
                }
                button:hover { background: #0056b3; }
                button:disabled { 
                    background: #ccc; 
                    cursor: not-allowed; 
                }
                .error { 
                    color: #dc3545; 
                    margin-top: 10px; 
                    padding: 10px;
                    background: #f8d7da;
                    border-radius: 4px;
                }
                .success { 
                    color: #28a745; 
                    margin-top: 10px; 
                    padding: 10px;
                    background: #d4edda;
                    border-radius: 4px;
                }
                .results { margin-top: 20px; }
                .result-item { 
                    background: #f8f9fa; 
                    padding: 15px; 
                    margin-bottom: 10px; 
                    border-left: 4px solid #007bff;
                    border-radius: 4px;
                }
                .result-item h3 { 
                    margin-bottom: 8px; 
                    color: #333;
                    font-size: 16px;
                }
                .result-item p { 
                    margin: 5px 0; 
                    color: #666;
                    font-size: 14px;
                }
                .result-item .meta { 
                    font-size: 12px; 
                    color: #999; 
                    margin-top: 8px;
                }
                .info { 
                    background: #e7f3ff; 
                    padding: 10px; 
                    border-radius: 4px; 
                    margin-bottom: 20px;
                    font-size: 14px;
                }
                .layout {
                    display: grid;
                    grid-template-columns: 1fr 500px;
                    gap: 20px;
                    align-items: start;
                }
                .compact-form { 
                    position: sticky;
                    top: 20px;
                }
                .compact-form .form-group {
                    margin-bottom: 10px;
                }
                .compact-form input,
                .compact-form textarea {
                    padding: 8px;
                    font-size: 13px;
                }
                .compact-form textarea {
                    min-height: 60px;
                }
                .compact-form label {
                    font-size: 13px;
                    margin-bottom: 3px;
                }
                .compact-form h2 {
                    font-size: 16px;
                    margin-bottom: 12px;
                }
                @media (max-width: 968px) {
                    .layout { 
                        grid-template-columns: 1fr;
                    }
                    .compact-form {
                        position: static;
                    }
                }
            "#}
        </Style>
        
        <h1>"SPFresh Review System"</h1>
        
        <div class="info">
            <strong>"Backend: "</strong> {BACKEND_URL} " | "
            <strong>"Frontend Port: "</strong> "3000 | "
            <strong>"Backend Port: "</strong> "8000"
        </div>

        <div class="layout">
            <SearchForm />
            <ReviewForm />
        </div>
    }
}

#[component]
fn ReviewForm() -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (body, set_body) = create_signal(String::new());
    let (product_id, set_product_id) = create_signal(String::new());
    let (rating, set_rating) = create_signal(5u8);
    let (message, set_message) = create_signal(None::<(bool, String)>);
    let (loading, set_loading) = create_signal(false);

    let submit_review = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_message.set(None);

        let review_data = ReviewData {
            review_title: title.get(),
            review_body: body.get(),
            product_id: product_id.get(),
            review_rating: rating.get(),
        };

        spawn_local(async move {
            match submit_review_request(review_data).await {
                Ok(_) => {
                    set_message.set(Some((true, "✓ Review submitted successfully!".to_string())));
                    set_title.set(String::new());
                    set_body.set(String::new());
                    set_product_id.set(String::new());
                    set_rating.set(5);
                }
                Err(e) => {
                    set_message.set(Some((false, format!("✗ Error: {}", e))));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="container compact-form">
            <h2>"Add Review"</h2>
            <form on:submit=submit_review>
                <div class="form-group">
                    <label for="title">"Review Title:"</label>
                    <input
                        type="text"
                        id="title"
                        required
                        prop:value=title
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="form-group">
                    <label for="body">"Review Body:"</label>
                    <textarea
                        id="body"
                        required
                        prop:value=body
                        on:input=move |ev| set_body.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="form-group">
                    <label for="product-id">"Product ID:"</label>
                    <input
                        type="text"
                        id="product-id"
                        required
                        prop:value=product_id
                        on:input=move |ev| set_product_id.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="form-group">
                    <label for="rating">"Rating (1-5):"</label>
                    <input
                        type="number"
                        id="rating"
                        min="1"
                        max="5"
                        required
                        prop:value=rating
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<u8>() {
                                set_rating.set(val);
                            }
                        }
                    />
                </div>
                
                <button type="submit" disabled=loading>
                    {move || if loading.get() { "Submitting..." } else { "Submit Review" }}
                </button>
            </form>
            
            {move || message.get().map(|(is_success, msg)| {
                let class = if is_success { "success" } else { "error" };
                view! {
                    <div class=class>{msg}</div>
                }
            })}
        </div>
    }
}

#[component]
fn SearchForm() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (k, set_k) = create_signal(3usize);
    let (message, set_message) = create_signal(None::<(bool, String)>);
    let (results, set_results) = create_signal(Vec::<SearchResult>::new());
    let (loading, set_loading) = create_signal(false);

    let submit_search = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_message.set(None);
        set_results.set(Vec::new());

        let search_data = SearchRequest {
            query: query.get(),
            k: k.get(),
        };

        spawn_local(async move {
            match search_reviews_request(search_data).await {
                Ok(response) => {
                    if !response.results.is_empty() {
                        set_message.set(Some((true, format!("✓ Found {} results", response.results.len()))));
                        set_results.set(response.results);
                    } else {
                        set_message.set(Some((false, "No results found".to_string())));
                    }
                }
                Err(e) => {
                    set_message.set(Some((false, format!("✗ Error: {}", e))));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div>
            <div class="container">
                <h2>"Search Reviews"</h2>
                <form on:submit=submit_search>
                <div class="form-group">
                    <label for="query">"Search Query:"</label>
                    <input
                        type="text"
                        id="query"
                        required
                        placeholder="e.g., SPFresh"
                        prop:value=query
                        on:input=move |ev| set_query.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="form-group">
                    <label for="k">"Number of Results (k):"</label>
                    <input
                        type="number"
                        id="k"
                        min="1"
                        required
                        prop:value=k
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                set_k.set(val);
                            }
                        }
                    />
                </div>
                
                <button type="submit" disabled=loading>
                    {move || if loading.get() { "Searching..." } else { "Search" }}
                </button>
            </form>
            
                {move || message.get().map(|(is_success, msg)| {
                let class = if is_success { "success" } else { "error" };
                view! {
                    <div class=class>{msg}</div>
                }
            })}
            </div>
        
            <Show when=move || !results.get().is_empty()>
                <div class="container results">
                <h2>"Search Results"</h2>
                <For
                    each=move || results.get().into_iter().enumerate()
                    key=|(idx, _)| *idx
                    children=|(idx, result)| {
                        let stars = "⭐".repeat(result.review_rating.unwrap_or(0) as usize);
                        view! {
                            <div class="result-item">
                                <h3>{idx + 1}". "{result.review_title.clone().unwrap_or_else(|| "Untitled".to_string())}</h3>
                                <p><strong>"Review: "</strong>{result.review_body.clone().unwrap_or_else(|| "No content".to_string())}</p>
                                <p><strong>"Rating: "</strong>{stars}" ("{result.review_rating.unwrap_or(0)}"/5)"</p>
                                <p class="meta">
                                    "Product ID: "{result.product_id.clone().unwrap_or_else(|| "N/A".to_string())}
                                </p>
                            </div>
                        }
                    }
                />
                </div>
            </Show>
        </div>
    }
}

async fn submit_review_request(review: ReviewData) -> Result<(), String> {
    let url = format!("{}/reviews", BACKEND_URL);
    
    let response = gloo_net::http::Request::post(&url)
        .json(&review)
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, text));
    }

    Ok(())
}

async fn search_reviews_request(search: SearchRequest) -> Result<SearchResponse, String> {
    let url = format!("{}/reviews/search", BACKEND_URL);
    
    let response = gloo_net::http::Request::post(&url)
        .json(&search)
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, text));
    }

    response
        .json::<SearchResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App /> });
}

