use prisma_models::PrismaValue;
use query_core::{Operation, SelectionBuilder};

use crate::{
    include::Include,
    select::{Select, SelectType},
    Action, BatchQuery, ModelActions, PrismaClientInternals,
};

pub struct Upsert<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub where_param: Actions::Where,
    pub create_params: Vec<Actions::Set>,
    pub update_params: Vec<Actions::Set>,
    pub with_params: Vec<Actions::With>,
}

impl<'a, Actions> Action for Upsert<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const NAME: &'static str = "upsertOne";
}

impl<'a, Actions> Upsert<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(
        client: &'a PrismaClientInternals,
        where_param: Actions::Where,
        create_params: Vec<Actions::Set>,
        update_params: Vec<Actions::Set>,
    ) -> Self {
        Self {
            client,
            where_param,
            create_params,
            update_params,
            with_params: vec![],
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    fn to_selection(
        where_param: Actions::Where,
        create_params: Vec<Actions::Set>,
        update_params: Vec<Actions::Set>,
    ) -> SelectionBuilder {
        let mut selection = Self::base_selection();

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

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, S::Data> {
        let mut selection =
            Self::to_selection(self.where_param, self.create_params, self.update_params);

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Select::new(self.client, op)
    }

    pub fn include<I: SelectType<ModelData = Actions::Data>>(
        self,
        select: I,
    ) -> Include<'a, I::Data> {
        let mut selection =
            Self::to_selection(self.where_param, self.create_params, self.update_params);

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Include::new(self.client, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut selection =
            Self::to_selection(self.where_param, self.create_params, self.update_params);
        let mut scalar_selections = Actions::scalar_selections();

        if self.with_params.len() > 0 {
            scalar_selections.append(&mut self.with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        (Operation::Write(selection.build()), self.client)
    }

    pub async fn exec(self) -> super::Result<Actions::Data> {
        let (op, client) = self.exec_operation();

        client.execute(op).await
    }
}

impl<'a, Actions> BatchQuery for Upsert<'a, Actions>
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
