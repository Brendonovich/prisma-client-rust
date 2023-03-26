use query_core::{Operation, Selection};
use std::marker::PhantomData;

use crate::{PrismaClientInternals, Query, QueryConvert};

use super::query;

pub trait IncludeType {
    // TODO: ModelActions
    type Data: query::Data;
    type ModelData;

    fn to_selections(self) -> Vec<Selection>;
}

pub struct Include<'a, Data> {
    operation: Operation,
    client: &'a PrismaClientInternals,
    _data: PhantomData<Data>,
}

impl<'a, Data: query::Data> Include<'a, Data> {
    pub fn new(client: &'a PrismaClientInternals, operation: Operation) -> Self {
        Self {
            client,
            operation,
            _data: PhantomData {},
        }
    }

    pub async fn exec(self) -> super::Result<Data> {
        super::exec(self).await
    }
}

impl<'a, Data: query::Data> QueryConvert for Include<'a, Data> {
    type RawType = Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'a, Data: query::Data> Query<'a> for Include<'a, Data> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (self.operation, self.client)
    }
}
