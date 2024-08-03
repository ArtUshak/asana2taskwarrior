//! Taskwarrior format types
use std::{collections::HashSet, fmt::Display};

use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use sscanf::scanf;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UUID {
    pub uuid: Uuid,
}

impl UUID {
    pub fn new(uuid: Uuid) -> Self {
        UUID { uuid }
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.uuid.as_bytes();
        write!(f, "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15])
    }
}

pub fn get_depends_fields(uuids: Vec<Uuid>) -> String {
    let uuid_strings_set: HashSet<String> = uuids
        .iter()
        .map(|&uuid| UUID::new(uuid).to_string())
        .collect();
    return uuid_strings_set.iter().join(",");
}

impl Serialize for UUID {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for UUID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UUIDVisitor {}

        impl<'de> Visitor<'de> for UUIDVisitor {
            type Value = UUID;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "UUID in format XXXX-XX-XX-XX-XXXXXX where X is hexadecimal digit (0-9 or a-f)"
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let parsed = scanf!(
                    v,
                    "{u8:x}{u8:x}{u8:x}{u8:x}-{u8:x}{u8:x}-{u8:x}{u8:x}-{u8:x}{u8:x}-{u8:x}{u8:x}{u8:x}{u8:x}{u8:x}{u8:x}"
                )
                .or_else(|_| {
                    Err(serde::de::Error::invalid_value(
                        Unexpected::Str("string in invalid format"),
                        &self,
                    ))
                })?;

                Ok(UUID::new(Uuid::from_bytes([
                    parsed.0, parsed.1, parsed.2, parsed.3, parsed.4, parsed.5, parsed.6, parsed.7,
                    parsed.8, parsed.9, parsed.10, parsed.11, parsed.12, parsed.13, parsed.14,
                    parsed.15,
                ])))
            }
        }

        deserializer.deserialize_str(UUIDVisitor {})
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub status: Status,
    pub uuid: UUID,
    pub entry: DateTime<Utc>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imask: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<UUID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "recurring")]
    Recurring,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Priority {
    #[serde(rename = "H")]
    High,
    #[serde(rename = "M")]
    Medium,
    #[serde(rename = "L")]
    Low,
}
