use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use super::{QueryContext, QueryInfo};

pub struct Create<'a, Set, With, Data>
where
    Set: Into<(String, PrismaValue)>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub set_params: Vec<Set>,
    pub with_params: Vec<With>,
    _data: PhantomData<Data>,
}

impl<'a, Set, With, Data> Create<'a, Set, With, Data>
where
    Set: Into<(String, PrismaValue)>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    pub fn new(ctx: QueryContext<'a>, info: QueryInfo, set_params: Vec<Set>) -> Self {
        Self {
            ctx,
            info,
            set_params,
            with_params: vec![],
            _data: PhantomData,
        }
    }

    pub fn with(mut self, param: impl Into<With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    pub async fn exec(self) -> super::Result<Data> {
        let Self {
            ctx,
            info,
            set_params,
            with_params,
            ..
        } = self;

        let QueryInfo {
            model,
            mut scalar_selections,
        } = info;

        let mut selection = Selection::builder(format!("createOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "data",
            PrismaValue::Object(set_params.into_iter().map(Into::into).collect()),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        let op = Operation::Write(selection.build());

        ctx.execute(op).await
    }
}
