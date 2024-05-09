use reqwest::StatusCode;
use snafu::IntoError;

use crate::{
    error::{LufaError, LufaSnafu, Result, SerdePhpSnafu, UrlEncodingSnafu},
    models, Lufa, State,
};

#[derive(Debug, Clone)]
pub struct AuthService<'a>(pub(crate) &'a Lufa);

impl<'a> AuthService<'a> {
    /// Logs a user into the Lufa API using their email
    /// and password credentials.
    ///
    /// # Examples
    ///
    /// ```
    /// use lufa::{Lufa, Language, Error, Result};
    ///
    /// async fn sign_in() -> Result<()> {
    ///   let client = Lufa::new(Language::English)?;
    ///   client.auth().login("bob@example.com", "passw0rd").await?;
    ///   Ok(())
    /// }
    /// ```
    pub async fn login<E: AsRef<str>, P: AsRef<str>>(
        &mut self,
        email: E,
        password: P,
    ) -> Result<()> {
        let email = email.as_ref().to_string();
        let password = password.as_ref().to_string();

        let response = self
            .0
            ._post_form("/login", &models::auth::LoginReqForm { email, password })
            .await?;

        // Upon successful login, a cookie, `lufaState`, is sent
        // from the server and contains the user's ID and email
        // (among other values)
        let lufa_state_cookie = response
            .cookies()
            .find(|c| c.name() == "lufaState" && c.value() != "deleted" && c.value().len() > 0)
            .ok_or(LufaSnafu.into_error(LufaError {
                message: format!("failed to login"),
            }))?;

        // There's some 40-character value we don't care about,
        // we'll just ignore that. The cookie is also urlencoded
        // so we decode that here
        let lufa_state_encoded = &lufa_state_cookie.value()[40..];
        let lufa_state_decoded = urlencoding::decode(lufa_state_encoded)
            .map_err(|e| UrlEncodingSnafu.into_error(e.into()))?;

        // The `lufaState` is a PHP-serialized array -- we
        // deserialize it with serde_php
        let lufa_state: models::cookies::LufaState =
            serde_php::from_bytes(lufa_state_decoded.as_bytes())
                .map_err(|e| SerdePhpSnafu.into_error(e.into()))?;

        // Modify the client's state
        {
            let mut state = self.0.state.write().await;
            *state = Some(State {
                user_id: lufa_state.0,
                email: lufa_state.1,
            })
        }

        Ok(())
    }

    /// Logs a user out of the Lufa API.
    ///
    /// # Examples
    ///
    /// ```
    /// use lufa::{Lufa, Language, Error, Result};
    ///
    /// async fn sign_in_and_out() -> Result<()> {
    ///   let client = Lufa::new(Language::English)?;
    ///   client.auth().login("bob@example.com", "passw0rd").await?;
    ///   client.auth().logout().await?;
    ///   Ok(())
    /// }
    /// ```
    pub async fn logout(&mut self) -> Result<()> {
        let response = self.0._get("/logout").await?;

        if response.status() == StatusCode::OK {
            // Modify the client's state
            let mut state = self.0.state.write().await;
            *state = None;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Language, Lufa};

    #[tokio::test]
    async fn test_login_ok() {
        let client = Lufa::new(Language::English).unwrap();

        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        let res = client.auth().login(&email, &password).await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, true);

        let state = client.state.read().await;
        assert!(state.is_some());

        let state = state.clone().unwrap();
        assert_eq!(email, state.email);
    }

    #[tokio::test]
    async fn test_logout_ok() {
        let client = Lufa::new(Language::English).unwrap();
        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        let res = client.auth().login(&email, &password).await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, true);

        let res = client.auth().logout().await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, false);
    }
}
