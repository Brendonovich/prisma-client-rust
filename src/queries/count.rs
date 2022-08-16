use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::Deserialize;

use crate::{merged_object, SerializedWhere};

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
    Cursor: Into<(String, PrismaValue)>,
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

    pub fn order_by(mut self, param: impl Into<OrderBy>) -> Self {
        self.order_by_params.push(param.into());
        self
    }

    pub fn cursor(mut self, param: impl Into<Cursor>) -> Self {
        self.cursor_params.push(param.into());
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

    pub async fn exec(self) -> super::Result<i64> {
        let Self {
            ctx,
            info,
            where_params,
            order_by_params,
            cursor_params,
            skip,
            take,
            ..
        } = self;

        let QueryInfo { model, .. } = info;

        let mut selection = Selection::builder(format!("aggregate{}", model));

        selection.alias("result");

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                merged_object(
                    where_params
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

        if order_by_params.len() > 0 {
            selection.push_argument(
                "orderBy".to_string(),
                PrismaValue::Object(order_by_params.into_iter().map(Into::into).collect()),
            );
        }

        if cursor_params.len() > 0 {
            selection.push_argument(
                "cursor".to_string(),
                PrismaValue::Object(cursor_params.into_iter().map(Into::into).collect()),
            );
        }

        skip.map(|skip| selection.push_argument("skip".to_string(), PrismaValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), PrismaValue::Int(take as i64)));

        let op = Operation::Read(selection.build());

        #[derive(Deserialize)]
        struct CountAggregateResult {
            _count: CountResult,
        }

        #[derive(Deserialize)]
        struct CountResult {
            _all: i64,
        }

        ctx.execute(op)
            .await
            .map(|res: CountAggregateResult| res._count._all)
    }
}
