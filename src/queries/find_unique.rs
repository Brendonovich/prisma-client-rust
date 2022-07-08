use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use super::{delete::Delete, QueryContext, QueryInfo, SerializedWhere, Update};

pub struct FindUnique<'a, Where, With, Set, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    Set: Into<(String, PrismaValue)>,
    Data: DeserializeOwned,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_param: Where,
    pub with_params: Vec<With>,
    _data: PhantomData<(Set, Data)>,
}

impl<'a, Where, With, Set, Data> FindUnique<'a, Where, With, Set, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    Set: Into<(String, PrismaValue)>,
    Data: DeserializeOwned,
{
    pub fn new(ctx: QueryContext<'a>, info: QueryInfo, where_param: Where) -> Self {
        Self {
            ctx,
            info,
            where_param,
            with_params: vec![],
            _data: PhantomData,
        }
    }

    pub fn with(mut self, param: impl Into<With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    pub fn update(self, params: Vec<Set>) -> Update<'a, Where, With, Set, Data> {
        let Self {
            ctx,
            info,
            where_param,
            with_params,
            ..
        } = self;

        Update::new(ctx, info, where_param, params, with_params)
    }
    
    pub fn delete(self) -> Delete<'a, Where, With, Data> {
        let Self {
            ctx,
            info,
            where_param,
            with_params,
            ..
        } = self;

        Delete::new(ctx, info, where_param, with_params)
    }

    pub async fn exec(self) -> super::Result<Option<Data>> {
        let Self {
            ctx,
            info,
            where_param,
            with_params,
            ..
        } = self;

        let QueryInfo {
            model,
            mut scalar_selections,
        } = info;

        let mut selection = Selection::builder(format!("findUnique{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            PrismaValue::Object(vec![where_param.into().transform_equals()]),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        let op = Operation::Read(selection.build());

        ctx.execute(op).await
    }
}

#[derive(Clone)]
pub struct UniqueArgs<With>
where
    With: Into<Selection>,
{
    pub with_params: Vec<With>,
}

impl<With> UniqueArgs<With>
where
    With: Into<Selection>,
{
    pub fn new() -> Self {
        Self {
            with_params: vec![],
        }
    }

    pub fn with(mut self, with: impl Into<With>) -> Self {
        self.with_params.push(with.into());
        self
    }
}
