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

    fn to_selection(model: &str, where_param: Where) -> SelectionBuilder {
        let mut selection = Selection::builder(format!("findUnique{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            PrismaValue::Object(vec![where_param.into().transform_equals()]),
        );

        selection
    }

    pub fn select<S: SelectType<ModelData = Data>>(self, select: S) -> Select<'a, Option<S::Data>> {
        let mut selection = Self::to_selection(self.info.model, self.where_param);

        selection.nested_selections(select.to_selections());

        let op = Operation::Read(selection.build());

        Select::new(self.ctx, op)
    }

    pub fn include<I: IncludeType<ModelData = Data>>(
        self,
        include: I,
    ) -> Include<'a, Option<I::Data>> {
        let mut selection = Self::to_selection(self.info.model, self.where_param);

        selection.nested_selections(include.to_selections());

        let op = Operation::Read(selection.build());

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

        (Operation::Read(selection.build()), self.ctx)
    }

    pub async fn exec(self) -> super::Result<Option<Data>> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await
    }
}

impl<'a, Where, With, Set, Data> BatchQuery for FindUnique<'a, Where, With, Set, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    Set: Into<(String, PrismaValue)>,
    Data: DeserializeOwned,
{
    type RawType = Option<Data>;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
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
