use chrono::DateTime;
use chrono_tz::Tz;
use rusty_money::{iso, Money};
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::de;

// Data corresponding to the Lufa incentive program.
// Contains various statistics pertaining to the 
// user's order habits
#[derive(Deserialize, Debug)]
pub struct IncentiveData {

    // The current ISO week of the year
    #[serde(
        rename = "iso_week",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub iso_week: u8,

    // The number of baskets ordered this week
    //
    // Note: might actually be a boolean on if an order 
    // was placed during the week, not sure yet.
    #[serde(
        rename = "ordered_this_week",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub ordered_this_week: usize,

    // The amount of weeks that have passed by in 
    // year the so far
    #[serde(
        rename = "nb_weeks_considered",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub weeks_considered: usize,

    // The amount of weeks that the user has completed
    // an order
    #[serde(
        rename = "nb_weeks_with_purchase",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub weeks_with_purchase: usize,

    // The total amount spent this year
    #[serde(
        rename = "amount_spent",
        deserialize_with = "de::money"
    )]
    pub amount_spent: Money<'static, iso::Currency>,

    // The current amount earned for the giveback program
    #[serde(
        rename = "current_earnings",
        deserialize_with = "de::money"
    )]
    pub current_earnings: Money<'static, iso::Currency>,

    // The projected amount earnined for the giveback program
    #[serde(
        rename = "projected_earnings",
        deserialize_with = "de::money"
    )]
    pub projected_earnings: Money<'static, iso::Currency>,

    // The datetime at which the statistics were generated
    #[serde(
        rename = "created_at",
        deserialize_with = "de::timestamp"
    )]
    pub created_at: DateTime<Tz>,

    // The datetime at which the statistics were updated
    #[serde(
        rename = "updated_at",
        deserialize_with = "de::timestamp"
    )]
    pub updated_at: DateTime<Tz>,

    // A string representing the amount of weeks ordered
    //
    // E.g.: "17 out of 19 weeks"
    #[serde(rename = "weeks_ordered")]
    pub weeks_ordered: String,

    // A string representing the percentage of weeks that
    // the user has completed an order for
    //
    // E.g.: "(89% of the time)"
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
