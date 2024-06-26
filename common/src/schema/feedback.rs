use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Feedback {
    pub id: uuid::Uuid,
    pub text: String,
    pub rating: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackData {
    pub feedback: Feedback,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackResponse {
    pub status: String,
    pub data: FeedbackData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackListResponse {
    pub status: String,
    pub results: i32,
    pub feedbacks: Vec<Feedback>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            status: "error".to_string(),
            message: message.to_string(),
        }
    }
}