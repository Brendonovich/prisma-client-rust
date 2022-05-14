use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use super::{transform_equals, QueryContext, QueryInfo, SerializedWhere};

pub struct Upsert<'a, Where, Set, With, Data>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, PrismaValue)>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_param: Where,
    pub create_params: Vec<Set>,
    pub update_params: Vec<Set>,
    pub with_params: Vec<With>,
    _data: PhantomData<Data>,
}

impl<'a, Where, Set, With, Data> Upsert<'a, Where, Set, With, Data>
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
        create_params: Vec<Set>,
        update_params: Vec<Set>,
    ) -> Self {
        Self {
            ctx,
            info,
            where_param,
            create_params,
            update_params,
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
            where_param,
            create_params,
            update_params,
            with_params,
            ..
        } = self;

        let QueryInfo {
            model,
            mut scalar_selections,
        } = info;

        let mut selection = Selection::builder(format!("upsertOne{}", model));

        selection.alias("result");

        if create_params.len() > 0 {
            selection.push_argument(
                "create",
                PrismaValue::Object(create_params.into_iter().map(Into::into).collect()),
            );
        }

        if update_params.len() > 0 {
            selection.push_argument(
                "update",
                PrismaValue::Object(update_params.into_iter().map(Into::into).collect()),
            );
        }

        selection.push_argument(
            "where",
            PrismaValue::Object(transform_equals(vec![where_param.into()])),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        let op = Operation::Write(selection.build());

        ctx.execute(op).await
    }
}
