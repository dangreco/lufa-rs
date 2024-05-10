// Deserializers for money/currency strings.

use rusty_money::{iso, Money};
use serde::{Deserialize, Deserializer};


// Function to parse a Money instance from a North American-styled currency
// string, i.e. "." for decimal points, "," for thousands separators.
fn parse_na<'de, D>(s: &str) -> Result<Money<'static, iso::Currency>, D::Error>
where
    D: Deserializer<'de>,
{
    let cleaned = s.replace(",", "");
    Money::from_str(&cleaned, iso::CAD).map_err(serde::de::Error::custom)
}

// Function to parse a Money instance from a European-styled currency
// string, i.e. "," for decimal points, "." for thousands separators.
fn parse_eu<'de, D>(s: &str) -> Result<Money<'static, iso::Currency>, D::Error>
where
    D: Deserializer<'de>,
{
    let cleaned = s.replace(".", "").replace(",", ".");
    Money::from_str(&cleaned, iso::CAD).map_err(serde::de::Error::custom)
}

// Function to parse a Money instance from either a North American or
// European styled currency string. Examples below:
//
// `"$ 1,00" => 1 CAD`
// `"$ 1.00" => 1 CAD`
// `"$ 1,000.00" => 1_000 CAD`
// `"$ 1.000,00" => 1_000 CAD`
// `"$ 1,000,000.00" => 1_000_000 CAD`
// `"$ 1.000.000,00" => 1_000_000 CAD`
fn parse_str<'de, D>(s: &str) -> Result<Money<'static, iso::Currency>, D::Error>
where
    D: Deserializer<'de>,
{
    let cleaned: String = s
        .chars()
        .filter(|&c| c.is_digit(10) || c == '.' || c == ',' || c == '-' || c == '+')
        .collect();

    if cleaned.len() == 0 {
        return Err(serde::de::Error::custom("empty string"));
    }

    let dot_count = cleaned.chars().filter(|&c| c == '.').count();
    let comma_count = cleaned.chars().filter(|&c| c == ',').count();

    match (dot_count, comma_count) {
        // treat as regular number
        (0, 0) => Money::from_str(&cleaned, iso::CAD).map_err(serde::de::Error::custom),
        (1, 0) => {
            // Get number of characters after dot
            let n = cleaned.len() - (cleaned.find(".").unwrap() + 1);

            match n {
                0 | 1 | 2 => parse_na::<D>(&cleaned),
                3 => parse_eu::<D>(&cleaned),
                _ => Err(serde::de::Error::custom("malformed currency string")),
            }
        }
        (0, 1) => {
            // Get number of characters after comma
            let n = cleaned.len() - (cleaned.find(",").unwrap() + 1);

            match n {
                0 | 1 | 2 => parse_eu::<D>(&cleaned),
                3 => parse_na::<D>(&cleaned),
                _ => Err(serde::de::Error::custom("malformed currency string")),
            }
        }
        (1, 1) => {
            // Compare indexes of dot and comma
            let idx_dot = cleaned.find(".").unwrap();
            let idx_comma = cleaned.find(",").unwrap();

            if idx_dot > idx_comma {
                parse_na::<D>(&cleaned)
            } else if idx_comma > idx_dot {
                parse_eu::<D>(&cleaned)
            } else {
                Err(serde::de::Error::custom(
                    "something has gone terribly wrong",
                ))
            }
        }
        (1, _) => {
            // We must have a NA string
            parse_na::<D>(&cleaned)
        }
        (_, 1) => {
            // We must have a EU string
            parse_eu::<D>(&cleaned)
        }
        _ => Err(serde::de::Error::custom(
            "malformed currency string -- too many dots/commmas",
        )),
    }
}

// Deserializes a value into a Money instance in CAD
pub fn deserialize_money<'de, D>(deserializer: D) -> Result<Money<'static, iso::Currency>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MoneyFormat {
        String(String),
        Int(i64),
        Float(f64),
    }

    match MoneyFormat::deserialize(deserializer)? {
        MoneyFormat::Int(n) => Ok(Money::from_major(n, iso::CAD)),
        MoneyFormat::Float(n) => Ok(Money::from_decimal(
            n.try_into().map_err(serde::de::Error::custom)?,
            iso::CAD,
        )),
        MoneyFormat::String(s) => parse_str::<D>(&s),
    }
}

// Optionally deserializes a value into a Money instance in CAD
pub fn deserialize_money_optional<'de, D>(
    deserializer: D,
) -> Result<Option<Money<'static, iso::Currency>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MoneyFormat {
        String(String),
        Int(i64),
        Float(f64),
        Null,
    }

    match MoneyFormat::deserialize(deserializer)? {
        MoneyFormat::Int(n) => Ok(Some(Money::from_major(n, iso::CAD))),
        MoneyFormat::Float(n) => Ok(Some(Money::from_decimal(
            n.try_into().map_err(serde::de::Error::custom)?,
            iso::CAD,
        ))),
        MoneyFormat::String(s) => match parse_str::<D>(&s) {
            Ok(m) => Ok(Some(m)),
            Err(e) => {
                if e.to_string() == "empty string" {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        },
        MoneyFormat::Null => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::deserialize_money;
    use rusty_money::{iso, Money};
    use serde::Deserialize;

    #[test]
    fn test_deserialize_money() {
        #[derive(Deserialize)]
        struct Price {
            #[serde(deserialize_with = "deserialize_money")]
            pub amount: Money<'static, iso::Currency>,
        }

        // integers
        assert!(serde_json::from_str::<Price>("{ \"amount\": -1 }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": 0 }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": 1 }").is_ok());

        // floats
        assert!(serde_json::from_str::<Price>("{ \"amount\": 1.0 }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": 0.0 }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": -1.0 }").is_ok());

        // strings - int
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"1\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"0\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"-1\" }").is_ok());

        // strings - float, NA
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"1.0\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"0.0\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"-1.0\" }").is_ok());

        // strings - float, EU
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"1,0\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"0,0\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"-1,0\" }").is_ok());

        // strings - currency, US
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"$ 1.00\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"$ 0.00\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"$ -1.00\" }").is_ok());

        // strings - currency, EU
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"$ 1,00\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"$ 0,00\" }").is_ok());
        assert!(serde_json::from_str::<Price>("{ \"amount\": \"$ -1,00\" }").is_ok());

        // NA format
        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1.0\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1.00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1000.00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1_000, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1,000.00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1_000, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1,000,000.00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1_000_000, iso::CAD));

        // EU format
        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1,0\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1,00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1000,00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1_000, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1.000,00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1_000, iso::CAD));

        let p = serde_json::from_str::<Price>("{ \"amount\": \"$1.000.000,00\"}");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().amount, Money::from_major(1_000_000, iso::CAD));
    }
}
