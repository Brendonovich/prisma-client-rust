use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::raw::Raw;

use super::QueryContext;

pub struct QueryRaw<'a, Data>
where
    Data: DeserializeOwned,
{
    ctx: QueryContext<'a>,
    sql: String,
    params: Vec<Value>,
    _data: PhantomData<Data>,
}

impl<'a, Data> QueryRaw<'a, Data>
where
    Data: DeserializeOwned,
{
    pub fn new(ctx: QueryContext<'a>, query: Raw, database: &'static str) -> Self {
        let (sql, params) = query.convert(database);

        Self {
            ctx,
            sql,
            params,
            _data: PhantomData,
        }
    }

    pub async fn exec(self) -> super::Result<Vec<Data>> {
        let Self {
            ctx, sql, params, ..
        } = self;

        let mut selection = Selection::builder("queryRaw".to_string());

        selection.push_argument("query", PrismaValue::String(sql));
        selection.push_argument(
            "parameters",
            PrismaValue::String(serde_json::to_string(&params).unwrap()),
        );

        let op = Operation::Write(selection.build());

        ctx.execute(op).await
    }
}
