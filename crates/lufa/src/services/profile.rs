use snafu::IntoError;

use crate::{
    error::{LufaError, LufaSnafu, ReqwestSnafu},
    models, Lufa, Result,
};

#[derive(Debug, Clone)]
pub struct ProfileService<'a>(pub(crate) &'a Lufa);

impl<'a> ProfileService<'a> {
    /// Retrieves the profile of the currently
    /// logged in user.
    ///
    /// # Examples
    ///
    /// ```
    /// use lufa::{Lufa, Language, Error, Result};
    ///
    /// async fn get_my_cards() -> Result<()> {
    ///   let client = Lufa::new(Language::English)?;
    ///   client.auth().login("bob@example.com", "passw0rd").await?;
    ///     
    ///   let profile = client.profile().get().await?;
    ///
    ///   println!("Account under {}:", profile.donation_name);
    ///   println!("\tFamily Size: {} person(s)", profile.family_size);
    ///   println!("\tCredits: ${:?}", profile.user_credits);
    ///   println!("\tEarnings: ${:?}", profile.earnings);
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn get(&self) -> Result<models::profile::Profile> {
        self.0.guard_logged_in().await?;

        self.0
            ._post_form(
                "/users/profileData",
                &models::PerUserForm {
                    user_id: self.0.user_id().await?,
                },
            )
            .await?
            .json::<models::ApiResponse<models::profile::Profile>>()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?
            .data
            .ok_or(LufaSnafu.into_error(LufaError {
                message: format!("failed to get profile data"),
            }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Language, Lufa};

    #[tokio::test]
    async fn test_get_profile_ok() {
        let client = Lufa::new(Language::English).unwrap();
        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        let res = client.auth().login(&email, &password).await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, true);

        let profile = client.profile().get().await;
        assert!(profile.is_ok());
    }
}
