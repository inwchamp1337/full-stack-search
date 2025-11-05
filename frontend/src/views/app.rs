use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use super::{SearchView, ReviewForm};
use crate::config::Config;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    let backend_url = Config::get_backend_url();
    let backend_port = Config::get_backend_port();
    let backend_full = format!("{}:{}", backend_url, backend_port);

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="SPFresh Review System"/>
        
        <Router>
            <main>
                <style>
                    "* { margin: 0; padding: 0; box-sizing: border-box; }
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
                    .two-col { 
                        display: grid; 
                        grid-template-columns: 1fr 1fr; 
                        gap: 20px; 
                    }
                    @media (max-width: 768px) {
                        .two-col { grid-template-columns: 1fr; }
                    }"
                </style>
                
                <Routes>
                    <Route path="" view=move || view! {
                        <h1>"SPFresh Review System"</h1>
                        
                        <div class="info">
                            <strong>"Backend: "</strong> {backend_full.clone()} " | "
                            <strong>"Frontend Port: "</strong> "3000 | "
                            <strong>"Backend Port: "</strong> {backend_port}
                        </div>

                        <div class="two-col">
                            <ReviewForm/>
                            <SearchView/>
                        </div>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}