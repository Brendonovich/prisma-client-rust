use std::marker::PhantomData;

use prisma_models::PrismaValue;
use query_core::{Operation, QueryValue, Selection};
use serde::de::DeserializeOwned;

use super::{
    count::Count, delete_many::DeleteMany, transform_equals, QueryContext, QueryInfo,
    SerializedWhere, UpdateMany,
};

pub struct FindMany<'a, Where, With, OrderBy, Cursor, Set, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<(String, PrismaValue)>,
    Set: Into<(String, PrismaValue)>,
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
    _data: PhantomData<(Set, Data)>,
}

impl<'a, Where, With, OrderBy, Cursor, Set, Data>
    FindMany<'a, Where, With, OrderBy, Cursor, Set, Data>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<(String, PrismaValue)>,
    Set: Into<(String, PrismaValue)>,
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

    pub fn update(self, data: Vec<Set>) -> UpdateMany<'a, Where, Set> {
        let Self {
            ctx,
            info,
            where_params,
            ..
        } = self;

        UpdateMany::new(ctx, info, where_params, data)
    }

    pub fn delete(self) -> DeleteMany<'a, Where> {
        let Self {
            ctx,
            info,
            where_params,
            ..
        } = self;

        DeleteMany::new(ctx, info, where_params)
    }

    pub fn count(self) -> Count<'a, Where, OrderBy, Cursor> {
        let Self {
            ctx,
            info,
            where_params,
            ..
        } = self;

        Count::new(ctx, info, where_params)
    }

    pub async fn exec(self) -> super::Result<Vec<Data>> {
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

        let mut selection = Selection::builder(format!("findMany{}", model));

        selection.alias("result");

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                PrismaValue::Object(transform_equals(where_params.into_iter())),
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

        skip.map(|skip| selection.push_argument("skip".to_string(), PrismaValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), PrismaValue::Int(take as i64)));

        let op = Operation::Read(selection.build());

        ctx.execute(op).await
    }
}

#[derive(Clone)]
pub struct ManyArgs<Where, With, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<(String, PrismaValue)>,
{
    pub where_params: Vec<Where>,
    pub with_params: Vec<With>,
    pub order_by_params: Vec<OrderBy>,
    pub cursor_params: Vec<Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<Where, With, OrderBy, Cursor> ManyArgs<Where, With, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, PrismaValue)>,
    Cursor: Into<(String, PrismaValue)>,
{
    pub fn new(where_params: Vec<Where>) -> Self {
        Self {
            where_params,
            with_params: vec![],
            order_by_params: vec![],
            cursor_params: vec![],
            skip: None,
            take: None,
        }
    }

    pub fn with(mut self, param: impl Into<With>) -> Self {
        self.with_params.push(param.into());
        self
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

    pub fn to_graphql(self) -> (Vec<(String, QueryValue)>, Vec<Selection>) {
        let Self {
            where_params,
            with_params,
            order_by_params,
            cursor_params,
            skip,
            take,
        } = self;

        let (mut arguments, mut nested_selections) = (vec![], vec![]);

        if with_params.len() > 0 {
            nested_selections = with_params.into_iter().map(Into::into).collect()
        }

        if where_params.len() > 0 {
            arguments.push((
                "where".to_string(),
                PrismaValue::Object(transform_equals(where_params.into_iter())).into(),
            ));
        }

        if order_by_params.len() > 0 {
            arguments.push((
                "orderBy".to_string(),
                PrismaValue::Object(order_by_params.into_iter().map(Into::into).collect()).into(),
            ));
        }

        if cursor_params.len() > 0 {
            arguments.push((
                "cursor".to_string(),
                PrismaValue::Object(cursor_params.into_iter().map(Into::into).collect()).into(),
            ));
        }

        skip.map(|skip| arguments.push(("skip".to_string(), QueryValue::Int(skip))));
        take.map(|take| arguments.push(("take".to_string(), QueryValue::Int(take))));

        (arguments, nested_selections)
    }
}
