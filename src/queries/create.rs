use query_core::{Operation, SelectionBuilder};

use crate::{
    include::{Include, IncludeType},
    merged_object,
    select::{Select, SelectType},
    Action, BatchQuery, ModelActions,
};

use super::QueryContext;

pub struct Create<'a, Actions>
where
    Actions: ModelActions,
{
    ctx: QueryContext<'a>,
    pub set_params: Vec<Actions::Set>,
    pub with_params: Vec<Actions::With>,
}

impl<'a, Actions> Action for Create<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const NAME: &'static str = "createOne";
}

impl<'a, Actions> Create<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(ctx: QueryContext<'a>, set_params: Vec<Actions::Set>) -> Self {
        Self {
            ctx,
            set_params,
            with_params: vec![],
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    fn to_selection(set_params: Vec<Actions::Set>) -> SelectionBuilder {
        let mut selection = Self::base_selection();

        selection.push_argument(
            "data",
            merged_object(set_params.into_iter().map(Into::into).collect()),
        );

        selection
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, S::Data> {
        let mut selection = Self::to_selection(self.set_params);

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Select::new(self.ctx, op)
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        include: I,
    ) -> Include<'a, I::Data> {
        let mut selection = Self::to_selection(self.set_params);

        selection.nested_selections(include.to_selections());

        let op = Operation::Write(selection.build());

        Include::new(self.ctx, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Self::to_selection(self.set_params);
        let mut scalar_selections = Actions::scalar_selections();

        if self.with_params.len() > 0 {
            scalar_selections.append(&mut self.with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        (Operation::Write(selection.build()), self.ctx)
    }

    pub async fn exec(self) -> super::Result<Actions::Data> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await
    }
}

impl<'a, Actions> BatchQuery for Create<'a, Actions>
where
    Actions: ModelActions,
{
    type RawType = Actions::Data;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
