use prisma_models::PrismaValue;
use query_core::{Operation, QueryValue, Selection};

use crate::{
    merge_fields, BatchQuery, BatchResult, ModelAction, ModelActionType, ModelActions,
    ModelMutationType, PrismaClientInternals,
};

pub struct CreateMany<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub set_params: Vec<Vec<Actions::Set>>,
    pub skip_duplicates: bool,
}

impl<'a, Actions> ModelAction for CreateMany<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Mutation(ModelMutationType::CreateMany);
}

impl<'a, Actions> CreateMany<'a, Actions>
where
    Actions: ModelActions,
{
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
                    QueryValue::Boolean(_skip_duplicates),
                ),
            ],
            nested_selections,
        )
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Write(Self::to_selection(
                self.set_params,
                self.skip_duplicates,
                [BatchResult::selection()],
            )),
            self.client,
        )
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

impl<'a, Actions> BatchQuery for CreateMany<'a, Actions>
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
