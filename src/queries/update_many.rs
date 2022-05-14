use prisma_models::PrismaValue;
use query_core::{Operation, Selection};

use crate::BatchResult;

use super::{transform_equals, QueryContext, QueryInfo, SerializedWhere};

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

    pub async fn exec(self) -> super::Result<i64> {
        let Self {
            ctx,
            info,
            where_params,
            set_params,
            ..
        } = self;

        let QueryInfo { model, .. } = info;

        let mut selection = Selection::builder(format!("updateMany{}", model));

        selection.alias("result");

        selection.push_argument(
            "data",
            PrismaValue::Object(set_params.into_iter().map(Into::into).collect()),
        );

        if where_params.len() > 0 {
            selection.push_argument("where", PrismaValue::Object(transform_equals(where_params)));
        }

        selection.push_nested_selection(BatchResult::selection());

        let op = Operation::Write(selection.build());

        ctx.execute(op).await.map(|res: BatchResult| res.count)
    }
}
