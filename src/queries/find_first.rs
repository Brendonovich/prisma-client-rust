use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, QueryValue, Selection};
use serde::de::DeserializeOwned;

use super::{QueryContext, QueryInfo, SerializedWhere};

pub struct FindFirst<'a, Where, With, OrderBy, Cursor, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<(String, PrismaValue)>,
    Data: DeserializeOwned,
{
    ctx: QueryContext<'a>,
    info: QueryInfo,
    pub where_params: Vec<Where>,
    pub with_params: Vec<With>,
    pub order_by_params: Vec<OrderBy>,
    pub cursor_params: Vec<Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
    _data: PhantomData<Data>,
}

impl<'a, Where, With, OrderBy, Cursor, Data> FindFirst<'a, Where, With, OrderBy, Cursor, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<(String, PrismaValue)>,
    Data: DeserializeOwned,
{
    pub fn new(ctx: QueryContext<'a>, info: QueryInfo, where_params: Vec<Where>) -> Self {
        Self {
            ctx,
            info,
            where_params,
            with_params: vec![],
            order_by_params: vec![],
            cursor_params: vec![],
            skip: None,
            take: None,
            _data: PhantomData,
        }
    }

    pub fn with(mut self, param: impl Into<With>) -> Self {
        self.with_params.push(param.into());
        self
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

    pub async fn exec(self) -> super::Result<Option<Data>> {
        let Self {
            ctx,
            info,
            where_params,
            with_params,
            order_by_params,
            cursor_params,
            skip,
            take,
            ..
        } = self;

        let QueryInfo {
            model,
            mut scalar_selections,
        } = info;

        let mut selection = Selection::builder(format!("findFirst{}", model));

        selection.alias("result");

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                PrismaValue::Object(
                    where_params
                        .into_iter()
                        .map(Into::<SerializedWhere>::into)
                        .map(Into::into)
                        .collect(),
                ),
            );
        }

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

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

        skip.map(|skip| selection.push_argument("skip".to_string(), QueryValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), QueryValue::Int(take as i64)));

        let op = Operation::Read(selection.build());

        ctx.execute(op).await
    }
}
