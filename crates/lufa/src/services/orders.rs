use lufa_models::PerOrderForm;
use snafu::IntoError;

use crate::{models, Lufa, ReqwestSnafu, Result};

#[derive(Debug, Clone)]
pub struct OrdersService<'a>(pub(crate) &'a Lufa);

impl<'a> OrdersService<'a> {
    pub async fn get_active(&self) -> Result<Option<models::orders::Order>> {
        self.0.guard_logged_in().await?;

        self.0
            ._get("/superMarket/GetUserOrderDetails")
            .await?
            .json::<models::orders::Order>()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))
            .map(Some)
    }

    pub async fn track<O: AsRef<str>>(&self, order_id: O) -> Result<models::orders::OrderTracking> {
        self.0.guard_logged_in().await?;

        self.0
            ._post_form(
                "/orders/getTrackOrderData",
                &PerOrderForm {
                    order_id: order_id.as_ref().to_string(),
                },
            )
            .await?
            .json::<models::orders::OrderTracking>()
            .await
            .map_err(|e| ReqwestSnafu.into_error(e.into()))
    }
}

#[cfg(test)]
mod tests {
    use snafu::IntoError;

    use crate::{Error, Language, Lufa, LufaError, LufaSnafu};

    #[tokio::test]
    async fn test_get_active_order() {
        let client = Lufa::new(Language::English).unwrap();
        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        let res = client.auth().login(&email, &password).await;
        assert!(res.is_ok());
        assert_eq!(client.is_logged_in().await, true);

        let order = client.orders().get_active().await;
        assert!(order.is_ok());
    }

    #[tokio::test]
    async fn test_get_tracking() -> Result<(), Error> {
        env_logger::init();

        let client = Lufa::new(Language::English).unwrap();
        let email = std::env::var("LUFA_EMAIL").unwrap();
        let password = std::env::var("LUFA_PASSWORD").unwrap();

        client.auth().login(&email, &password).await?;
        let order = client
            .orders()
            .get_active()
            .await?
            .ok_or(LufaSnafu.into_error(LufaError {
                message: format!("failed to get order"),
            }))?;

        let tracking = client.orders().track(order.id).await;
        assert!(tracking.is_ok());

        Ok(())
    }

    // https://montreal.lufa.com/en/orders/getTrackOrderData
}
