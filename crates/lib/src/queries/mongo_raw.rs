use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{ModelTypes, PrismaClientInternals, Query, QueryConvert, QueryError};

pub struct RunCommandRaw<'a, Data>
where
    Data: DeserializeOwned,
{
    client: &'a PrismaClientInternals,
    command: Value,
    _data: PhantomData<Data>,
}
impl<'a, Data> RunCommandRaw<'a, Data>
where
    Data: DeserializeOwned + 'static,
{
    pub fn new(client: &'a PrismaClientInternals, command: Value) -> Self {
        Self {
            client,
            command,
            _data: PhantomData,
        }
    }

    pub(crate) fn convert(raw: serde_json::Value) -> super::Result<Data> {
        serde_json::from_value(raw)
            .map_err(|e| e.to_string())
            .map_err(QueryError::Deserialize)
    }

    pub async fn exec(self) -> super::Result<Data> {
        super::exec(self).await
    }
}

impl<'a, Data> QueryConvert for RunCommandRaw<'a, Data>
where
    Data: DeserializeOwned + 'static,
{
    type RawType = serde_json::Value;
    type ReturnValue = Data;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Self::convert(raw)
    }
}

impl<'a, Data> Query<'a> for RunCommandRaw<'a, Data>
where
    Data: DeserializeOwned + 'static,
{
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Selection::new(
                "runCommandRaw",
                None,
                [(
                    "command".to_string(),
                    PrismaValue::try_from(self.command).unwrap().into(),
                )],
                [],
            )),
            self.client,
        )
    }
}

pub struct FindRaw<'a, Types, Data> {
    client: &'a PrismaClientInternals,
    filter: Option<Value>,
    options: Option<Value>,
    _data: PhantomData<(Data, Types)>,
}
impl<'a, Types, Data> FindRaw<'a, Types, Data>
where
    Types: ModelTypes,
    Data: DeserializeOwned + 'static,
{
    pub fn new(client: &'a PrismaClientInternals) -> Self {
        Self {
            client,
            filter: None,
            options: None,
            _data: PhantomData,
        }
    }

    pub fn filter(self, filter: Value) -> Self {
        Self {
            filter: Some(filter),
            ..self
        }
    }

    pub fn options(self, options: Value) -> Self {
        Self {
            options: Some(options),
            ..self
        }
    }

    pub async fn exec(self) -> super::Result<Data> {
        super::exec(self).await
    }
}

impl<'a, Types, Data> QueryConvert for FindRaw<'a, Types, Data>
where
    Data: DeserializeOwned + 'static,
{
    type RawType = serde_json::Value;
    type ReturnValue = Data;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        serde_json::from_value(raw)
            .map_err(|e| e.to_string())
            .map_err(QueryError::Deserialize)
    }
}

impl<'a, Types, Data> Query<'a> for FindRaw<'a, Types, Data>
where
    Types: ModelTypes,
    Data: DeserializeOwned + 'static,
{
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Read(Selection::new(
                format!("find{}Raw", Types::MODEL),
                None,
                [
                    self.filter.map(|filter| {
                        (
                            "filter".to_string(),
                            PrismaValue::try_from(filter).unwrap().into(),
                        )
                    }),
                    self.options.map(|options| {
                        (
                            "options".to_string(),
                            PrismaValue::try_from(options).unwrap().into(),
                        )
                    }),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
                [],
            )),
            self.client,
        )
    }
}

pub struct AggregateRaw<'a, Types, Data> {
    client: &'a PrismaClientInternals,
    pipeline: Option<Value>,
    options: Option<Value>,
    _data: PhantomData<(Data, Types)>,
}
impl<'a, Types, Data> AggregateRaw<'a, Types, Data>
where
    Types: ModelTypes,
    Data: DeserializeOwned + 'static,
{
    pub fn new(client: &'a PrismaClientInternals) -> Self {
        Self {
            client,
            pipeline: None,
            options: None,
            _data: PhantomData,
        }
    }

    pub fn pipeline(self, pipeline: Value) -> Self {
        Self {
            pipeline: Some(pipeline),
            ..self
        }
    }

    pub fn options(self, options: Value) -> Self {
        Self {
            options: Some(options),
            ..self
        }
    }

    pub async fn exec(self) -> super::Result<Data> {
        super::exec(self).await
    }
}

impl<'a, Types, Data> QueryConvert for AggregateRaw<'a, Types, Data>
where
    Data: DeserializeOwned + 'static,
{
    type RawType = serde_json::Value;
    type ReturnValue = Data;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        serde_json::from_value(raw)
            .map_err(|e| e.to_string())
            .map_err(QueryError::Deserialize)
    }
}

impl<'a, Types, Data> Query<'a> for AggregateRaw<'a, Types, Data>
where
    Types: ModelTypes,
    Data: DeserializeOwned + 'static,
{
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Read(Selection::new(
                format!("aggregate{}Raw", Types::MODEL),
                None,
                [
                    self.pipeline.map(|filter| {
                        (
                            "pipeline".to_string(),
                            PrismaValue::try_from(filter).unwrap().into(),
                        )
                    }),
                    self.options.map(|options| {
                        (
                            "options".to_string(),
                            PrismaValue::try_from(options).unwrap().into(),
                        )
                    }),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
                [],
            )),
            self.client,
        )
    }
}
