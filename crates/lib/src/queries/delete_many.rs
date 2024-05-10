use query_core::Operation;

use crate::{
    merge_fields, BatchResult, ModelOperation, ModelQuery, ModelTypes, ModelWriteOperation,
    PrismaClientInternals, PrismaValue, Query, QueryConvert, WhereInput, WhereQuery,
};

pub struct DeleteMany<'a, Actions: ModelTypes> {
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
}

impl<'a, Actions: ModelTypes> DeleteMany<'a, Actions> {
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

impl<'a, Actions: ModelTypes> QueryConvert for DeleteMany<'a, Actions> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(Self::convert(raw))
    }
}

impl<'a, Actions: ModelTypes> Query<'a> for DeleteMany<'a, Actions> {
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
                        )),
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

impl<'a, Actions: ModelTypes> ModelQuery<'a> for DeleteMany<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::DeleteMany);
}

impl<'a, Actions: ModelTypes> WhereQuery<'a> for DeleteMany<'a, Actions> {
    fn add_where(&mut self, param: Actions::Where) {
        self.where_params.push(param);
    }
}
