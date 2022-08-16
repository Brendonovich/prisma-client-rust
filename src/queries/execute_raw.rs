use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde_json::Value;

use crate::{raw::Raw, BatchQuery};

use super::QueryContext;

pub struct ExecuteRaw<'a> {
    ctx: QueryContext<'a>,
    sql: String,
    params: Vec<Value>,
}

impl<'a> ExecuteRaw<'a> {
    pub fn new(ctx: QueryContext<'a>, query: Raw, database: &'static str) -> Self {
        let (sql, params) = query.convert(database);

        Self { ctx, sql, params }
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Selection::builder("executeRaw".to_string());

        selection.push_argument("query", PrismaValue::String(self.sql));
        selection.push_argument(
            "parameters",
            PrismaValue::String(serde_json::to_string(&self.params).unwrap()),
        );

        (Operation::Write(selection.build()), self.ctx)
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

    fn convert(raw: super::Result<Self::RawType>) -> super::Result<Self::ReturnType> {
        raw
    }
}
