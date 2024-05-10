use snafu::IntoError;

use crate::{models, Lufa, ReqwestSnafu, Result};

#[derive(Debug, Clone)]
pub struct OrdersService<'a>(pub(crate) &'a Lufa);

impl<'a> OrdersService<'a> {
    pub async fn get_active(&self) -> Result<Option<models::orders::Order>> {
        self.0
            ._get("/superMarket/GetUserOrderDetails")
            .await?
            .json::<models::orders::Order>()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))
            .map(Some)
    }
}


#[cfg(test)]
mod tests {
    use crate::{Language, Lufa};

    #[tokio::test]
    async fn test_get_active_order() {
        let client = Lufa::new(Language::English).unwrap();
        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        let res = client.auth().login(&email, &password).await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, true);

        let cards = client.billing().get_cards().await;
        assert!(cards.is_ok());
    }

}
