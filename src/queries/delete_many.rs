use query_core::Operation;

use crate::{
    merge_fields, BatchResult, ModelActions, ModelOperation, ModelQuery, ModelWriteOperation,
    PrismaClientInternals, Query, QueryConvert, WhereInput, WhereQuery,
};
use prisma_models::PrismaValue;

pub struct DeleteMany<'a, Actions: ModelActions> {
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
}

impl<'a, Actions: ModelActions> DeleteMany<'a, Actions> {
    pub fn new(client: &'a PrismaClientInternals, where_params: Vec<Actions::Where>) -> Self {
        Self {
            client,
            where_params,
        }
    }

    pub(crate) fn convert(raw: BatchResult) -> i64 {
        raw.count
    }

    pub async fn exec(self) -> super::Result<i64> {
        super::exec(self).await
    }
}

impl<'a, Actions: ModelActions> Query<'a> for DeleteMany<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Self::base_selection(
                [(!self.where_params.is_empty()).then(|| {
                    (
                        "where".to_string(),
                        PrismaValue::Object(merge_fields(
                            self.where_params
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|s| (s.field, s.value.into()))
                                .collect(),
                        ))
                        .into(),
                    )
                })]
                .into_iter()
                .flatten(),
                [BatchResult::selection()],
            )),
            self.client,
        )
    }
}

impl<'a, Actions: ModelActions> QueryConvert for DeleteMany<'a, Actions> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> Self::ReturnValue {
        Self::convert(raw)
    }
}

impl<'a, Actions: ModelActions> ModelQuery<'a> for DeleteMany<'a, Actions> {
    type Actions = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::DeleteMany);
}

impl<'a, Actions: ModelActions> WhereQuery<'a> for DeleteMany<'a, Actions> {
    fn add_where(&mut self, param: Actions::Where) {
        self.where_params.push(param);
    }
}
