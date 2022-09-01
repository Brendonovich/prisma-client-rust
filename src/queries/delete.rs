use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection, SelectionBuilder};
use serde::de::DeserializeOwned;

use crate::{
    include::{Include, IncludeType},
    select::{Select, SelectType},
    BatchQuery,
};

use super::{QueryContext, QueryInfo, SerializedWhere};

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

    fn to_selection(model: &str, where_param: Where) -> SelectionBuilder {
        let mut selection = Selection::builder(format!("deleteOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            PrismaValue::Object(vec![where_param.into().transform_equals()]),
        );

        selection
    }

    pub fn select<S: SelectType<ModelData = Data>>(self, select: S) -> Select<'a, S::Data> {
        let mut selection = Self::to_selection(self.info.model, self.where_param);

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Select::new(self.ctx, op)
    }

    pub fn include<I: IncludeType<ModelData = Data>>(self, select: I) -> Include<'a, I::Data> {
        let mut selection = Self::to_selection(self.info.model, self.where_param);

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Include::new(self.ctx, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let QueryInfo {
            model,
            mut scalar_selections,
        } = self.info;

        let mut selection = Self::to_selection(model, self.where_param);

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

impl<'a, Where, With, Data> BatchQuery for Delete<'a, Where, With, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    Data: DeserializeOwned,
{
    type RawType = Data;
    type ReturnType = Data;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
