mod error;
mod services;

use std::{collections::HashMap, fmt, sync::Arc};

use error::ReqwestSnafu;
use reqwest::{
    cookie::{CookieStore, Jar},
    Response,
};
use serde::Serialize;
use services::AuthService;
use snafu::IntoError;
use tokio::sync::RwLock;

pub use error::{Error, Result};

pub mod models {
    pub use lufa_models::*;
}

const DEFAULT_API_BASE_URL: &str = "https://montreal.lufa.com";

#[derive(Debug, Clone)]
pub enum Language {
    English,
    French,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::English => write!(f, "en"),
            Self::French => write!(f, "fr"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct State {
    user_id: String,
    email: String,
}

#[derive(Debug, Clone)]
pub struct Lufa {
    client: Arc<reqwest::Client>,
    jar: Arc<Jar>,
    state: Arc<RwLock<Option<State>>>,
    language: Language,
}

impl Lufa {
    pub fn new(language: Language) -> Result<Self> {
        let jar = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar.clone())
            .build()
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?;

        Ok(Self {
            client: Arc::new(client),
            jar,
            language,
            state: Arc::new(RwLock::new(None)),
        })
    }

    fn _cookies(&self) -> HashMap<String, String> {
        let cookie_str = match self.jar.cookies(&self._build_url("")) {
            Some(hv) => hv.to_str().unwrap().to_string(),
            None => String::default(),
        };

        let cookies: HashMap<String, String> = cookie_str
            .split(";")
            .map(|t| {
                let mut kv = t.trim().split("=").map(|kv| kv.to_string());
                (kv.next().unwrap(), kv.next().unwrap())
            })
            .collect();

        cookies
    }

    pub async fn is_logged_in(&self) -> bool {
        let has_state = self.state.read().await.is_some();

        let cookies = self._cookies();
        let has_cookie = cookies.contains_key("lufaState");

        return has_state && has_cookie;
    }

    fn _build_url(&self, path: &str) -> reqwest::Url {
        let base = format!("{}/{}", DEFAULT_API_BASE_URL, &self.language);
        let url = reqwest::Url::parse(base.as_str()).unwrap();
        url.join(path).unwrap()
    }

    pub(crate) async fn _get(&self, path: &str) -> Result<Response> {
        let res = self
            .client
            .get(self._build_url(path))
            .send()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?;

        Ok(res)
    }

    pub(crate) async fn _post<Body: Into<reqwest::Body>>(
        &self,
        path: &str,
        body: Body,
    ) -> Result<Response> {
        let res = self
            .client
            .post(self._build_url(path))
            .body(body)
            .send()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?;

        Ok(res)
    }

    pub(crate) async fn _post_json<Payload: Serialize>(
        &self,
        path: &str,
        payload: &Payload,
    ) -> Result<Response> {
        let res = self
            .client
            .post(self._build_url(path))
            .json(payload)
            .send()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?;

        Ok(res)
    }

    pub(crate) async fn _post_form<Form: Serialize>(
        &self,
        path: &str,
        form: &Form,
    ) -> Result<Response> {
        let res = self
            .client
            .post(self._build_url(path))
            .form(form)
            .send()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?;

        Ok(res)
    }

    pub fn auth(&self) -> AuthService {
        AuthService(self)
    }
}
