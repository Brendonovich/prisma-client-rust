use std::marker::PhantomData;

use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use crate::{option_on_not_found, BatchQuery, QueryContext};

pub trait SelectType<T> {
    type Data: DeserializeOwned;

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

    fn convert(raw: super::Result<Self::RawType>) -> super::Result<Self::ReturnType> {
        raw
    }
}

pub struct SelectOption<'a, Data: DeserializeOwned> {
    select: Select<'a, Data>,
}

impl<'a, T: DeserializeOwned> SelectOption<'a, T> {
    pub fn new(ctx: QueryContext<'a>, operation: Operation) -> Self {
        Self {
            select: Select::new(ctx, operation),
        }
    }

    pub async fn exec(self) -> super::Result<Option<T>> {
        option_on_not_found(self.select.exec().await)
    }
}

impl<'a, Data: DeserializeOwned> BatchQuery for SelectOption<'a, Data> {
    type RawType = Option<Data>;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.select.operation
    }

    fn convert(raw: super::Result<Self::RawType>) -> super::Result<Self::ReturnType> {
        raw
    }
}
