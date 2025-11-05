use leptos::*;
use crate::controllers::api::search_reviews;
use crate::models::search::SearchResponse;

#[component]
pub fn SearchView() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (k, set_k) = create_signal(3);
    let (results, set_results) = create_signal(Option::<SearchResponse>::None);
    let (message, set_message) = create_signal(None::<(bool, String)>);
    let (is_searching, set_is_searching) = create_signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        if query.get().trim().is_empty() {
            set_message.set(Some((false, "Please enter a search query".to_string())));
            return;
        }

        set_is_searching.set(true);
        set_message.set(None);
        set_results.set(None);

        let q = query.get();
        let k_val = k.get();

        spawn_local(async move {
            match search_reviews(q, k_val).await {
                Ok(response) => {
                    let count = response.results.len();
                    set_results.set(Some(response));
                    set_message.set(Some((true, format!("✓ Found {} results", count))));
                }
                Err(e) => {
                    set_message.set(Some((false, format!("✗ Error: {}", e))));
                }
            }
            set_is_searching.set(false);
        });
    };

    view! {
        <div class="container">
            <h2>"Search Reviews"</h2>
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="query">"Search Query:"</label>
                    <input
                        type="text"
                        id="query"
                        placeholder="e.g., SPFresh"
                        prop:value=move || query.get()
                        on:input=move |ev| set_query.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="k">"Number of Results (k):"</label>
                    <input
                        type="number"
                        id="k"
                        min="1"
                        prop:value=move || k.get()
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                set_k.set(val);
                            }
                        }
                        required
                    />
                </div>
                
                <button type="submit" disabled=move || is_searching.get()>
                    {move || if is_searching.get() { "Searching..." } else { "Search" }}
                </button>
            </form>
            
            {move || message.get().map(|(is_success, msg)| {
                let class = if is_success { "success" } else { "error" };
                view! { <div class=class>{msg}</div> }
            })}
        </div>

        <Show when=move || results.get().is_some()>
            <div class="container results">
                <h2>"Search Results"</h2>
                {move || results.get().map(|res| view! {
                    <div>
                        <For
                            each=move || res.results.clone()
                            key=|r| format!("{}-{}", r.product_id, r.score)
                            children=move |result| {
                                let stars = "⭐".repeat(result.review_rating.max(0).min(5) as usize);
                                view! {
                                    <div class="result-item">
                                        <h3>{result.review_title.clone()}</h3>
                                        <p><strong>"Review: "</strong> {result.review_body.clone()}</p>
                                        <p><strong>"Rating: "</strong> {stars} " (" {result.review_rating} "/5)"</p>
                                        <p class="meta">
                                            "Product ID: " {result.product_id.clone()} " | "
                                            "Score: " {format!("{:.4}", result.score)}
                                        </p>
                                    </div>
                                }
                            }
                        />
                    </div>
                })}
            </div>
        </Show>
    }
}