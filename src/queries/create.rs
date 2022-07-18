use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection, SelectionBuilder};
use serde::de::DeserializeOwned;

use crate::select::{Select, SelectType};

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

    fn to_selection(model: &str, set_params: Vec<Set>) -> SelectionBuilder {
        let mut selection = Selection::builder(format!("createOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "data",
            PrismaValue::Object(set_params.into_iter().map(Into::into).collect()),
        );

        selection
    }

    pub fn select<S: SelectType<Data>>(self, select: S) -> Select<'a, S::Data> {
        let mut selection = Self::to_selection(self.info.model, self.set_params);

        selection.nested_selections(select.to_selections());

        let op = Operation::Read(selection.build());

        Select::new(self.ctx, op)
    }

    pub async fn exec(self) -> super::Result<Data> {
        let QueryInfo { model, mut scalar_selections } = self.info;

        let mut selection = Self::to_selection(model, self.set_params);

        if self.with_params.len() > 0 {
            scalar_selections.append(&mut self.with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        let op = Operation::Write(selection.build());

        self.ctx.execute(op).await
    }
}
