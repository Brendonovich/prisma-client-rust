use prisma_models::PrismaValue;
use query_core::Operation;

use crate::{
    merge_fields, BatchResult, ModelActions, ModelOperation, ModelQuery, ModelWriteOperation,
    PrismaClientInternals, Query, SetQuery, WhereInput, WhereQuery,
};

pub struct UpdateMany<'a, Actions: ModelActions> {
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
    pub set_params: Vec<Actions::Set>,
}

impl<'a, Actions: ModelActions> UpdateMany<'a, Actions> {
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

    pub async fn exec(self) -> super::Result<i64> {
        super::exec(self).await
    }
}

impl<'a, Actions: ModelActions> Query<'a> for UpdateMany<'a, Actions> {
    type RawType = BatchResult;
    type ReturnType = i64;

    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Self::base_selection(
                [
                    Some((
                        "data".to_string(),
                        PrismaValue::Object(merge_fields(
                            self.set_params.into_iter().map(Into::into).collect(),
                        ))
                        .into(),
                    )),
                    (!self.where_params.is_empty()).then(|| {
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
                    }),
                ]
                .into_iter()
                .flatten(),
                [BatchResult::selection()],
            )),
            self.client,
        )
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw.count
    }
}

impl<'a, Actions: ModelActions> ModelQuery<'a> for UpdateMany<'a, Actions> {
    type Actions = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::UpdateMany);
}

impl<'a, Actions: ModelActions> WhereQuery<'a> for UpdateMany<'a, Actions> {
    fn add_where(&mut self, param: Actions::Where) {
        self.where_params.push(param);
    }
}

impl<'a, Actions: ModelActions> SetQuery<'a> for UpdateMany<'a, Actions> {
    fn add_set(&mut self, param: Actions::Set) {
        self.set_params.push(param);
    }
}
