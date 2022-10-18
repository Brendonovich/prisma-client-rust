use query_core::Operation;

use crate::{merged_object, Action, BatchQuery, BatchResult, ModelActions};

use super::{QueryContext, SerializedWhere};

pub struct DeleteMany<'a, Actions>
where
    Actions: ModelActions,
{
    ctx: QueryContext<'a>,
    pub where_params: Vec<Actions::Where>,
}

impl<'a, Actions> Action for DeleteMany<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const NAME: &'static str = "deleteMany";
}

impl<'a, Actions> DeleteMany<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(ctx: QueryContext<'a>, where_params: Vec<Actions::Where>) -> Self {
        Self { ctx, where_params }
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Self::base_selection();

        if self.where_params.len() > 0 {
            selection.push_argument(
                "where",
                merged_object(
                    self.where_params
                        .into_iter()
                        .map(Into::<SerializedWhere>::into)
                        .map(|s| (s.field, s.value.into()))
                        .collect(),
                ),
            );
        }

        selection.push_nested_selection(BatchResult::selection());

        (Operation::Write(selection.build()), self.ctx)
    }

    pub(crate) fn convert(raw: BatchResult) -> i64 {
        raw.count
    }

    pub async fn exec(self) -> super::Result<i64> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await.map(Self::convert)
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
