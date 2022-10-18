use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde_json::Value;

use crate::{raw::Raw, BatchQuery, PrismaClientInternals};

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

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut selection = Selection::builder("executeRaw".to_string());

        selection.push_argument("query", PrismaValue::String(self.sql));
        selection.push_argument(
            "parameters",
            PrismaValue::String(serde_json::to_string(&self.params).unwrap()),
        );

        (Operation::Write(selection.build()), self.client)
    }

    pub async fn exec(self) -> super::Result<i64> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await
    }
}

impl<'a> BatchQuery for ExecuteRaw<'a> {
    type RawType = i64;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
