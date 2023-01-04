use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{
    merge_fields, BatchResult, ModelActions, ModelOperation, ModelQuery, ModelWriteOperation,
    PrismaClientInternals, Query, QueryConvert,
};

pub struct CreateMany<'a, Actions: ModelActions> {
    client: &'a PrismaClientInternals,
    pub set_params: Vec<Vec<Actions::Set>>,
    pub skip_duplicates: bool,
}

impl<'a, Actions: ModelActions> CreateMany<'a, Actions> {
    pub fn new(client: &'a PrismaClientInternals, set_params: Vec<Vec<Actions::Set>>) -> Self {
        Self {
            client,
            set_params,
            skip_duplicates: false,
        }
    }

    #[cfg(not(any(feature = "mongodb", feature = "mssql")))]
    pub fn skip_duplicates(mut self) -> Self {
        self.skip_duplicates = true;
        self
    }

    fn to_selection(
        set_params: Vec<Vec<Actions::Set>>,
        _skip_duplicates: bool,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [
                (
                    "data".to_string(),
                    PrismaValue::List(
                        set_params
                            .into_iter()
                            .map(|fields| {
                                PrismaValue::Object(merge_fields(
                                    fields.into_iter().map(Into::into).collect(),
                                ))
                            })
                            .collect(),
                    )
                    .into(),
                ),
                #[cfg(not(any(feature = "mongodb", feature = "mssql")))]
                (
                    "skipDuplicates".to_string(),
                    PrismaValue::Boolean(_skip_duplicates).into(),
                ),
            ],
            nested_selections,
        )
    }

    pub async fn exec(self) -> super::Result<i64> {
        super::exec(self).await
    }
}

impl<'a, Actions: ModelActions> Query<'a> for CreateMany<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Self::to_selection(
                self.set_params,
                self.skip_duplicates,
                [BatchResult::selection()],
            )),
            self.client,
        )
    }
}

impl<'a, Actions: ModelActions> QueryConvert for CreateMany<'a, Actions> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> Self::ReturnValue {
        raw.count
    }
}

impl<'a, Actions: ModelActions> ModelQuery<'a> for CreateMany<'a, Actions> {
    type Actions = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::CreateMany);
}
