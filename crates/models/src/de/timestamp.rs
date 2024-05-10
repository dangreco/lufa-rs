use chrono::{DateTime, NaiveDate, NaiveDateTime};
use chrono_tz::{America, Tz};
use serde::de::{Deserializer, Visitor};

macro_rules! timestamp {
    ($i:ident, $iv:ident, $t:ty, $pattern:literal, $parse:expr, $de:ident) => {
        pub type $i = $t;
        pub struct $iv;

        impl<'de> Visitor<'de> for $iv {
            type Value = $i;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(&format!(
                    "a str/string representing a timestamp in the format: {}",
                    $pattern
                ))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                $parse(v)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(v.as_str())
            }
        }

        pub fn $de<'de, D>(deserializer: D) -> Result<$i, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any($iv)
        }
    };
}

timestamp!(
    Timestamp,
    TimestampVisitor,
    DateTime<Tz>,
    "YYYY-mm-dd HH:MM:SS",
    |s: &str| {
        let date = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .map_err(E::custom)?
            .and_local_timezone(America::Montreal)
            .unwrap();

        Ok(date)
    },
    deserialize_timestamp
);

timestamp!(
    Date,
    DateVisitor,
    NaiveDate,
    "YYYY-mm-dd or Month dd, YYYY",
    |s: &str| {
        let pattern = if s.chars().next().unwrap().is_alphabetic() {
            "%B %d, %Y"
        } else {
            "%Y-%m-%d"
        };

        NaiveDate::parse_from_str(s, pattern).map_err(E::custom)
    },
    deserialize_date
);
