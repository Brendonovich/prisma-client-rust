use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::{merge_fields, BatchQuery, BatchResult};

use super::{QueryContext, QueryInfo, SerializedWhere};

pub struct DeleteMany<'a, Where>
where
    Where: Into<SerializedWhere>,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_params: Vec<Where>,
}

impl<'a, Where> DeleteMany<'a, Where>
where
    Where: Into<SerializedWhere>,
{
    pub fn new(ctx: QueryContext<'a>, info: QueryInfo, where_params: Vec<Where>) -> Self {
        Self {
            ctx,
            info,
            where_params,
        }
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Selection::builder(format!("deleteMany{}", self.info.model));

        selection.alias("result");

        if self.where_params.len() > 0 {
            selection.push_argument(
                "where",
                PrismaValue::Object(merge_fields(
                    self.where_params
                        .into_iter()
                        .map(Into::<SerializedWhere>::into)
                        .map(|s| (s.field, s.value.into()))
                        .collect(),
                )),
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

impl<'a, Where> BatchQuery for DeleteMany<'a, Where>
where
    Where: Into<SerializedWhere>,
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
