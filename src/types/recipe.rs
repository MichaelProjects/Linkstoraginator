use serde::{Deserialize, Serialize};
extern crate chrono;
use bson::oid::ObjectId;
use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    #[serde(
        rename(deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_object_id"
    )]
    pub id: Option<String>,
    pub url: String,
    pub preview_image: Option<String>,
    pub title: Option<String>,
    pub added: DateTime<Utc>,
}
fn deserialize_option_object_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<ObjectId>::deserialize(deserializer)?;
    Ok(opt.map(|oid| oid.to_hex()))
}
