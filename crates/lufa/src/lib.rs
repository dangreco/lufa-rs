mod error;
mod language;
mod services;

use std::{collections::HashMap, sync::Arc};

use reqwest::{
    cookie::{CookieStore, Jar},
    Response, Url,
};
use serde::Serialize;
use snafu::IntoError;
use tokio::sync::RwLock;

pub use error::*;
pub use language::*;
pub use services::*;

pub mod models {
    pub use lufa_models::*;
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_API_BASE_URL: &str = "https://montreal.lufa.com";

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
        let cookie_str = match self.jar.cookies(&self._build_url("").unwrap()) {
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

    pub(crate) async fn guard_logged_in(&self) -> Result<()> {
        match self.is_logged_in().await {
            true => Ok(()),
            false => Err(LufaSnafu.into_error(LufaError {
                message: format!("not logged in"),
            })),
        }
    }

    pub(crate) async fn is_logged_in(&self) -> bool {
        let has_state = self.state.read().await.is_some();

        let cookies = self._cookies();
        let has_cookie = cookies.contains_key("lufaState");

        return has_state && has_cookie;
    }

    pub(crate) async fn user_id(&self) -> Result<String> {
        let state = self.state.read().await;
        let user_id = state
            .clone()
            .map(|s| s.user_id)
            .ok_or(LufaSnafu.into_error(LufaError {
                message: format!("not logged in"),
            }))?;

        Ok(user_id)
    }

    fn _build_url(&self, path: &str) -> Result<reqwest::Url> {
        let path: &str = path.trim_start_matches('/');
        let language: &str = self.language.into();
        let full: String = format!("{}/{}/{}", DEFAULT_API_BASE_URL, language, path);

        Url::parse(&full).map_err(|e| UrlParseSnafu.into_error(e.into()))
    }

    pub(crate) async fn _get(&self, path: &str) -> Result<Response> {
        let res = self
            .client
            .get(self._build_url(path)?)
            .header("User-Agent", format!("{}/{}", NAME, VERSION))
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
            .post(self._build_url(path)?)
            .header("User-Agent", format!("{}/{}", NAME, VERSION))
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
            .post(self._build_url(path)?)
            .header("User-Agent", format!("{}/{}", NAME, VERSION))
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
        let req = self
            .client
            .post(self._build_url(path)?)
            .header("User-Agent", format!("{}/{}", NAME, VERSION))
            .form(form)
            .build()
            .unwrap();

        let res = self
            .client
            .execute(req)
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?;

        Ok(res)
    }

    pub fn auth(&self) -> AuthService {
        AuthService(self)
    }

    pub fn billing(&self) -> BillingService {
        BillingService(self)
    }

    pub fn profile(&self) -> ProfileService {
        ProfileService(self)
    }

    pub fn orders(&self) -> OrdersService {
        OrdersService(self)
    }
}
