use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Review {
    pub review_title: String,
    pub review_body: String,
    pub product_id: String,
    pub review_rating: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewInput {
    pub review_title: String,
    pub review_body: String,
    pub product_id: String,
    pub review_rating: i32,
}