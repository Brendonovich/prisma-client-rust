use prisma_models::PrismaValue;
use query_core::{Operation, Selection, SelectionBuilder};

use crate::{merged_object, BatchResult};

use super::{QueryContext, QueryInfo};

pub struct CreateMany<'a, Set>
where
    Set: Into<(String, PrismaValue)>,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub set_params: Vec<Vec<Set>>,
}

impl<'a, Set> CreateMany<'a, Set>
where
    Set: Into<(String, PrismaValue)>,
{
    pub fn new(ctx: QueryContext<'a>, info: QueryInfo, set_params: Vec<Vec<Set>>) -> Self {
        Self {
            ctx,
            info,
            set_params,
        }
    }

    fn to_selection(model: &str, set_params: Vec<Vec<Set>>) -> SelectionBuilder {
        let mut selection = Selection::builder(format!("createMany{}", model));

        selection.alias("result");

        selection.push_argument(
            "data",
            PrismaValue::List(
                set_params
                    .into_iter()
                    .map(|fields| merged_object(fields.into_iter().map(Into::into).collect()))
                    .collect(),
            ),
        );

        selection
    }

    pub async fn exec(self) -> super::Result<i64> {
        let mut selection = Self::to_selection(self.info.model, self.set_params);

        selection.push_nested_selection(BatchResult::selection());

        let op = Operation::Write(selection.build());

        self.ctx.execute(op).await.map(|res: BatchResult| res.count)
    }
}
