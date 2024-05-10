use chrono::{DateTime, NaiveDateTime};
use chrono_tz::{America, Tz};
use serde::de::{Deserializer, Visitor};

pub type Timestamp = DateTime<Tz>;

pub struct TimestampVisitor;

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a str or string representing a timestamp in YYYY-mm-dd HH:MM:SS")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let date = NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S")
            .map_err(E::custom)?
            .and_local_timezone(America::Montreal)
            .unwrap();

        Ok(date)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.as_str())
    }
}

pub fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(TimestampVisitor)
}
