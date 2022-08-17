use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::Deserialize;

use crate::{merged_object, BatchQuery, SerializedWhere};

use super::{QueryContext, QueryInfo};

pub struct Count<'a, Where, OrderBy, Cursor> {
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_params: Vec<Where>,
    pub order_by_params: Vec<OrderBy>,
    pub cursor_params: Vec<Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<'a, Where, OrderBy, Cursor> Count<'a, Where, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<Where>,
{
    pub fn new(ctx: QueryContext<'a>, info: QueryInfo, where_params: Vec<Where>) -> Self {
        Self {
            ctx,
            info,
            where_params,
            order_by_params: vec![],
            cursor_params: vec![],
            skip: None,
            take: None,
        }
    }

    pub fn order_by(mut self, param: OrderBy) -> Self {
        self.order_by_params.push(param);
        self
    }

    pub fn cursor(mut self, param: Cursor) -> Self {
        self.cursor_params.push(param);
        self
    }

    pub fn skip(mut self, skip: i64) -> Self {
        self.skip = Some(skip);
        self
    }

    pub fn take(mut self, take: i64) -> Self {
        self.take = Some(take);
        self
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let mut selection = Selection::builder(format!("aggregate{}", &self.info.model));

        selection.alias("result");

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

        selection.push_nested_selection({
            let mut count_builder = Selection::builder("_count");
            count_builder.push_nested_selection(Selection::builder("_all").build());
            count_builder.build()
        });

        if self.order_by_params.len() > 0 {
            selection.push_argument(
                "orderBy".to_string(),
                PrismaValue::Object(self.order_by_params.into_iter().map(Into::into).collect()),
            );
        }

        if self.cursor_params.len() > 0 {
            selection.push_argument(
                "cursor".to_string(),
                PrismaValue::Object(
                    self.cursor_params
                        .into_iter()
                        .map(Into::into)
                        .map(Into::<SerializedWhere>::into)
                        .map(SerializedWhere::transform_equals)
                        .collect(),
                ),
            );
        }

        self.skip
            .map(|skip| selection.push_argument("skip".to_string(), PrismaValue::Int(skip as i64)));
        self.take
            .map(|take| selection.push_argument("take".to_string(), PrismaValue::Int(take as i64)));

        (Operation::Read(selection.build()), self.ctx)
    }

    pub(crate) fn convert(data: CountAggregateResult) -> i64 {
        data._count._all
    }

    pub async fn exec(self) -> super::Result<i64> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await.map(Self::convert)
    }
}

#[derive(Deserialize)]
pub struct CountAggregateResult {
    _count: CountResult,
}

#[derive(Deserialize)]
pub struct CountResult {
    _all: i64,
}

impl<'a, Where, OrderBy, Cursor> BatchQuery for Count<'a, Where, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<Where>,
{
    type RawType = CountAggregateResult;
    type ReturnType = i64;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: super::Result<Self::RawType>) -> super::Result<Self::ReturnType> {
        raw.map(Self::convert)
    }
}
