use std::{collections::HashMap, marker::PhantomData};

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    raw::{Raw, RawOperationData, RawPrismaValue},
    PrismaClientInternals, Query, QueryConvert, QueryError,
};

pub struct QueryRaw<'a, Data>
where
    Data: DeserializeOwned,
{
    client: &'a PrismaClientInternals,
    sql: String,
    params: Vec<Value>,
    _data: PhantomData<Data>,
}

impl<'a, Data> QueryRaw<'a, Data>
where
    Data: DeserializeOwned + 'static,
{
    pub fn new(client: &'a PrismaClientInternals, query: Raw, database: &'static str) -> Self {
        let (sql, params) = query.convert(database);

        Self {
            client,
            sql,
            params,
            _data: PhantomData,
        }
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
                let v = serde_value::to_value(&row)
                    .map_err(|e| e.to_string())
                    .map_err(QueryError::Deserialize)?;

                v.deserialize_into::<Data>()
                    .map_err(|e| e.to_string())
                    .map_err(QueryError::Deserialize)
            })
            .collect::<Result<_, _>>()
            .map_err(Into::into)
    }

    pub async fn exec(self) -> super::Result<Vec<Data>> {
        super::exec(self).await
    }
}

impl<'a, Data> QueryConvert for QueryRaw<'a, Data>
where
    Data: DeserializeOwned + 'static,
{
    type RawType = RawOperationData;
    type ReturnValue = Vec<Data>;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Self::convert(raw)
    }
}

impl<'a, Data> Query<'a> for QueryRaw<'a, Data>
where
    Data: DeserializeOwned + 'static,
{
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Selection::new(
                "queryRaw".to_string(),
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
