use serde::{Deserialize, Deserializer};

// Very generous deserializer of boolean values.
// This function accepts:
//   1. String values of the forms:
//     - "true", "t", "yes", "y", "1" => Some(true)
//     - "false", "f", "no", "n", "0" => Some(false)
//   2. Integral values s.t. 0 => Some(false), 1 => Some(true)
//   3. Decimal values s.t. 0.0 => Some(false), 1.0 => Some(true)
//   4. Null values => None
// All other values error.
pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    use std::f64::EPSILON;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AnythingOrBoolOrNull {
        String(String),
        Int(i64),
        Float(f64),
        Boolean(bool),
        Null,
    }

    match AnythingOrBoolOrNull::deserialize(deserializer)? {
        AnythingOrBoolOrNull::Boolean(b) => Ok(Some(b)),
        AnythingOrBoolOrNull::Int(i) => match i {
            1 => Ok(Some(true)),
            0 => Ok(Some(false)),
            _ => Err(serde::de::Error::custom("The number is neither 1 nor 0")),
        },
        AnythingOrBoolOrNull::Float(f) => {
            if (f - 1.0f64).abs() < EPSILON {
                Ok(Some(true))
            } else if f == 0.0f64 {
                Ok(Some(false))
            } else {
                Err(serde::de::Error::custom(
                    "The number is neither 1.0 nor 0.0",
                ))
            }
        }
        AnythingOrBoolOrNull::String(string) => {
            let s = string.to_lowercase();
            let s = s.trim();

            match s {
                "true" | "t" | "yes" | "y" | "1" => Ok(Some(true)),
                "false" | "f" | "no" | "n" | "0" => Ok(Some(false)),
                _ => {
                    if let Ok(b) = string.parse::<bool>() {
                        Ok(Some(b))
                    } else if let Ok(i) = string.parse::<i64>() {
                        match i {
                            1 => Ok(Some(true)),
                            0 => Ok(Some(false)),
                            _ => Err(serde::de::Error::custom("The number is neither 1 nor 0")),
                        }
                    } else if let Ok(f) = string.parse::<f64>() {
                        if (f - 1.0f64).abs() < EPSILON {
                            Ok(Some(true))
                        } else if f == 0.0f64 {
                            Ok(Some(false))
                        } else {
                            Err(serde::de::Error::custom(
                                "The number is neither 1.0 nor 0.0",
                            ))
                        }
                    } else {
                        Err(serde::de::Error::custom(format!(
                            "Could not parse boolean from a string: {}",
                            string
                        )))
                    }
                }
            }
        }
        AnythingOrBoolOrNull::Null => Ok(None),
    }
}
