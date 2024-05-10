use query_core::{Operation, Selection};

use crate::{
    merge_fields, Include, IncludeType, ModelOperation, ModelQuery, ModelTypes,
    ModelWriteOperation, PrismaClientInternals, PrismaValue, Query, QueryConvert, Select,
    SelectType, SetQuery, WhereInput, WithQuery,
};

pub struct Update<'a, Actions: ModelTypes> {
    client: &'a PrismaClientInternals,
    pub where_param: Actions::WhereUnique,
    pub set_params: Vec<Actions::Set>,
    pub with_params: Vec<Actions::With>,
}

impl<'a, Actions: ModelTypes> Update<'a, Actions> {
    pub fn new(
        client: &'a PrismaClientInternals,
        where_param: Actions::WhereUnique,
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
        where_param: Actions::WhereUnique,
        set_params: Vec<Actions::Set>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [
                (
                    "where".to_string(),
                    PrismaValue::Object(vec![where_param.serialize().transform_equals()]),
                ),
                (
                    "data".to_string(),
                    PrismaValue::Object(merge_fields(
                        set_params.into_iter().map(Into::into).collect(),
                    )),
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

impl<'a, Actions: ModelTypes> QueryConvert for Update<'a, Actions> {
    type RawType = Actions::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'a, Actions: ModelTypes> Query<'a> for Update<'a, Actions> {
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

impl<'a, Actions: ModelTypes> ModelQuery<'a> for Update<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Update);
}

impl<'a, Actions: ModelTypes> SetQuery<'a> for Update<'a, Actions> {
    fn add_set(&mut self, param: Actions::Set) {
        self.set_params.push(param);
    }
}

impl<'a, Actions: ModelTypes> WithQuery<'a> for Update<'a, Actions> {
    fn add_with(&mut self, param: impl Into<Actions::With>) {
        self.with_params.push(param.into());
    }
}
