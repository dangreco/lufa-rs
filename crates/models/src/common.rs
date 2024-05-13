use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PerUserForm {
    #[serde(rename = "user_id")]
    pub user_id: String,
}

#[derive(Debug, Serialize)]
pub struct PerOrderForm {
    #[serde(rename = "order_id")]
    pub order_id: String,
}
