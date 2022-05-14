use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::BatchResult;

use super::{transform_equals, QueryContext, QueryInfo, SerializedWhere};

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

    pub async fn exec(self) -> super::Result<i64> {
        let Self {
            ctx,
            info,
            where_params,
            ..
        } = self;

        let QueryInfo { model, .. } = info;

        let mut selection = Selection::builder(format!("deleteMany{}", model));

        selection.alias("result");

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                PrismaValue::Object(transform_equals(
                    where_params.into_iter().map(Into::into).collect(),
                )),
            );
        }

        selection.push_nested_selection(BatchResult::selection());

        let op = Operation::Write(selection.build());

        ctx.execute(op).await.map(|r: BatchResult| r.count)
    }
}
