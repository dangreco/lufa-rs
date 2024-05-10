use std::collections::HashMap;

use rusty_money::{iso, Money};
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::types;

#[derive(Deserialize, Debug)]
pub struct CheckoutAmountsItem {
    #[serde(rename = "price", deserialize_with = "types::deserialize_money")]
    pub price: Money<'static, iso::Currency>,

    #[serde(
        rename = "quantity",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub quantity: usize,

    #[serde(rename = "row_total", deserialize_with = "types::deserialize_money")]
    pub total: Money<'static, iso::Currency>,
}

#[derive(Deserialize, Debug)]
pub struct CheckoutAmounts {
    #[serde(rename = "total", deserialize_with = "types::deserialize_money")]
    pub total: Money<'static, iso::Currency>,

    #[serde(rename = "subtotal", deserialize_with = "types::deserialize_money")]
    pub subtotal: Money<'static, iso::Currency>,

    #[serde(
        rename = "delivery_fees",
        deserialize_with = "types::deserialize_money"
    )]
    pub delivery_fees: Money<'static, iso::Currency>,

    #[serde(
        rename = "remaining_balance",
        deserialize_with = "types::deserialize_money"
    )]
    pub remaining_balance: Money<'static, iso::Currency>,

    #[serde(rename = "balance", deserialize_with = "types::deserialize_money")]
    pub balance: Money<'static, iso::Currency>,

    #[serde(
        rename = "consigne_amount",
        deserialize_with = "types::deserialize_money"
    )]
    pub consigne_amount: Money<'static, iso::Currency>,

    #[serde(rename = "national_tax", deserialize_with = "types::deserialize_money")]
    pub national_tax: Money<'static, iso::Currency>,

    #[serde(
        rename = "provincial_tax",
        deserialize_with = "types::deserialize_money"
    )]
    pub provincial_tax: Money<'static, iso::Currency>,

    #[serde(
        rename = "coupon_discount_amount",
        deserialize_with = "types::deserialize_money"
    )]
    pub coupon_discount_amount: Money<'static, iso::Currency>,

    #[serde(
        rename = "order_donation",
        deserialize_with = "types::deserialize_money"
    )]
    pub order_donation: Money<'static, iso::Currency>,

    #[serde(
        rename = "donation_discount",
        deserialize_with = "types::deserialize_money"
    )]
    pub donation_discount: Money<'static, iso::Currency>,

    #[serde(
        rename = "available_weekly",
        deserialize_with = "types::deserialize_money"
    )]
    pub available_weekly: Money<'static, iso::Currency>,

    #[serde(
        rename = "remaining_weekly",
        deserialize_with = "types::deserialize_money"
    )]
    pub remaining_weekly: Money<'static, iso::Currency>,

    #[serde(rename = "order_details")]
    pub items: HashMap<String, CheckoutAmountsItem>,
}

#[cfg(test)]
mod tests {
    use super::CheckoutAmounts;

    #[test]
    fn test_deserialize() {
        let s = r#"
    {
      "total": "137.95",
      "subtotal": "130.00",
      "delivery_fees": "6,00",
      "remaining_balance": "1.05",
      "balance": "0.00",
      "consigne_amount": "0.00",
      "order_details": {
        "1776": {
          "price": "3.50",
          "quantity": "1",
          "row_total": 3.5
        },
        "2917": {
          "price": "8.75",
          "quantity": "1",
          "row_total": 8.75
        },
        "3210": {
          "price": "6.00",
          "quantity": "1",
          "row_total": 6
        },
        "8945": {
          "price": "12.00",
          "quantity": "1",
          "row_total": 12
        },
        "10274": {
          "price": "4.25",
          "quantity": "1",
          "row_total": 4.25
        },
        "10941": {
          "price": "6.25",
          "quantity": "1",
          "row_total": 6.25
        },
        "11994": {
          "price": "3.00",
          "quantity": "1",
          "row_total": 3
        },
        "12644": {
          "price": "15.50",
          "quantity": "1",
          "row_total": 15.5
        },
        "13357": {
          "price": "6.00",
          "quantity": "1",
          "row_total": 6
        },
        "13378": {
          "price": "16.75",
          "quantity": "1",
          "row_total": 16.75
        },
        "13763": {
          "price": "20.25",
          "quantity": "1",
          "row_total": 20.25
        },
        "15304": {
          "price": "5.00",
          "quantity": "1",
          "row_total": 5
        },
        "16414": {
          "price": "4.00",
          "quantity": "1",
          "row_total": 4
        },
        "16443": {
          "price": "1.75",
          "quantity": "1",
          "row_total": 1.75
        },
        "16814": {
          "price": "2.25",
          "quantity": "1",
          "row_total": 2.25
        },
        "17161": {
          "price": "3.50",
          "quantity": "1",
          "row_total": 3.5
        },
        "17350": {
          "price": "5.00",
          "quantity": "1",
          "row_total": 5
        },
        "17719": {
          "price": "1.75",
          "quantity": "1",
          "row_total": 1.75
        },
        "17811": {
          "price": "0.00",
          "quantity": "1",
          "row_total": 0
        },
        "17817": {
          "price": "4.75",
          "quantity": "1",
          "row_total": 4.75
        },
        "17931": {
          "price": "5.75",
          "quantity": "1",
          "row_total": 5.75
        }
      },
      "national_tax": "0.30",
      "provincial_tax": "0.60",
      "coupon_discount_amount": "0.00",
      "nb_item": 20,
      "basket_array": [
        85
      ],
      "order_donation": "0.00",
      "donation_discount": "0.00",
      "available_weekly": "0.00",
      "remaining_weekly": "0.00",
      "unformatted_total": 137.95,
      "unformatted_order_donation": "0.00",
      "unformatted_subtotal": 130,
      "unformatted_delivery_fees": 6,
      "unformatted_remaining_balance": 1.05,
      "unformatted_balance": 0,
      "unformatted_consigne_amount": 0,
      "unformatted_coupon_discount_amount": 0
    }
    "#;

        let amounts = serde_json::from_str::<CheckoutAmounts>(s);
        assert!(amounts.is_ok());
    }
}
