use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde_json::Value;

use crate::raw::Raw;

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

    pub async fn exec(self) -> super::Result<i64> {
        let Self {
            ctx, sql, params, ..
        } = self;

        let mut selection = Selection::builder("executeRaw".to_string());

        selection.push_argument("query", PrismaValue::String(sql));
        selection.push_argument(
            "parameters",
            PrismaValue::String(serde_json::to_string(&params).unwrap()),
        );

        let op = Operation::Write(selection.build());

        ctx.execute(op).await
    }
}
