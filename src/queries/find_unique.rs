use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

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

    fn to_selection(
        where_param: Actions::Where,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [(
                "where".to_string(),
                PrismaValue::Object(vec![where_param.serialize().transform_equals()]).into(),
            )],
            nested_selections,
        )
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, Option<S::Data>> {
        Select::new(
            self.client,
            Operation::Read(Self::to_selection(self.where_param, select.to_selections())),
        )
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        include: I,
    ) -> Include<'a, Option<I::Data>> {
        Include::new(
            self.client,
            Operation::Read(Self::to_selection(
                self.where_param,
                include.to_selections(),
            )),
        )
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut scalar_selections = Actions::scalar_selections();

        scalar_selections.extend(self.with_params.into_iter().map(Into::into));

        (
            Operation::Read(Self::to_selection(self.where_param, scalar_selections)),
            self.client,
        )
    }

    pub async fn exec(self) -> super::Result<Option<Actions::Data>> {
        let (op, client) = self.exec_operation();

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
