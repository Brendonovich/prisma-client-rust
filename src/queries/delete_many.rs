use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::BatchResult;

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

    pub async fn exec(self) -> super::Result<i64> {
        let mut selection = Selection::builder(format!("deleteMany{}", self.info.model));

        selection.alias("result");

        if self.where_params.len() > 0 {
            selection.push_argument(
                "where",
                PrismaValue::Object(
                    self.where_params
                        .into_iter()
                        .map(Into::<SerializedWhere>::into)
                        .map(Into::into)
                        .collect(),
                ),
            );
        }

        selection.push_nested_selection(BatchResult::selection());

        let op = Operation::Write(selection.build());

        self.ctx.execute(op).await.map(|r: BatchResult| r.count)
    }
}
