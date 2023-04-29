use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde_json::Value;

use crate::{raw::Raw, PrismaClientInternals, Query, QueryConvert};

pub struct ExecuteRaw<'a> {
    client: &'a PrismaClientInternals,
    sql: String,
    params: Vec<Value>,
}

impl<'a> ExecuteRaw<'a> {
    pub fn new(client: &'a PrismaClientInternals, query: Raw, database: &'static str) -> Self {
        let (sql, params) = query.convert(database);

        Self {
            client,
            sql,
            params,
        }
    }

    pub async fn exec(self) -> super::Result<i64> {
        super::exec(self).await
    }
}

impl<'a> QueryConvert for ExecuteRaw<'a> {
    type RawType = i64;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'a> Query<'a> for ExecuteRaw<'a> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Selection::new(
                "executeRaw".to_string(),
                None,
                [
                    ("query".to_string(), PrismaValue::String(self.sql).into()),
                    (
                        "parameters".to_string(),
                        PrismaValue::String(serde_json::to_string(&self.params).unwrap()).into(),
                    ),
                ],
                [],
            )),
            self.client,
        )
    }
}
