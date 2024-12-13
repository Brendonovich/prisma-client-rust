use std::collections::HashMap;

use crate::PrismaValue;
use chrono::SecondsFormat;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};
use std::str::FromStr;

#[macro_export]
macro_rules! raw {
    ($e: expr) => {
        $crate::Raw::new($e, vec![]);
    };
    ($e: expr, $($params:expr),+) => {
        $crate::Raw::new($e, vec![$($params),+])
    };
}

pub struct Raw {
    pub(crate) query: String,
    pub values: Vec<Value>,
}

impl Raw {
    pub fn new(query: &str, values: Vec<PrismaValue>) -> Self {
        Self {
            query: query.to_string(),
            values: values
                .into_iter()
                .map(|v| match v.into() {
                    prisma_models::PrismaValue::DateTime(dt) => json!({
                        "prisma__type": "date",
                        "prisma__value": dt.to_rfc3339_opts(SecondsFormat::Millis, true)
                    }),
                    v => serde_json::to_value(v).unwrap(),
                })
                .collect(),
        }
    }

    pub fn convert(self, database: &'static str) -> (String, Vec<Value>) {
        let Self { mut query, values } = self;

        for i in 1..=values.len() {
            let variable_indicator = match database {
                "postgresql" | "cockroachdb" => format!("${i}"),
                "sqlite" | "mysql" => "?".to_string(),
                _ => panic!("Raw queries are not supported with database '{database}'"),
            };

            query = query.replacen("{}", &variable_indicator, 1);
        }

        // TODO: do this at compile time
        if query.contains("{}") {
            panic!("Raw query has not been given enough parameters");
        }

        (query, values)
    }
}

pub type RawOperationData = Vec<HashMap<String, RawTypedJson>>;

#[derive(Deserialize)]
pub struct RawTypedJson {
    #[serde(rename = "prisma__type")]
    typ: String,
    #[serde(rename = "prisma__value")]
    value: serde_json::Value,
}

impl From<RawTypedJson> for RawPrismaValue {
    fn from(json: RawTypedJson) -> Self {
        use serde_json::Value::*;

        match (json.typ.as_str(), json.value) {
            ("int", Number(n)) => RawPrismaValue::Int(n.as_i64().unwrap() as i32),
            ("bigint", String(s)) => RawPrismaValue::BigInt(s.parse().unwrap()),
            ("float", Number(n)) => RawPrismaValue::Float(n.as_f64().unwrap() as f32),
            ("double", Number(n)) => RawPrismaValue::Double(n.as_f64().unwrap()),
            ("string", String(s)) => RawPrismaValue::String(s),
            ("enum", String(s)) => RawPrismaValue::Enum(s),
            ("bytes", String(b64)) => RawPrismaValue::Bytes(base64::decode(b64).unwrap()),
            ("bool", Bool(b)) => RawPrismaValue::Bool(b),
            ("char", String(s)) => RawPrismaValue::Char(s.chars().next().unwrap()),
            ("decimal", String(n)) => {
                RawPrismaValue::Decimal(bigdecimal::BigDecimal::from_str(n.as_str()).unwrap())
            }
            ("json", v) => RawPrismaValue::Json(v),
            ("xml", String(s)) => RawPrismaValue::Xml(s),
            ("uuid", String(s)) => RawPrismaValue::Uuid(uuid::Uuid::from_str(&s).unwrap()),
            ("datetime", String(s)) => {
                RawPrismaValue::DateTime(chrono::DateTime::parse_from_rfc3339(&s).unwrap().into())
            }
            ("date", String(s)) => {
                RawPrismaValue::Date(serde_json::from_slice(s.as_bytes()).unwrap())
            }
            ("time", String(s)) => {
                RawPrismaValue::Time(serde_json::from_slice(s.as_bytes()).unwrap())
            }
            ("array", Array(arr)) => RawPrismaValue::Array(
                arr.into_iter()
                    .map(serde_json::from_value::<RawTypedJson>)
                    .map(Result::unwrap)
                    .map(Into::into)
                    .collect(),
            ),
            ("null", _) => RawPrismaValue::Null,
            _ => unreachable!("Invalid value for raw type {}", &json.typ),
        }
    }
}

// See quaint::ast::Value & IntoTypedJsonExtension
#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum RawPrismaValue {
    Int(i32),
    BigInt(i64),
    Float(f32),
    Double(f64),
    String(String),
    Enum(String),
    Bytes(Vec<u8>),
    Bool(bool),
    Char(char),
    Decimal(bigdecimal::BigDecimal),
    Json(serde_json::Value),
    Xml(String),
    Uuid(uuid::Uuid),
    DateTime(chrono::DateTime<chrono::Utc>),
    Date(chrono::NaiveDate),
    Time(chrono::NaiveTime),
    Array(Vec<RawPrismaValue>),
    #[serde(serialize_with = "serialize_null")]
    Null,
}

fn serialize_null<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    Option::<()>::None.serialize(serializer)
}
