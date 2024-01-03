use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{
    merge_fields, CreateModelTypes, Include, IncludeType, ModelOperation, ModelQuery, ModelTypes,
    ModelWriteOperation, PrismaClientInternals, Query, QueryConvert, Select, SelectType, SetQuery,
    WithQuery,
};

pub struct Create<'a, Types: CreateModelTypes> {
    client: &'a PrismaClientInternals,
    pub set_params: Vec<Types::Create>,
    pub with_params: Vec<Types::With>,
}

impl<'a, Actions: CreateModelTypes> Create<'a, Actions> {
    pub fn new(client: &'a PrismaClientInternals, set_params: Vec<Actions::Create>) -> Self {
        Self {
            client,
            set_params,
            with_params: vec![],
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    fn to_selection(
        set_params: Vec<Actions::Create>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [(
                "data".to_string(),
                PrismaValue::Object(merge_fields(
                    set_params.into_iter().map(Into::into).collect(),
                ))
                .into(),
            )]
            .into_iter(),
            nested_selections,
        )
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, S::Data> {
        Select::new(
            self.client,
            Operation::Write(Self::to_selection(self.set_params, select.to_selections())),
        )
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        include: I,
    ) -> Include<'a, I::Data> {
        Include::new(
            self.client,
            Operation::Write(Self::to_selection(self.set_params, include.to_selections())),
        )
    }

    pub async fn exec(self) -> super::Result<Actions::Data> {
        super::exec(self).await
    }
}

impl<'a, Types: CreateModelTypes> QueryConvert for Create<'a, Types> {
    type RawType = Types::Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw)
    }
}

impl<'a, Types: CreateModelTypes> Query<'a> for Create<'a, Types> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        let mut scalar_selections = Types::scalar_selections();

        scalar_selections.extend(self.with_params.into_iter().map(Into::into));

        (
            Operation::Write(Self::to_selection(self.set_params, scalar_selections)),
            self.client,
        )
    }
}

impl<'a, Types: CreateModelTypes> ModelQuery<'a> for Create<'a, Types> {
    type Types = Types;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::Create);
}

// impl<'a, Actions: CreateModelTypes> SetQuery<'a> for Create<'a, Actions> {
//     fn add_set(&mut self, param: Actions::Set) {
//         self.set_params.push(param);
//     }
// }

impl<'a, Types: CreateModelTypes> WithQuery<'a> for Create<'a, Types> {
    fn add_with(&mut self, param: impl Into<Types::With>) {
        self.with_params.push(param.into());
    }
}
