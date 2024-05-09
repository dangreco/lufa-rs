use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LufaState(pub String, pub String, pub i32, pub LufaStateInfo);

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LufaStateInfo {
    pub user_email: String,
    pub first_name: String,
}
