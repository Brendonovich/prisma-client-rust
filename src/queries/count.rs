use prisma_models::PrismaValue;
use query_core::{Operation, Selection};
use serde::Deserialize;

use crate::{
    merge_fields, BatchQuery, ModelAction, ModelActionType, ModelActions, ModelQueryType,
    PrismaClientInternals, SerializedWhereInput, WhereInput,
};

pub struct Count<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
    pub order_by_params: Vec<Actions::OrderBy>,
    pub cursor_params: Vec<Actions::Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<'a, Actions> ModelAction for Count<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Query(ModelQueryType::Count);
}

impl<'a, Actions> Count<'a, Actions>
where
    Actions: ModelActions,
{
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

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
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
                                    .map(Into::into)
                                    .map(|(k, v)| PrismaValue::Object(vec![(k, v)]))
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
                                    .map(Into::into)
                                    .map(WhereInput::serialize)
                                    .map(SerializedWhereInput::transform_equals)
                                    .collect(),
                            )
                            .into(),
                        )
                    }),
                    self.skip
                        .map(|skip| ("skip".to_string(), PrismaValue::Int(skip as i64).into())),
                    self.take
                        .map(|take| ("take".to_string(), PrismaValue::Int(take as i64).into())),
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

    pub(crate) fn convert(data: CountAggregateResult) -> i64 {
        data._count._all
    }

    pub async fn exec(self) -> super::Result<i64> {
        let (op, client) = self.exec_operation();

        client.execute(op).await.map(Self::convert)
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

impl<'a, Actions> BatchQuery for Count<'a, Actions>
where
    Actions: ModelActions,
{
    type RawType = CountAggregateResult;
    type ReturnType = i64;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        Self::convert(raw)
    }
}
