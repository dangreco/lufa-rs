use chrono::{DateTime, NaiveDate, NaiveDateTime};
use chrono_tz::{America, Tz};
use serde::{de::Deserializer, Deserialize};

pub fn date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    match date_optional(deserializer) {
        Ok(Some(d)) => Ok(d),
        Ok(None) => Err(serde::de::Error::custom("expected string, found null")),
        Err(e) => Err(e),
    }
}

pub fn date_optional<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum DateOrNull {
        String(String),
        Null,
    }

    match DateOrNull::deserialize(deserializer)? {
        DateOrNull::String(s) => {
            let pattern = if s.chars().next().unwrap().is_alphabetic() {
                "%B %d, %Y"
            } else {
                "%Y-%m-%d"
            };

            NaiveDate::parse_from_str(&s, pattern)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        DateOrNull::Null => Ok(None),
    }
}

pub fn timestamp<'de, D>(deserializer: D) -> Result<DateTime<Tz>, D::Error>
where
    D: Deserializer<'de>,
{
    match timestamp_optional(deserializer) {
        Ok(Some(t)) => Ok(t),
        Ok(None) => Err(serde::de::Error::custom("expected string, found null")),
        Err(e) => Err(e),
    }
}

pub fn timestamp_optional<'de, D>(deserializer: D) -> Result<Option<DateTime<Tz>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TimestampOrNull {
        String(String),
        Null,
    }

    match TimestampOrNull::deserialize(deserializer)? {
        TimestampOrNull::String(s) => {
            let timestamp = s.as_str();

            regexm::regexm!(match timestamp {
                r#"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$"# => {
                    let naive = match timestamp {
                        "0000-00-00 00:00:00" => NaiveDateTime::MIN,
                        t => NaiveDateTime::parse_from_str(t, "%Y-%m-%d %H:%M:%S")
                            .map_err(serde::de::Error::custom)?,
                    };

                    let datetime = naive.and_local_timezone(America::Montreal).unwrap();

                    Ok(Some(datetime))
                }
                _ => Err(serde::de::Error::custom("invalid format")),
            })
        }
        TimestampOrNull::Null => Ok(None),
    }
}
