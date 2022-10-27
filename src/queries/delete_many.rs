use query_core::Operation;

use crate::{
    merge_fields, BatchQuery, BatchResult, ModelAction, ModelActionType, ModelActions,
    ModelMutationType, PrismaClientInternals, WhereInput,
};
use prisma_models::PrismaValue;

pub struct DeleteMany<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
}

impl<'a, Actions> ModelAction for DeleteMany<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Mutation(ModelMutationType::DeleteMany);
}

impl<'a, Actions> DeleteMany<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(client: &'a PrismaClientInternals, where_params: Vec<Actions::Where>) -> Self {
        Self {
            client,
            where_params,
        }
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut selection = Self::base_selection();

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

impl<'a, Actions> BatchQuery for DeleteMany<'a, Actions>
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
