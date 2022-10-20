use prisma_models::PrismaValue;
use query_core::{Operation, QueryValue, SelectionBuilder};

use crate::{
    include::{Include, IncludeType},
    merged_object,
    select::{Select, SelectType},
    BatchQuery, ModelAction, ModelActionType, ModelActions, ModelQueryType, PrismaClientInternals,
    WhereInput,
};

use super::SerializedWhereInput;

pub struct FindFirst<'a, Actions>
where
    Actions: ModelActions,
{
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
    pub with_params: Vec<Actions::With>,
    pub order_by_params: Vec<Actions::OrderBy>,
    pub cursor_params: Vec<Actions::Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<'a, Actions> ModelAction for FindFirst<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const TYPE: ModelActionType = ModelActionType::Query(ModelQueryType::FindFirst);
}

impl<'a, Actions> FindFirst<'a, Actions>
where
    Actions: ModelActions,
{
    pub fn new(client: &'a PrismaClientInternals, where_params: Vec<Actions::Where>) -> Self {
        Self {
            client,
            where_params,
            with_params: vec![],
            order_by_params: vec![],
            cursor_params: vec![],
            skip: None,
            take: None,
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
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

    fn to_selection(
        where_params: Vec<Actions::Where>,
        order_by_params: Vec<Actions::OrderBy>,
        cursor_params: Vec<Actions::Cursor>,
        skip: Option<i64>,
        take: Option<i64>,
    ) -> SelectionBuilder {
        let mut selection = Self::base_selection();

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                merged_object(
                    where_params
                        .into_iter()
                        .map(WhereInput::serialize)
                        .map(|s| (s.field, s.value.into()))
                        .collect(),
                ),
            );
        }

        if order_by_params.len() > 0 {
            selection.push_argument(
                "orderBy".to_string(),
                PrismaValue::List(
                    order_by_params
                        .into_iter()
                        .map(Into::into)
                        .map(|(k, v)| PrismaValue::Object(vec![(k, v)]))
                        .collect(),
                ),
            );
        }

        if cursor_params.len() > 0 {
            selection.push_argument(
                "cursor".to_string(),
                PrismaValue::Object(
                    cursor_params
                        .into_iter()
                        .map(Into::into)
                        .map(WhereInput::serialize)
                        .map(SerializedWhereInput::transform_equals)
                        .collect(),
                ),
            );
        }

        skip.map(|skip| selection.push_argument("skip".to_string(), QueryValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), QueryValue::Int(take as i64)));

        selection
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, Option<S::Data>> {
        let mut selection = Self::to_selection(
            self.where_params,
            self.order_by_params,
            self.cursor_params,
            self.skip,
            self.take,
        );

        selection.nested_selections(select.to_selections());

        let op = Operation::Read(selection.build());

        Select::new(self.client, op)
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        include: I,
    ) -> Include<'a, Option<I::Data>> {
        let mut selection = Self::to_selection(
            self.where_params,
            self.order_by_params,
            self.cursor_params,
            self.skip,
            self.take,
        );

        selection.nested_selections(include.to_selections());

        let op = Operation::Read(selection.build());

        Include::new(self.client, op)
    }

    pub(crate) fn exec_operation(self) -> (Operation, &'a PrismaClientInternals) {
        let mut scalar_selections = Actions::scalar_selections();

        let mut selection = Self::to_selection(
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

        (Operation::Read(selection.build()), self.client)
    }

    pub async fn exec(self) -> super::Result<Option<Actions::Data>> {
        let (op, client) = self.exec_operation();

        client.notify_model_action::<Self>();
        client.execute(op).await
    }
}

impl<'a, Actions> BatchQuery for FindFirst<'a, Actions>
where
    Actions: ModelActions,
{
    type RawType = Actions::Data;
    type ReturnType = Self::RawType;

    fn graphql(self) -> Operation {
        self.exec_operation().0
    }

    fn convert(raw: Self::RawType) -> Self::ReturnType {
        raw
    }
}
