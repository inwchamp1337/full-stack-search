use std::env;

pub struct Config;

impl Config {
    pub fn get_backend_url() -> String {
        env::var("BACKEND_URL")
            .unwrap_or_else(|_| "http://localhost".to_string())
    }
    
    pub fn get_backend_port() -> String {
        env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "8000".to_string())
    }
    
    pub fn get_full_backend_url() -> String {
        let url = Self::get_backend_url();
        let port = Self::get_backend_port();
        format!("{}:{}", url, port)
    }
}
