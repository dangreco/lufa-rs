use chrono::DateTime;
use chrono_tz::Tz;
use rusty_money::{iso, Money};
use serde::Deserialize;

use crate::types;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "order_id")]
    pub order_id: String,

    #[serde(rename = "total", deserialize_with = "types::deserialize_money")]
    pub total: Money<'static, iso::Currency>,

    #[serde(rename = "title_string")]
    pub title: String,

    #[serde(
        rename = "transaction_time",
        deserialize_with = "types::deserialize_timestamp"
    )]
    pub timestamp: DateTime<Tz>,

    #[serde(rename = "transaction_type")]
    pub _type: String,

    #[serde(
        default,
        rename = "total_order_amount",
        deserialize_with = "types::deserialize_money_optional"
    )]
    pub total_order_amount: Option<Money<'static, iso::Currency>>,

    #[serde(
        default,
        rename = "basket_cost",
        deserialize_with = "types::deserialize_money_optional"
    )]
    pub basket_cost: Option<Money<'static, iso::Currency>>,

    #[serde(
        default,
        rename = "previous_amount_due",
        deserialize_with = "types::deserialize_money_optional"
    )]
    pub previous_amount_due: Option<Money<'static, iso::Currency>>,

    #[serde(
        default,
        rename = "donation_amount",
        deserialize_with = "types::deserialize_money_optional"
    )]
    pub donation_amount: Option<Money<'static, iso::Currency>>,

    #[serde(
        default,
        rename = "charity_received",
        deserialize_with = "types::deserialize_money_optional"
    )]
    pub charity_received: Option<Money<'static, iso::Currency>>,

    #[serde(
        default,
        rename = "total_consigne_amount",
        deserialize_with = "types::deserialize_money_optional"
    )]
    pub total_consigne_amount: Option<Money<'static, iso::Currency>>,
}

#[cfg(test)]
mod tests {
    use super::Transaction;

    #[test]
    fn test_deserialize() {
        let s = r#"
          {
            "transactions": {
              "20": {
                "transaction_id": "12345678",
                "transaction_order_id": "12345678",
                "transaction_type": "20",
                "transaction_amount": "-4.18",
                "transaction_time": "2024-05-07 00:32:54",
                "transaction_last_4": null,
                "total_order_amount": "140.91",
                "donation_amount": "0.00",
                "total_consigne_amount": "0.25",
                "charity_received": "0.00",
                "previous_amount_due": "0.00",
                "basket_cost": "140.66000366210938",
                "product_name": null,
                "transaction_amount_formatted": "$ 4.18",
                "transaction_date": "2024-05-07",
                "transaction_type_formatted": "Basket Order - Remaining credits payment"
              },
              "38": {
                "transaction_id": "12345678",
                "transaction_order_id": "12345678",
                "transaction_type": 38,
                "transaction_amount": -1.05,
                "transaction_time": "2024-05-07 14:38:31",
                "transaction_last_4": null,
                "total_order_amount": "140.91",
                "donation_amount": "0.00",
                "total_consigne_amount": "0.25",
                "charity_received": "0.00",
                "previous_amount_due": "0.00",
                "basket_cost": "140.66000366210938",
                "product_name": "Champignons shiitakes",
                "transaction_amount_formatted": " - $ 1.05",
                "transaction_date": "2024-05-07",
                "transaction_type_formatted": "Product price adjustments based on weight"
              }
            },
            "order_id": "12345678",
            "total": "-136.73",
            "formatted_total": "$ 136.73",
            "title_string": "Basket Order - Credit card payment 1234",
            "transaction_date": "2024-05-07",
            "transaction_time": "2024-05-07 00:32:54",
            "transaction_type": "21",
            "total_order_amount": "$ 140.91",
            "basket_cost": "$ 140.66",
            "previous_amount_due": "",
            "donation_amount": "",
            "charity_received": "",
            "total_consigne_amount": "$ 0.25",
            "ranges": [
              true,
              true,
              true,
              true
            ]
          }
        "#;

        let tx: Result<Transaction, _> = serde_json::from_str(s);
        assert!(tx.is_ok());
    }
}
