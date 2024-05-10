use std::borrow::Cow;

use chrono::NaiveDate;
use serde::{de::Error, Deserialize, Deserializer};

pub fn naive_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
    let with_date = s.to_string() + "/01";
    let date =
        NaiveDate::parse_from_str(with_date.as_str(), "%m/%y/%d").map_err(D::Error::custom)?;
    Ok(date)
}

pub fn deserialize_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
    str::parse::<usize>(&s).map_err(D::Error::custom)
}