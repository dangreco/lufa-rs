use crate::utils;
use chrono::NaiveDate;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Card {
    #[serde(rename = "cc_id")]
    pub id: String,

    #[serde(rename = "cc_type")]
    pub brand: String,

    #[serde(rename = "cc_last_4")]
    pub last_four: String,

    #[serde(rename = "cc_exp", deserialize_with = "utils::naive_date_from_str")]
    pub expiry: NaiveDate,

    #[serde(rename = "expired")]
    pub expired: bool,

    #[serde(rename = "cc_priority", deserialize_with = "utils::deserialize_usize")]
    pub priority: usize,

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
