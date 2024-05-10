use query_core::{Operation, Selection};
use serde::Deserialize;

use crate::{
    merge_fields, ModelOperation, ModelQuery, ModelReadOperation, ModelTypes, OrderByQuery,
    PaginatedQuery, PrismaClientInternals, PrismaValue, Query, QueryConvert, WhereInput,
    WhereQuery,
};

pub struct Count<'a, Actions: ModelTypes> {
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
    pub order_by_params: Vec<Actions::OrderBy>,
    pub cursor_params: Vec<Actions::Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<'a, Actions: ModelTypes> Count<'a, Actions> {
    pub fn new(client: &'a PrismaClientInternals, where_params: Vec<Actions::Where>) -> Self {
        Self {
            client,
            where_params,
            order_by_params: vec![],
            cursor_params: vec![],
            skip: None,
            take: None,
        }
    }

    pub fn order_by(mut self, param: Actions::OrderBy) -> Self {
        self.order_by_params.push(param);
        self
    }

    pub fn cursor(mut self, param: Actions::Cursor) -> Self {
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

    pub async fn exec(self) -> super::Result<i64> {
        super::exec(self).await
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

impl<'a, Actions: ModelTypes> QueryConvert for Count<'a, Actions> {
    type RawType = CountAggregateResult;
    type ReturnValue = i64;

    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue> {
        Ok(raw._count._all)
    }
}

impl<'a, Actions: ModelTypes> Query<'a> for Count<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (
            Operation::Read(Self::base_selection(
                [
                    (!self.where_params.is_empty()).then(|| {
                        (
                            "where".to_string(),
                            PrismaValue::Object(merge_fields(
                                self.where_params
                                    .into_iter()
                                    .map(WhereInput::serialize)
                                    .map(|s| (s.field, s.value.into()))
                                    .collect(),
                            ))
                            .into(),
                        )
                    }),
                    (!self.order_by_params.is_empty()).then(|| {
                        (
                            "orderBy".to_string(),
                            PrismaValue::List(
                                self.order_by_params
                                    .into_iter()
                                    .map(|p| PrismaValue::Object(vec![p.into()]))
                                    .collect(),
                            )
                            .into(),
                        )
                    }),
                    (!self.cursor_params.is_empty()).then(|| {
                        (
                            "cursor".to_string(),
                            PrismaValue::Object(
                                self.cursor_params
                                    .into_iter()
                                    .map(WhereInput::serialize)
                                    .map(|s| (s.field, s.value.into()))
                                    .collect(),
                            ),
                        )
                    }),
                    self.skip
                        .map(|skip| ("skip".to_string(), PrismaValue::Int(skip as i32))),
                    self.take
                        .map(|take| ("take".to_string(), PrismaValue::Int(take as i32))),
                ]
                .into_iter()
                .flatten(),
                [Selection::new(
                    "_count",
                    None,
                    [],
                    [Selection::new("_all", None, [], [])],
                )],
            )),
            self.client,
        )
    }
}

impl<'a, Actions: ModelTypes> ModelQuery<'a> for Count<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Read(ModelReadOperation::Count);
}

impl<'a, Actions: ModelTypes> WhereQuery<'a> for Count<'a, Actions> {
    fn add_where(&mut self, param: Actions::Where) {
        self.where_params.push(param);
    }
}

impl<'a, Actions: ModelTypes> OrderByQuery<'a> for Count<'a, Actions> {
    fn add_order_by(&mut self, param: Actions::OrderBy) {
        self.order_by_params.push(param);
    }
}

impl<'a, Actions: ModelTypes> PaginatedQuery<'a> for Count<'a, Actions> {
    fn add_cursor(&mut self, param: Actions::Cursor) {
        self.cursor_params.push(param);
    }

    fn set_skip(&mut self, skip: i64) {
        self.skip = Some(skip);
    }

    fn set_take(&mut self, take: i64) {
        self.take = Some(take);
    }
}
