use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{merged_object, BatchQuery, BatchResult};

use super::{QueryContext, QueryInfo, SerializedWhere};

pub struct UpdateMany<'a, Where, Set>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, PrismaValue)>,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_params: Vec<Where>,
    pub set_params: Vec<Set>,
}
impl<'a, Where, Set> UpdateMany<'a, Where, Set>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, PrismaValue)>,
{
    pub fn new(
        ctx: QueryContext<'a>,
        info: QueryInfo,
        where_params: Vec<Where>,
        set_params: Vec<Set>,
    ) -> Self {
        Self {
            ctx,
            info,
            where_params,
            set_params,
        }
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Selection::builder(format!("updateMany{}", &self.info.model));

        selection.alias("result");

        selection.push_argument(
            "data",
            merged_object(self.set_params.into_iter().map(Into::into).collect()),
        );

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

impl<'a, Where, Set> BatchQuery for UpdateMany<'a, Where, Set>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, PrismaValue)>,
{
    type RawType = BatchResult;
    type ReturnType = i64;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: super::Result<Self::RawType>) -> super::Result<Self::ReturnType> {
        raw.map(Self::convert)
    }
}
