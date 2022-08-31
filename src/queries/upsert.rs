use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection, SelectionBuilder};
use serde::de::DeserializeOwned;

use crate::{
    include::Include,
    select::{Select, SelectType},
    BatchQuery,
};

use super::{QueryContext, QueryInfo, SerializedWhere};

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

    fn to_selection(
        model: &str,
        where_param: Where,
        create_params: Vec<Set>,
        update_params: Vec<Set>,
    ) -> SelectionBuilder {
        let mut selection = Selection::builder(format!("upsertOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            PrismaValue::Object(vec![where_param.into().transform_equals()]),
        );

        selection.push_argument(
            "create",
            PrismaValue::Object(create_params.into_iter().map(Into::into).collect()),
        );

        selection.push_argument(
            "update",
            PrismaValue::Object(update_params.into_iter().map(Into::into).collect()),
        );

        selection
    }

    pub fn select<S: SelectType<ModelData = Data>>(self, select: S) -> Select<'a, S::Data> {
        let mut selection = Self::to_selection(
            self.info.model,
            self.where_param,
            self.create_params,
            self.update_params,
        );

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Select::new(self.ctx, op)
    }

    pub fn include<I: SelectType<ModelData = Data>>(self, select: I) -> Include<'a, I::Data> {
        let mut selection = Self::to_selection(
            self.info.model,
            self.where_param,
            self.create_params,
            self.update_params,
        );

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Include::new(self.ctx, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let QueryInfo {
            model,
            mut scalar_selections,
        } = self.info;

        let mut selection = Self::to_selection(
            model,
            self.where_param,
            self.create_params,
            self.update_params,
        );

        if self.with_params.len() > 0 {
            scalar_selections.append(&mut self.with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        (Operation::Write(selection.build()), self.ctx)
    }

    pub async fn exec(self) -> super::Result<Data> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await
    }
}

impl<'a, Where, Set, With, Data> BatchQuery for Upsert<'a, Where, Set, With, Data>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, PrismaValue)>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    type RawType = Data;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
