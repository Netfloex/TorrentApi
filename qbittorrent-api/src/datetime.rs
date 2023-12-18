use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::{Deserializer, Serializer};

pub fn serialize<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.to_rfc3339())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i64::deserialize(deserializer)?;
    Ok(DateTime::from_timestamp(s, 0).unwrap_or_default())
}
