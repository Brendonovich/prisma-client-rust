use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{
    merge_fields, BatchResult, CreateModelTypes, ModelOperation, ModelQuery, ModelWriteOperation,
    PrismaClientInternals, Query, QueryConvert,
};

pub struct CreateMany<'a, Actions: CreateModelTypes> {
    client: &'a PrismaClientInternals,
    pub set_params: Vec<Vec<Actions::CreateUnchecked>>,
    pub skip_duplicates: bool,
}

impl<'a, Actions: CreateModelTypes> CreateMany<'a, Actions> {
    pub fn new(
        client: &'a PrismaClientInternals,
        set_params: Vec<Vec<Actions::CreateUnchecked>>,
    ) -> Self {
        Self {
            client,
            set_params,
            skip_duplicates: false,
        }
    }

    #[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgresql"))]
    pub fn skip_duplicates(mut self) -> Self {
        self.skip_duplicates = true;
        self
    }

    fn to_selection(
        set_params: Vec<Vec<Actions::CreateUnchecked>>,
        _skip_duplicates: bool,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [
                Some((
                    "data".to_string(),
                    PrismaValue::List(
                        set_params
                            .into_iter()
                            .map(|fields| {
                                PrismaValue::Object(merge_fields(
                                    fields.into_iter().map(Into::into).map(Into::into).collect(),
                                ))
                            })
                            .collect(),
                    )
                    .into(),
                )),
                _skip_duplicates.then(|| {
                    (
                        "skipDuplicates".to_string(),
                        PrismaValue::Boolean(_skip_duplicates).into(),
                    )
                }),
            ]
            .into_iter()
            .flatten(),
            nested_selections,
        )
    }

    pub async fn exec(self) -> super::Result<i64> {
        super::exec(self).await
    }
}

impl<'a, Actions: CreateModelTypes> QueryConvert for CreateMany<'a, Actions> {
    type RawType = BatchResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw.count)
    }
}

impl<'a, Actions: CreateModelTypes> Query<'a> for CreateMany<'a, Actions> {
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

impl<'a, Actions: CreateModelTypes> ModelQuery<'a> for CreateMany<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Write(ModelWriteOperation::CreateMany);
}
