use query_core::Operation;

use crate::{
    merge_fields, BatchResult, ModelOperation, ModelQuery, ModelTypes, ModelWriteOperation,
    PrismaClientInternals, PrismaValue, Query, QueryConvert, SetQuery, WhereInput, WhereQuery,
};

pub struct UpdateMany<'a, Actions: ModelTypes> {
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
    pub set_params: Vec<Actions::Set>,
}

impl<'a, Actions: ModelTypes> UpdateMany<'a, Actions> {
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

impl<'a, Actions: ModelTypes> QueryConvert for UpdateMany<'a, Actions> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw.count)
    }
}

impl<'a, Actions: ModelTypes> Query<'a> for UpdateMany<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Self::base_selection(
                [
                    Some((
                        "data".to_string(),
                        PrismaValue::Object(merge_fields(
                            self.set_params
                                .into_iter()
                                .map(Into::into)
                                .map(Into::into)
                                .collect(),
                        )),
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
                            )),
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
}

impl<'a, Actions: ModelTypes> ModelQuery<'a> for UpdateMany<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::UpdateMany);
}

impl<'a, Actions: ModelTypes> WhereQuery<'a> for UpdateMany<'a, Actions> {
    fn add_where(&mut self, param: Actions::Where) {
        self.where_params.push(param);
    }
}

impl<'a, Actions: ModelTypes> SetQuery<'a> for UpdateMany<'a, Actions> {
    fn add_set(&mut self, param: Actions::Set) {
        self.set_params.push(param);
    }
}
