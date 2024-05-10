use query_core::{Operation, Selection};

use crate::{
    Include, IncludeType, ModelOperation, ModelQuery, ModelTypes, ModelWriteOperation,
    PrismaClientInternals, PrismaValue, Query, QueryConvert, Select, SelectType, WhereInput,
    WithQuery,
};

pub struct Upsert<'a, Actions: ModelTypes> {
    client: &'a PrismaClientInternals,
    pub where_param: Actions::WhereUnique,
    pub create_params: Vec<Actions::Set>,
    pub update_params: Vec<Actions::Set>,
    pub with_params: Vec<Actions::With>,
}

impl<'a, Actions: ModelTypes> Upsert<'a, Actions> {
    pub fn new(
        client: &'a PrismaClientInternals,
        where_param: Actions::WhereUnique,
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
        where_param: Actions::WhereUnique,
        create_params: Vec<Actions::Set>,
        update_params: Vec<Actions::Set>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [
                (
                    "where".to_string(),
                    PrismaValue::Object(vec![where_param.serialize().transform_equals()]),
                ),
                (
                    "create".to_string(),
                    PrismaValue::Object(create_params.into_iter().map(Into::into).collect()),
                ),
                (
                    "update".to_string(),
                    PrismaValue::Object(update_params.into_iter().map(Into::into).collect()),
                ),
            ],
            nested_selections,
        )
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, S::Data> {
        Select::new(
            self.client,
            Operation::Write(Self::to_selection(
                self.where_param,
                self.create_params,
                self.update_params,
                select.to_selections(),
            )),
        )
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        select: I,
    ) -> Include<'a, I::Data> {
        Include::new(
            self.client,
            Operation::Write(Self::to_selection(
                self.where_param,
                self.create_params,
                self.update_params,
                select.to_selections(),
            )),
        )
    }

    pub async fn exec(self) -> super::Result<Actions::Data> {
        super::exec(self).await
    }
}

impl<'a, Actions: ModelTypes> QueryConvert for Upsert<'a, Actions> {
    type RawType = Actions::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'a, Actions: ModelTypes> Query<'a> for Upsert<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        let mut scalar_selections = Actions::scalar_selections();

        scalar_selections.extend(self.with_params.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(
                self.where_param,
                self.create_params,
                self.update_params,
                scalar_selections,
            )),
            self.client,
        )
    }
}

impl<'a, Actions: ModelTypes> ModelQuery<'a> for Upsert<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Upsert);
}

impl<'a, Actions: ModelTypes> WithQuery<'a> for Upsert<'a, Actions> {
    fn add_with(&mut self, param: impl Into<Actions::With>) {
        self.with_params.push(param.into());
    }
}
