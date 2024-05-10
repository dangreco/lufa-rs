use chrono::DateTime;
use chrono_tz::Tz;
use rusty_money::{iso, Money};
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::types;

#[derive(Deserialize, Debug)]
pub struct IncentiveData {
    #[serde(
        rename = "iso_week",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub iso_week: u8,

    #[serde(
        rename = "ordered_this_week",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub ordered_this_week: usize,

    #[serde(
        rename = "nb_weeks_considered",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub weeks_considered: usize,

    #[serde(
        rename = "nb_weeks_with_purchase",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub weeks_with_purchase: usize,

    #[serde(
        rename = "amount_spent",
        deserialize_with = "types::deserialize_money"
    )]
    pub amount_spent: Money<'static, iso::Currency>,

    #[serde(
        rename = "current_earnings",
        deserialize_with = "types::deserialize_money"
    )]
    pub current_earnings: Money<'static, iso::Currency>,

    #[serde(
        rename = "projected_earnings",
        deserialize_with = "types::deserialize_money"
    )]
    pub projected_earnings: Money<'static, iso::Currency>,

    #[serde(
        rename = "created_at",
        deserialize_with = "types::deserialize_timestamp"
    )]
    pub created_at: DateTime<Tz>,

    #[serde(
        rename = "updated_at",
        deserialize_with = "types::deserialize_timestamp"
    )]
    pub updated_at: DateTime<Tz>,

    #[serde(rename = "weeks_ordered")]
    pub weeks_ordered: String,

    #[serde(rename = "percentage_of_time")]
    pub percentage_of_time: String,
}

mod tests {
    use super::IncentiveData;

    #[test]
    fn test_deserialize() {
        let s = r#"
        {
          "iso_week": "19",
          "ordered_this_week": "1",
          "nb_weeks_considered": "19",
          "nb_weeks_with_purchase": "17",
          "take_rate": "89",
          "pct": "2",
          "amount_spent": "2040.07",
          "current_earnings": "40.80",
          "projected_earnings": "111.66",
          "created_at": "2024-05-06 09:00:14",
          "updated_at": "2024-05-09 09:00:17",
          "active": "1",
          "subscription_type": "0",
          "subscription_status": "1",
          "weeks_ordered": "17 out of 19 weeks",
          "percentage_of_time": "(89 % of the time)"
        }
        "#;

        let data: Result<IncentiveData, _> = serde_json::from_str(s);
        assert!(data.is_ok());
    }
}
