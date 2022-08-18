use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, QueryValue, Selection, SelectionBuilder};
use serde::de::DeserializeOwned;

use crate::{
    merged_object,
    select::{Select, SelectType},
    BatchQuery,
};

use super::{QueryContext, QueryInfo, SerializedWhere};

pub struct FindFirst<'a, Where, With, OrderBy, Cursor, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<Where>,
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
    Cursor: Into<Where>,
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

    fn to_selection(
        model: &str,
        where_params: Vec<Where>,
        order_by_params: Vec<OrderBy>,
        cursor_params: Vec<Cursor>,
        skip: Option<i64>,
        take: Option<i64>,
    ) -> SelectionBuilder {
        let mut selection = Selection::builder(format!("findFirst{}", model));

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

        if order_by_params.len() > 0 {
            selection.push_argument(
                "orderBy".to_string(),
                PrismaValue::Object(order_by_params.into_iter().map(Into::into).collect()),
            );
        }

        if cursor_params.len() > 0 {
            selection.push_argument(
                "cursor".to_string(),
                PrismaValue::Object(
                    cursor_params
                        .into_iter()
                        .map(Into::into)
                        .map(Into::<SerializedWhere>::into)
                        .map(SerializedWhere::transform_equals)
                        .collect(),
                ),
            );
        }

        skip.map(|skip| selection.push_argument("skip".to_string(), QueryValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), QueryValue::Int(take as i64)));

        selection
    }

    pub fn select<S: SelectType<Data>>(self, select: S) -> Select<'a, Option<S::Data>> {
        let mut selection = Self::to_selection(
            self.info.model,
            self.where_params,
            self.order_by_params,
            self.cursor_params,
            self.skip,
            self.take,
        );

        selection.nested_selections(select.to_selections());

        let op = Operation::Read(selection.build());

        Select::new(self.ctx, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, QueryContext<'a>) {
        let QueryInfo {
            model,
            mut scalar_selections,
            ..
        } = self.info;

        let mut selection = Self::to_selection(
            model,
            self.where_params,
            self.order_by_params,
            self.cursor_params,
            self.skip,
            self.take,
        );

        if self.with_params.len() > 0 {
            scalar_selections.append(&mut self.with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        (Operation::Read(selection.build()), self.ctx)
    }

    pub async fn exec(self) -> super::Result<Option<Data>> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await
    }
}

impl<'a, Where, With, OrderBy, Cursor, Data> BatchQuery
    for FindFirst<'a, Where, With, OrderBy, Cursor, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<Where>,
    Data: DeserializeOwned,
{
    type RawType = Data;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
