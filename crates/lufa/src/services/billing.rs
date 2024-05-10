use std::collections::HashMap;

use lufa_models::billing::Card;
use snafu::IntoError;

use crate::{
    error::{LufaError, LufaSnafu, ReqwestSnafu},
    models, Lufa, Result,
};

#[derive(Debug, Clone)]
pub struct BillingService<'a>(pub(crate) &'a Lufa);

impl<'a> BillingService<'a> {

    /// Retrieves the billing data for the currently
    /// logged in user.
    /// 
    /// This function is not to be used by client code,
    /// thus it is not marked as public.
    /// ```
    async fn get_billing_data(&self) -> Result<models::billing::BillingData> {
        self.0
            ._post_form(
                "/users/billingData",
                &models::PerUserForm {
                    user_id: self.0.user_id().await?,
                },
            )
            .await?
            .json::<models::ApiResponse<models::billing::BillingData>>()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))?
            .data
            .ok_or(LufaSnafu.into_error(LufaError {
                message: format!("failed to get billing data"),
            }))
    }

    /// Retrieves the configured payment cards for
    /// the currently logged in user.
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
    ///   let cards = client.billing().get_cards().await?;
    /// 
    ///   for (_, card) in cards.into_iter() {
    ///     println!("{} - {}", card.brand, card.last_four);
    ///   }
    /// 
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_cards(&self) -> Result<HashMap<usize, Card>> {
        self.get_billing_data().await.map(|bd| bd.cards)
    }

    /// Retrieves the list of completed transactions for
    /// the currently logged in user.
    ///
    /// # Examples
    ///
    /// ```
    /// use lufa::{Lufa, Language, Error, Result};
    ///
    /// async fn find_most_expensive_tx() -> Result<()> {
    ///   let client = Lufa::new(Language::English)?;
    ///   client.auth().login("bob@example.com", "passw0rd").await?;
    ///     
    ///   let txs = client.billing().get_transactions().await?;
    ///   
    ///   let max_total = txs.iter()
    ///     .filter_map(|tx| tx.total)
    ///     .max();
    /// 
    ///   match max_total {
    ///     Some(max) => println!("Maximum total: ${}", max),
    ///     None => println!("No transactions!"),
    ///   }
    /// 
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_transactions(&self) -> Result<Vec<models::billing::Transaction>> {
        self.get_billing_data().await.map(|bd| bd.transactions)
    }
    
}

#[cfg(test)]
mod tests {
    use crate::{Language, Lufa};

    #[tokio::test]
    async fn test_get_cards_ok() {
        let client = Lufa::new(Language::English).unwrap();
        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        let res = client.auth().login(&email, &password).await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, true);

        let cards = client.billing().get_cards().await;
        assert!(cards.is_ok());
    }

    #[tokio::test]
    async fn test_get_transactions_ok() {
        let client = Lufa::new(Language::English).unwrap();
        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        let res = client.auth().login(&email, &password).await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, true);

        let txs = client.billing().get_transactions().await;
        assert!(txs.is_ok());
    }
}
