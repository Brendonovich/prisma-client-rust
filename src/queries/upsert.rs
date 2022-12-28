use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{
    include::{Include, IncludeType},
    select::{Select, SelectType},
    BatchQuery, ModelAction, ModelActionType, ModelActions, ModelMutationType,
    PrismaClientInternals, WhereInput,
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

impl<'a, Actions> ModelAction for Upsert<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Mutation(ModelMutationType::Upsert);
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
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [
                (
                    "where".to_string(),
                    PrismaValue::Object(vec![where_param.serialize().transform_equals()]).into(),
                ),
                (
                    "create".to_string(),
                    PrismaValue::Object(create_params.into_iter().map(Into::into).collect()).into(),
                ),
                (
                    "update".to_string(),
                    PrismaValue::Object(update_params.into_iter().map(Into::into).collect()).into(),
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

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
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

    pub async fn exec(self) -> super::Result<Actions::Data> {
        let (op, client) = self.exec_operation();

        let res = client.execute(op).await?;
        client.notify_model_mutation::<Self>();

        Ok(res)
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
