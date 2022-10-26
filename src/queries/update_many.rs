use prisma_models::PrismaValue;
use query_core::Operation;

use crate::{
    merge_fields, BatchQuery, BatchResult, ModelAction, ModelActionType, ModelActions,
    ModelMutationType, PrismaClientInternals, WhereInput,
};

pub struct UpdateMany<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
    pub set_params: Vec<Actions::Set>,
}

impl<'a, Actions> ModelAction for UpdateMany<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Mutation(ModelMutationType::UpdateMany);
}

impl<'a, Actions> UpdateMany<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(
        client: &'a PrismaClientInternals,
        where_params: Vec<Actions::Where>,
        set_params: Vec<Actions::Set>,
    ) -> Self {
        Self {
            client,
            where_params,
            set_params,
        }
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut selection = Self::base_selection();

        selection.push_argument(
            "data",
            PrismaValue::Object(merge_fields(
                self.set_params.into_iter().map(Into::into).collect(),
            )),
        );

        if self.where_params.len() > 0 {
            selection.push_argument(
                "where",
                PrismaValue::Object(merge_fields(
                    self.where_params
                        .into_iter()
                        .map(WhereInput::serialize)
                        .map(|s| (s.field, s.value.into()))
                        .collect(),
                )),
            );
        }

        selection.push_nested_selection(BatchResult::selection());

        (Operation::Write(selection.build()), self.client)
    }

    pub(crate) fn convert(raw: BatchResult) -> i64 {
        raw.count
    }

    pub async fn exec(self) -> super::Result<i64> {
        let (op, client) = self.exec_operation();

        let res = client.execute(op).await.map(Self::convert)?;

        #[cfg(feature = "mutation-callbacks")]
        client.notify_model_mutation::<Self>();

        Ok(res)
    }
}

impl<'a, Actions> BatchQuery for UpdateMany<'a, Actions>
where
    Actions: ModelActions,
{
    type RawType = BatchResult;
    type ReturnType = i64;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        Self::convert(raw)
    }
}
