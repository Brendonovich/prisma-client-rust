use prisma_models::PrismaValue;
use query_core::{Operation, Selection, SelectionBuilder};

use crate::{merged_object, BatchQuery, BatchResult};

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

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Self::to_selection(self.info.model, self.set_params);

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

impl<'a, Set> BatchQuery for CreateMany<'a, Set>
where
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
