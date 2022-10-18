use std::marker::PhantomData;

use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use crate::{BatchQuery, QueryContext};

pub trait SelectType {
    // TODO: ModelActions
    type Data: DeserializeOwned;
    type ModelData;

    fn to_selections(self) -> Vec<Selection>;
}

pub struct Select<'a, Data: DeserializeOwned> {
    operation: Operation,
    ctx: QueryContext<'a>,
    _data: PhantomData<Data>,
}

impl<'a, Data: DeserializeOwned> Select<'a, Data> {
    pub fn new(ctx: QueryContext<'a>, operation: Operation) -> Self {
        Self {
            ctx,
            operation,
            _data: PhantomData {},
        }
    }

    pub async fn exec(self) -> super::Result<Data> {
        self.ctx.execute(self.operation).await
    }
}

impl<'a, Data: DeserializeOwned> BatchQuery for Select<'a, Data> {
    type RawType = Data;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.operation
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
