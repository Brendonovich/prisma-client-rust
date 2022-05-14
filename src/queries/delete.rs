use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation,  Selection};
use serde::de::DeserializeOwned;

use super::{option_on_not_found, transform_equals, QueryContext, QueryInfo, SerializedWhere};

pub struct Delete<'a, Where, With, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_param: Where,
    pub with_params: Vec<With>,
    _data: PhantomData<Data>,
}
impl<'a, Where, With, Data> Delete<'a, Where, With, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    pub fn new(
        ctx: QueryContext<'a>,
        info: QueryInfo,
        where_param: Where,
        with_params: Vec<With>,
    ) -> Self {
        Self {
            ctx,
            info,
            where_param,
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
            with_params,
            ..
        } = self;

        let QueryInfo {
            model,
            mut scalar_selections,
        } = info;

        let mut selection = Selection::builder(format!("deleteOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            PrismaValue::Object(transform_equals(vec![where_param.into()])),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        let op = Operation::Write(selection.build());

        option_on_not_found(ctx.execute(op).await)
    }
}
