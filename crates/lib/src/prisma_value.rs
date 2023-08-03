use std::sync::Arc;

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::{DateTime, FixedOffset};
use indexmap::IndexMap;
use query_core::response_ir::Item as PrismaItem;
use serde::{Serialize, Serializer};
use uuid::Uuid;

/// A Rust-friendly version of Prisma's own PrismaValue.
///
/// Prisma's PrismaValue has serialization overrides that make it suitable for JSON serialization,
/// but they lose some type information (eg. Bytes are encoded as base64), and can be less efficient
/// (eg. float values are encoded as strings).
///
/// This implementation only has an override for `PrismaValue::Null`, which is serialized as `None`
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum PrismaValue {
    String(String),
    Boolean(bool),
    Enum(String),
    Int(i32),
    Uuid(Uuid),
    List(Vec<PrismaValue>),
    Json(serde_json::Value),
    Object(Vec<(String, PrismaValue)>),
    #[serde(serialize_with = "serialize_null")]
    Null,
    DateTime(DateTime<FixedOffset>),
    Float(f64),
    BigInt(i64),
    Bytes(Vec<u8>),
}

/// A Rust-friendly version of Prisma's own Item.
/// Exists solely for nicer conversion of query results to our PrismaValue.
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum Item {
    Map(IndexMap<String, Item>),
    List(Vec<Item>),
    Value(PrismaValue),
    Json(serde_json::Value),
}

impl From<PrismaItem> for Item {
    fn from(item: PrismaItem) -> Self {
        match item {
            PrismaItem::Map(map) => {
                Item::Map(map.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            PrismaItem::List(list) => Item::List(list.into_iter().map(|v| v.into()).collect()),
            PrismaItem::Value(scalar) => Item::Value(scalar.into()),
            PrismaItem::Json(json) => Item::Json(json),
            PrismaItem::Ref(arc) => Arc::try_unwrap(arc)
                .unwrap_or_else(|arc| (*arc).to_owned())
                .into(),
        }
    }
}

fn serialize_null<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    Option::<()>::None.serialize(serializer)
}

impl From<prisma_models::PrismaValue> for PrismaValue {
    fn from(value: prisma_models::PrismaValue) -> Self {
        match value {
            prisma_models::PrismaValue::String(value) => Self::String(value),
            prisma_models::PrismaValue::Boolean(value) => Self::Boolean(value),
            prisma_models::PrismaValue::Enum(value) => Self::Enum(value),
            prisma_models::PrismaValue::Int(value) => Self::Int(value as i32),
            prisma_models::PrismaValue::Uuid(value) => Self::Uuid(value),
            prisma_models::PrismaValue::List(value) => {
                Self::List(value.into_iter().map(Into::into).collect())
            }
            prisma_models::PrismaValue::Json(value) => {
                Self::Json(serde_json::from_str(&value).unwrap())
            }
            prisma_models::PrismaValue::Object(value) => {
                Self::Object(value.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            prisma_models::PrismaValue::Null => Self::Null,
            prisma_models::PrismaValue::DateTime(value) => Self::DateTime(value),
            prisma_models::PrismaValue::Float(value) => Self::Float(value.to_f64().unwrap()),
            prisma_models::PrismaValue::BigInt(value) => Self::BigInt(value),
            prisma_models::PrismaValue::Bytes(value) => Self::Bytes(value),
        }
    }
}

impl From<PrismaValue> for prisma_models::PrismaValue {
    fn from(val: PrismaValue) -> Self {
        match val {
            PrismaValue::String(value) => Self::String(value),
            PrismaValue::Boolean(value) => Self::Boolean(value),
            PrismaValue::Enum(value) => Self::Enum(value),
            PrismaValue::Int(value) => Self::Int(value as i64),
            PrismaValue::Uuid(value) => Self::Uuid(value),
            PrismaValue::List(value) => Self::List(value.into_iter().map(Into::into).collect()),
            PrismaValue::Json(value) => Self::Json(serde_json::to_string(&value).unwrap()),
            PrismaValue::Object(value) => {
                Self::Object(value.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            PrismaValue::Null => Self::Null,
            PrismaValue::DateTime(value) => Self::DateTime(value),
            PrismaValue::Float(value) => Self::Float(BigDecimal::from_f64(value).unwrap()),
            PrismaValue::BigInt(value) => Self::BigInt(value),
            PrismaValue::Bytes(value) => Self::Bytes(value),
        }
    }
}
