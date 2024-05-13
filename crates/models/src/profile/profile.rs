use chrono::NaiveDate;
use rusty_money::iso;
use rusty_money::Money;
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::de;

use super::IncentiveData;

// A Profile represents data that is user-specific
#[derive(Deserialize, Debug)]
pub struct Profile {
    // The user ID
    #[serde(rename = "user_id")]
    pub user_id: String,

    // The user's first name
    //
    // E.g.: "John"
    #[serde(rename = "first_name")]
    pub first_name: String,

    // The user's full name
    //
    // E.g.: "John Doe"
    #[serde(rename = "user_name")]
    pub user_name: String,

    // The user's abbreviated name
    //
    // E.g.: "John D."
    #[serde(rename = "donation_name")]
    pub donation_name: String,

    // TODO -- not sure
    #[serde(rename = "subscription_type")]
    pub subscription_type: String,

    // The date at which the user was created
    #[serde(rename = "user_created", deserialize_with = "de::date")]
    pub user_created: NaiveDate,

    // The credits balance for the user. A positive balance
    // indicates you are owed money, while a negative balance
    // indicates that you owe money
    #[serde(rename = "user_credits", deserialize_with = "de::money")]
    pub user_credits: Money<'static, iso::Currency>,

    // The date at which the user became a superlufavore
    #[serde(
        rename = "became_superlufavore_on",
        deserialize_with = "de::date"
    )]
    pub became_superlufavore_on: NaiveDate,

    // The number of people in the user's household
    #[serde(
        rename = "family_size",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub family_size: usize,

    // The date at which the user was created
    #[serde(rename = "created", deserialize_with = "de::date")]
    pub created: NaiveDate,

    // TODO -- not sure
    #[serde(
        default,
        rename = "giveback_donation_percent",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    pub giveback_donation_percent: Option<f64>,

    // TODO -- not sure
    #[serde(rename = "anonymous")]
    pub anonymous: bool,

    // How each weekly subscription order has the base basket
    // populated
    #[serde(rename = "subscriptions_order_prepopulation_method")]
    pub subscriptions_order_prepopulation_method: String,

    // How each "a-la-carte" order has the base basket
    // populated
    #[serde(rename = "orders_order_prepopulation_method")]
    pub orders_order_prepopulation_method: String,

    // The minimum required price for a basket order
    // under the user
    #[serde(
        rename = "min_basket_price",
        deserialize_with = "de::money"
    )]
    pub min_basket_price: Money<'static, iso::Currency>,

    // TODO -- not sure
    #[serde(rename = "reactivation")]
    pub reactivation: bool,

    // TODO -- not sure
    #[serde(
        rename = "user_free_credits_spendable",
        deserialize_with = "de::money"
    )]
    pub user_free_credits_spendable: Money<'static, iso::Currency>,

    // TODO -- not sure
    #[serde(
        rename = "user_all_credits_spendable",
        deserialize_with = "de::money"
    )]
    pub user_all_credits_spendable: Money<'static, iso::Currency>,

    // Incentive data for the user
    #[serde(rename = "incentive_data")]
    pub incentive_data: IncentiveData,

    // The projected earnings amount for the giveback program
    #[serde(rename = "earnings", deserialize_with = "de::money")]
    pub earnings: Money<'static, iso::Currency>,

    // TODO -- not sure
    #[serde(default, rename = "dg_company_coordinator")]
    pub dg_company_coordinator: bool,

    // TODO -- not sure
    #[serde(default, rename = "could_give_remaining_balance")]
    pub could_give_remaining_balance: bool,
}

mod tests {
    use super::Profile;

    #[test]
    fn test_deserialize() {
        let s = r#"
          {
            "user_id": "123456",
            "first_name": "John",
            "user_name": "John Doe",
            "donation_name": "John D.",
            "subscription_type": "0",
            "user_created": "January 01, 2021",
            "user_credits": "-1.00",
            "user_free_credits": "0",
            "subscription_active": "1",
            "became_superlufavore_on": "2022-01-01",
            "charitable_account": "0",
            "family_size": "1",
            "created": "2021-01-01",
            "giveback_donation_percent": "0.00",
            "anonymous": false,
            "subscriptions_order_prepopulation_method": "Based on purchase history",
            "orders_order_prepopulation_method": "Based on purchase history",
            "min_basket_price": "35",
            "reactivation": true,
            "user_free_credits_spendable": "0.00",
            "user_all_credits_spendable": "0.00",
            "zero_amount": "$ 0.00",
            "incentive_data": {
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
            },
            "earnings": "111.66",
            "dg_company_coordinator": false,
            "could_give_remaining_balance": false
        }
        "#;

        let profile: Result<Profile, _> = serde_json::from_str(s);
        assert!(profile.is_ok());

        let profile = profile.unwrap();
        assert_eq!(profile.user_id, "123456");
    }
}
