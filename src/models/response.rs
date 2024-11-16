use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub status: String,
    pub message: String,
    pub data: Option<T>, 
}

impl<T> ApiResponse<T> {
    pub fn new(code: i32, status: &str, message: &str, data: Option<T>) -> Self {
        ApiResponse {
            code,
            status: status.to_string(),
            message: message.to_string(),
            data,
        }
    }
}
