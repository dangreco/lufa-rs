use crate::utils;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_aux::prelude::*;

// A Card represents a user's saved card
// information from the platform
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Card {
    // The card ID
    #[serde(rename = "cc_id")]
    pub id: String,

    // The brand of the card, e.g. "Visa", "MasterCard"
    #[serde(rename = "cc_type")]
    pub brand: String,

    // The last four digits of the card number
    #[serde(rename = "cc_last_4")]
    pub last_four: String,

    // The expiry date of the card
    #[serde(rename = "cc_exp", deserialize_with = "utils::naive_date_from_str")]
    pub expiry: NaiveDate,

    // Whether or not the card is expired
    #[serde(rename = "expired")]
    pub expired: bool,

    // The payment priority of the card.
    // Cards of lowest priority number are of highest priority,
    // and will be attempted to be billed first. 
    #[serde(rename = "cc_priority", deserialize_with = "deserialize_number_from_string")]
    pub priority: usize,

    // The type of card, e.g. "primary", "secondary"
    #[serde(rename = "type")]
    pub _type: String,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::Card;

    #[test]
    fn test_deserialize() {
        let s = r#"
          {
            "cc_type": "Visa",
            "cc_last_4": "1234",
            "exp_month": "01",
            "exp_year": "24",
            "cc_exp": "01/24",
            "cc_priority": "1",
            "cc_id": "123456",
            "type": "primary",
            "expired": false
          }
        "#;

        let card: Result<Card, _> = serde_json::from_str(s);
        assert!(card.is_ok());

        let card = card.unwrap();

        assert_eq!(card.id, "123456");
        assert_eq!(card.brand, "Visa");
        assert_eq!(card.last_four, "1234");
        assert_eq!(card.expiry, NaiveDate::from_ymd_opt(2024, 01, 01).unwrap());
        assert_eq!(card.expired, false);
        assert_eq!(card.priority, 1);
        assert_eq!(card._type, "primary");
    }
}
