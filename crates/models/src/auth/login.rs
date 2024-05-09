use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginReqForm {
    #[serde(rename = "LoginForm[user_email]")]
    pub email: String,
    #[serde(rename = "LoginForm[password]")]
    pub password: String,
}
