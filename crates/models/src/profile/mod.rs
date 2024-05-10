use serde::Deserialize;

use crate::types::{self, Currency};
use crate::utils;

#[derive(Deserialize, Debug)]
pub struct Profile {
    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "user_name")]
    pub user_name: String,

    #[serde(rename = "donation_name")]
    pub donation_name: String,

    #[serde(rename = "subscription_type")]
    pub subscription_type: String,

    #[serde(default)]
    #[serde(
        rename = "user_credits",
        deserialize_with = "types::deserialize_currency"
    )]
    pub user_credits: Currency,

    #[serde(rename = "family_size", deserialize_with = "utils::deserialize_usize")]
    pub family_size: usize,

    #[serde(rename = "anonymous")]
    pub anonymous: bool,

    #[serde(default)]
    #[serde(
        rename = "min_basket_price",
        deserialize_with = "types::deserialize_currency"
    )]
    pub min_basket_price: Currency,

    #[serde(rename = "reactivation")]
    pub reactivation: bool,


    #[serde(default)]
    #[serde(
        rename = "earnings",
        deserialize_with = "types::deserialize_currency"
    )]
    pub earnings: Currency,
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
          "display_order_prepopulation_method": "show_switch",
          "subscriptions_order_prepopulation_method_number": 1,
      "reactivation": true,
          "user_free_credits_spendable": "0.00",
          "user_free_credits_spendable_formatted": "$ 0.00",
          "user_all_credits_spendable": "0.00",
          "user_all_credits_spendable_formatted": "$ 0.00",
          "donation_name_formatted": "Check here to make an anonymous contribution instead of displaying your first name and last initial.",
          "last_year": "Giveback to spend 2023",
          "user_credits_abs": "1.00",
          "user_credits_formatted": "- $ 1.00",
          "positive_credits_format": "$1.00",
          "zero_amount": "$ 0.00",
          "incentive_data": {
            "supervores_incentive_log_id": "12345678",
            "iso_week": "19",
            "year": "2024",
            "user_id": "123456",
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
            "super_lufavore_profile_prompt": "You've been a Super Lufavore for 2 years now!",
            "giveback_by_date": "By Dec. 31st, 2024",
            "currentlyInfoBubble": "2% of your orders to date in 2024.",
            "forecastInfoBubble": "A forecast of your total 2024 Giveback if you keep up your current order habits.",
            "end_of_year": "2024",
            "baskets_ordered": "In 2024, you have gotten a basket",
            "weeks_ordered": "17 out of 19 weeks",
            "percentage_of_time": "(89 % of the time)",
            "giveback_donation_title": "Give back to the community in 2024"
          },
          "donation_data": {
            "donates": "",
            "pct": "",
            "options": [
              0.25,
              0.5,
              0.75,
              1
            ]
          },
      "earnings": "111.66",
          "potential_spendable_credits_cancelation_date": "You’re on track to earn <span class=\"green-text small-text\">$111.66</span> in 2025. If you cancel your subscription, you’ll lose your Giveback credits.",
          "dg_company_coordinator": false,
          "thank_you_button": "Thank you!",
          "submit_button": "Submit",
          "show_walkthrought": false,
          "could_give_remaining_balance": false,
          "show_newsletter_pop_up": false,
          "new_weekly_subscription_walkthrough": {
            "1": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-1.jpg",
              "title": "Hey Daniel, welcome to your account settings.",
              "subtitle": null,
              "image_type": "jpg"
            },
            "2": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-2.jpg",
              "title": "Our mission is to create a better food system",
              "subtitle": "A weekly basket of local produce prepared on the morning of your delivery is the best way to eat responsibly and reduce food waste.",
              "image_type": "jpg"
            },
            "3": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-3.svg",
              "title": "Your basket will be activated each week",
              "subtitle": "Your Marketplace opens three days before your scheduled delivery day. Each week, you’ll receive an email reminding you that your basket is ready to customize.",
              "image_type": "svg"
            },
            "4": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-4.svg",
              "title": "It’s easy to keep track",
              "subtitle": "Your weekly orders, extra baskets, and gift baskets are all visible at the top of your account settings so you can easily keep track of what’s coming your way.",
              "image_type": "svg"
            },
            "5": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-5.svg",
              "title": "Hit pause whenever you want",
              "subtitle": "If you’re leaving on vacation or already have a full fridge, we’ve made it easy to skip a week or pause your subscription through the default delivery section of your account settings.",
              "image_type": "svg"
            },
            "6": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-6.svg",
              "title": "The base basket",
              "subtitle": "When your Marketplace opens, your basket will be filled with $35 worth of seasonal produce. Since your basket is fully customizable, feel free to add and remove any item and personalize to fit your needs.",
              "image_type": "svg"
            },
            "7": {
              "image": "/images/smart_base_basket.svg",
              "title": "The personalized base basket",
              "subtitle": "After your second order, the base basket will start to take into account your most recent order history and average basket price to fill your weekly basket with a selection of products tailored to you.",
              "image_type": "svg"
            },
            "8": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-7.svg",
              "title": "No need to check out",
              "subtitle": "Your order will be processed automatically at midnight before your delivery day. This allows our local partners to prepare your goods and our team to assemble  your baskets the next morning, all in time for delivery to you that very day!",
              "image_type": "svg"
            },
            "9": {
              "image": "/images/users/account-settings/New-weekly-subscription-walkthrough/Carousel-8-EN.svg",
              "title": "The yearly Giveback",
              "subtitle": "As part of our Giveback program,  you’ll earn up to 2% of your yearly Lufa Farms purchases back in Marketplace credits at the end of the year. The more you order, the more you earn! Check the Giveback section of your account settings to see your estimated earnings.",
              "image_type": "svg"
            }
          },
          "giveback_video": "https://youtu.be/52XwUqGIQbo",
          "profile_switch_walkthrough_slide": 1,
          "walkthrough_display_info": true,
          "profile_switch_walkthrough": {
            "1": {
              "image": "/images/users/account-settings/Lufavore-walkthrough-switch/Carousel-1.jpg",
              "title": "We’re simplifying our subscription: on March 28, 2022, we’re discontinuing the activation feature.",
              "subtitle": null,
              "image_type": "jpg"
            },
            "2": {
              "image": "/images/users/account-settings/Lufavore-walkthrough-switch/Carousel-2.jpg",
              "title": "Help us better plan harvests and prepare your orders with minimum waste and maximum freshness",
              "subtitle": "We’re making a significant change to move forward with our mission of building a better food system and we need the support of all of our Lufavores. Find out what this changes for you.",
              "image_type": "jpg"
            },
            "3": {
              "image": "/images/users/account-settings/Lufavore-walkthrough-switch/Carousel-3.svg",
              "title": "Your basket will be automatically activated every week",
              "subtitle": "When your Marketplace opens, you’ll find your base basket waiting and ready to customize!",
              "image_type": "svg"
            },
            "4": {
              "image": "/images/users/account-settings/Lufavore-walkthrough-switch/Carousel-4.svg",
              "title": "Don’t need a basket this week?",
              "subtitle": "We’ve made it even easier to cancel your order before your delivery day or pause your subscription for an extended period.",
              "image_type": "svg"
            },
            "5": {
              "image": "/images/users/account-settings/Lufavore-walkthrough-switch/Carousel-5-EN.svg",
              "title": "The yearly Giveback",
              "subtitle": "You’ll be part of our Giveback program! This means you can earn up to 2% of your Lufa Farms purchases back in Marketplace credits at the end of the year.",
              "image_type": "svg"
            },
            "6": {
              "image": "/images/users/account-settings/Lufavore-walkthrough-switch/Carousel-6.jpg",
              "title": "Switch your subscription to continue eating fresh, local, responsible!",
              "subtitle": "You have 773 days left to switch subscription, otherwise your account will be cancelled.",
              "image_type": "jpg"
            }
          }
      }
      "#;

        let profile: Result<Profile, _> = serde_json::from_str(s);
        assert!(profile.is_ok());

        let profile = profile.unwrap();
        assert_eq!(profile.user_id, "123456");
        println!("{:?}", profile);
    }
}
