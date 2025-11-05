use leptos::*;
use crate::controllers::api::submit_review;
use crate::models::review::ReviewInput;

#[component]
pub fn ReviewForm() -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (body, set_body) = create_signal(String::new());
    let (product_id, set_product_id) = create_signal(String::new());
    let (rating, set_rating) = create_signal(5);
    let (message, set_message) = create_signal(None::<(bool, String)>);
    let (is_submitting, set_is_submitting) = create_signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let review = ReviewInput {
            review_title: title.get(),
            review_body: body.get(),
            product_id: product_id.get(),
            review_rating: rating.get(),
        };

        set_is_submitting.set(true);
        set_message.set(None);

        spawn_local(async move {
            match submit_review(review).await {
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
            set_is_submitting.set(false);
        });
    };

    view! {
        <div class="container">
            <h2>"Add Review"</h2>
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="title">"Review Title:"</label>
                    <input
                        type="text"
                        id="title"
                        prop:value=move || title.get()
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="body">"Review Body:"</label>
                    <textarea
                        id="body"
                        prop:value=move || body.get()
                        on:input=move |ev| set_body.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="product-id">"Product ID:"</label>
                    <input
                        type="text"
                        id="product-id"
                        prop:value=move || product_id.get()
                        on:input=move |ev| set_product_id.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="rating">"Rating (1-5):"</label>
                    <input
                        type="number"
                        id="rating"
                        min="1"
                        max="5"
                        prop:value=move || rating.get()
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                set_rating.set(val);
                            }
                        }
                        required
                    />
                </div>
                
                <button type="submit" disabled=move || is_submitting.get()>
                    {move || if is_submitting.get() { "Submitting..." } else { "Submit Review" }}
                </button>
            </form>
            
            {move || message.get().map(|(is_success, msg)| {
                let class = if is_success { "success" } else { "error" };
                view! { <div class=class>{msg}</div> }
            })}
        </div>
    }
}
