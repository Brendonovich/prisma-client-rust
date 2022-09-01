use std::{collections::HashMap, marker::PhantomData};

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    raw::{Raw, RawOperationData, RawPrismaValue},
    BatchQuery,
};

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

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Selection::builder("queryRaw".to_string());

        selection.push_argument("query", PrismaValue::String(self.sql));
        selection.push_argument(
            "parameters",
            PrismaValue::String(serde_json::to_string(&self.params).unwrap()),
        );

        (Operation::Write(selection.build()), self.ctx)
    }

    pub(crate) fn convert(raw: RawOperationData) -> super::Result<Vec<Data>> {
        let typed_data: Vec<HashMap<String, RawPrismaValue>> = raw
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(column_name, cell)| (column_name, cell.into()))
                    .collect()
            })
            .collect();

        typed_data
            .into_iter()
            .map(|row| {
                let v = serde_value::to_value(&row).unwrap();
                v.deserialize_into::<Data>()
            })
            .collect::<Result<_, _>>()
            .map_err(Into::into)
    }

    pub async fn exec(self) -> super::Result<Vec<Data>> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await.and_then(Self::convert)
    }
}

impl<'a, Data> BatchQuery for QueryRaw<'a, Data>
where
    Data: DeserializeOwned,
{
    type RawType = RawOperationData;
    type ReturnType = Vec<Data>;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        Self::convert(raw).unwrap()
    }
}
