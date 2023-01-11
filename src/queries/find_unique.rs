use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{
    Include, IncludeType, ModelOperation, ModelQuery, ModelReadOperation, ModelTypes,
    PrismaClientInternals, Query, QueryConvert, Select, SelectType, WhereInput, WithQuery,
};

pub struct FindUnique<'a, Actions: ModelTypes> {
    client: &'a PrismaClientInternals,
    pub where_param: Actions::Where,
    pub with_params: Vec<Actions::With>,
    _data: PhantomData<(Actions::Set, Actions::Data)>,
}

impl<'a, Actions: ModelTypes> FindUnique<'a, Actions> {
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

    pub async fn exec(self) -> super::Result<Option<Actions::Data>> {
        super::exec(self).await
    }
}

impl<'a, Actions: ModelTypes> QueryConvert for FindUnique<'a, Actions> {
    type RawType = Option<Actions::Data>;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> Self::ReturnValue {
        raw
    }
}

impl<'a, Actions: ModelTypes> Query<'a> for FindUnique<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        let mut scalar_selections = Actions::scalar_selections();

        scalar_selections.extend(self.with_params.into_iter().map(Into::into));

        (
            Operation::Read(Self::to_selection(self.where_param, scalar_selections)),
            self.client,
        )
    }
}

impl<'a, Actions: ModelTypes> ModelQuery<'a> for FindUnique<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Read(ModelReadOperation::FindUnique);
}

impl<'a, Actions: ModelTypes> WithQuery<'a> for FindUnique<'a, Actions> {
    fn add_with(&mut self, param: impl Into<Actions::With>) {
        self.with_params.push(param.into());
    }
}

#[derive(Clone)]
pub struct UniqueArgs<Actions>
where
    Actions: ModelTypes,
{
    pub with_params: Vec<Actions::With>,
}

impl<Actions> UniqueArgs<Actions>
where
    Actions: ModelTypes,
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
