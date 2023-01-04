use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{
    merge_fields, Include, IncludeType, ModelActions, ModelOperation, ModelQuery,
    ModelWriteOperation, PrismaClientInternals, Query, QueryConvert, Select, SelectType, SetQuery,
    WhereInput, WithQuery,
};

pub struct Update<'a, Actions: ModelActions> {
    client: &'a PrismaClientInternals,
    pub where_param: Actions::Where,
    pub set_params: Vec<Actions::Set>,
    pub with_params: Vec<Actions::With>,
}

impl<'a, Actions: ModelActions> Update<'a, Actions> {
    pub fn new(
        client: &'a PrismaClientInternals,
        where_param: Actions::Where,
        set_params: Vec<Actions::Set>,
        with_params: Vec<Actions::With>,
    ) -> Self {
        Self {
            client,
            where_param,
            set_params,
            with_params,
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    fn to_selection(
        where_param: Actions::Where,
        set_params: Vec<Actions::Set>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [
                (
                    "where".to_string(),
                    PrismaValue::Object(vec![where_param.serialize().transform_equals()]).into(),
                ),
                (
                    "data".to_string(),
                    PrismaValue::Object(merge_fields(
                        set_params.into_iter().map(Into::into).collect(),
                    ))
                    .into(),
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
                self.set_params,
                select.to_selections(),
            )),
        )
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        include: I,
    ) -> Include<'a, I::Data> {
        Include::new(
            self.client,
            Operation::Write(Self::to_selection(
                self.where_param,
                self.set_params,
                include.to_selections(),
            )),
        )
    }

    pub async fn exec(self) -> super::Result<Actions::Data> {
        super::exec(self).await
    }
}

impl<'a, Actions: ModelActions> QueryConvert for Update<'a, Actions> {
    type RawType = Actions::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> Self::ReturnValue {
        raw
    }
}

impl<'a, Actions: ModelActions> Query<'a> for Update<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        let mut scalar_selections = Actions::scalar_selections();

        scalar_selections.extend(self.with_params.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(
                self.where_param,
                self.set_params,
                scalar_selections,
            )),
            self.client,
        )
    }
}

impl<'a, Actions: ModelActions> ModelQuery<'a> for Update<'a, Actions> {
    type Actions = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Update);
}

impl<'a, Actions: ModelActions> SetQuery<'a> for Update<'a, Actions> {
    fn add_set(&mut self, param: Actions::Set) {
        self.set_params.push(param);
    }
}

impl<'a, Actions: ModelActions> WithQuery<'a> for Update<'a, Actions> {
    fn add_with(&mut self, param: impl Into<Actions::With>) {
        self.with_params.push(param.into());
    }
}
