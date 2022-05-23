use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use crate::option_on_not_found;

use super::{transform_equals, QueryContext, QueryInfo, SerializedWhere};

pub struct Update<'a, Where, With, Set, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    Set: Into<(String, PrismaValue)>,
    Data: DeserializeOwned,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_param: Where,
    pub set_params: Vec<Set>,
    pub with_params: Vec<With>,
    _data: PhantomData<Data>,
}
impl<'a, Where, With, Set, Data> Update<'a, Where, With, Set, Data>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, PrismaValue)>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    pub fn new(
        ctx: QueryContext<'a>,
        info: QueryInfo,
        where_param: Where,
        set_params: Vec<Set>,
        with_params: Vec<With>,
    ) -> Self {
        Self {
            ctx,
            info,
            where_param,
            set_params,
            with_params,
            _data: PhantomData,
        }
    }

    pub fn with(mut self, param: impl Into<With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    pub async fn exec(self) -> super::Result<Option<Data>> {
        let Self {
            ctx,
            info,
            where_param,
            set_params,
            with_params,
            ..
        } = self;

        let QueryInfo {
            model,
            mut scalar_selections,
        } = info;

        let mut selection = Selection::builder(format!("updateOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            PrismaValue::Object(transform_equals(vec![where_param.into()].into_iter())),
        );

        selection.push_argument(
            "data",
            PrismaValue::Object(set_params.into_iter().map(Into::into).collect()),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        let op = Operation::Write(selection.build());

        option_on_not_found(ctx.execute(op).await)
    }
}
