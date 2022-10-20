use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, SelectionBuilder};

use crate::{
    include::{Include, IncludeType},
    select::{Select, SelectType},
    BatchQuery, ModelAction, ModelActionType, ModelActions, ModelQueryType, PrismaClientInternals,
    WhereInput,
};

pub struct FindUnique<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub where_param: Actions::Where,
    pub with_params: Vec<Actions::With>,
    _data: PhantomData<(Actions::Set, Actions::Data)>,
}

impl<'a, Actions> ModelAction for FindUnique<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Query(ModelQueryType::FindUnique);
}

impl<'a, Actions> FindUnique<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(client: &'a PrismaClientInternals, where_param: Actions::Where) -> Self {
        Self {
            client,
            where_param,
            with_params: vec![],
            _data: PhantomData,
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    fn to_selection(where_param: Actions::Where) -> SelectionBuilder {
        let mut selection = Self::base_selection();

        selection.push_argument(
            "where",
            PrismaValue::Object(vec![where_param.serialize().transform_equals()]),
        );

        selection
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, Option<S::Data>> {
        let mut selection = Self::to_selection(self.where_param);

        selection.nested_selections(select.to_selections());

        let op = Operation::Read(selection.build());

        Select::new(self.client, op)
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        include: I,
    ) -> Include<'a, Option<I::Data>> {
        let mut selection = Self::to_selection(self.where_param);

        selection.nested_selections(include.to_selections());

        let op = Operation::Read(selection.build());

        Include::new(self.client, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut selection = Self::to_selection(self.where_param);
        let mut scalar_selections = Actions::scalar_selections();

        if self.with_params.len() > 0 {
            scalar_selections.append(&mut self.with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        (Operation::Read(selection.build()), self.client)
    }

    pub async fn exec(self) -> super::Result<Option<Actions::Data>> {
        let (op, client) = self.exec_operation();

        client.notify_model_action::<Self>();
        client.execute(op).await
    }
}

impl<'a, Actions> BatchQuery for FindUnique<'a, Actions>
where
    Actions: ModelActions,
{
    type RawType = Option<Actions::Data>;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}

#[derive(Clone)]
pub struct UniqueArgs<Actions>
where
    Actions: ModelActions,
{
    pub with_params: Vec<Actions::With>,
}

impl<Actions> UniqueArgs<Actions>
where
    Actions: ModelActions,
{
    pub fn new() -> Self {
        Self {
            with_params: vec![],
        }
    }

    pub fn with(mut self, with: impl Into<Actions::With>) -> Self {
        self.with_params.push(with.into());
        self
    }
}
