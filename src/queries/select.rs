use std::marker::PhantomData;

use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use crate::{option_on_not_found, QueryContext};

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
