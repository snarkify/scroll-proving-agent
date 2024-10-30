use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    // The datetimes from the Snarkify API does not provide timezone information,
    // so we assume it is UTC.
    let s: Option<String> = Option::deserialize(deserializer)?;
    s.map(|s| {
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
            .map_err(serde::de::Error::custom)
    })
    .transpose()
}
