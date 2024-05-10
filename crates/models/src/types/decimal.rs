use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer};

pub fn deserialize_percentage<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum CurrencyOrNull<'a, T> {
        Str(&'a str),
        FromStr(T),
        Null,
    }

    match CurrencyOrNull::<T>::deserialize(deserializer)? {
        CurrencyOrNull::Str(s) => match s {
            "" => Ok(None),
            _ => {
                let s: String = s
                    .chars()
                    .filter(|&c| c.is_digit(10) || c == '.' || c == '-' || c == '+')
                    .collect();

                T::from_str(&s).map(Some).map_err(serde::de::Error::custom)
            }
        },
        CurrencyOrNull::FromStr(i) => Ok(Some(i)),
        CurrencyOrNull::Null => Ok(None),
    }
}
