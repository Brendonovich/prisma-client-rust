use prisma_models::PrismaValue;
use query_core::{Operation, SelectionBuilder};

use crate::{
    include::{Include, IncludeType},
    select::{Select, SelectType},
    BatchQuery, ModelAction, ModelActionType, ModelActions, ModelMutationType,
    PrismaClientInternals, WhereInput,
};

pub struct Delete<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub where_param: Actions::Where,
    pub with_params: Vec<Actions::With>,
}

impl<'a, Actions> ModelAction for Delete<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Mutation(ModelMutationType::Delete);
}

impl<'a, Actions> Delete<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(
        client: &'a PrismaClientInternals,
        where_param: Actions::Where,
        with_params: Vec<Actions::With>,
    ) -> Self {
        Self {
            client,
            where_param,
            with_params,
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
    ) -> Select<'a, S::Data> {
        let mut selection = Self::to_selection(self.where_param);

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Select::new(self.client, op)
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        select: I,
    ) -> Include<'a, I::Data> {
        let mut selection = Self::to_selection(self.where_param);

        selection.nested_selections(select.to_selections());

        let op = Operation::Write(selection.build());

        Include::new(self.client, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut selection = Self::to_selection(self.where_param);
        let mut scalar_selections = Actions::scalar_selections();

        if self.with_params.len() > 0 {
            scalar_selections.append(&mut self.with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        (Operation::Write(selection.build()), self.client)
    }

    pub async fn exec(self) -> super::Result<Actions::Data> {
        let (op, client) = self.exec_operation();

        let res = client.execute(op).await?;

        #[cfg(feature = "mutation-callbacks")]
        client.notify_model_mutation::<Self>();

        Ok(res)
    }
}

impl<'a, Actions> BatchQuery for Delete<'a, Actions>
where
    Actions: ModelActions,
{
    type RawType = Actions::Data;
    type ReturnType = Actions::Data;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
