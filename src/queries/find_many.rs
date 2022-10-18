use prisma_models::PrismaValue;
use query_core::{Operation, QueryValue, Selection, SelectionBuilder};

use crate::{
    actions::ModelActions,
    include::{Include, IncludeType},
    merged_object,
    select::{Select, SelectType},
    Action, BatchQuery, PrismaClientInternals,
};

use super::SerializedWhere;

pub struct FindMany<'a, Actions>
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

impl<'a, Actions> Action for FindMany<'a, Actions>
where
    Actions: ModelActions,
{
    type Actions = Actions;

    const NAME: &'static str = "findMany";
}

impl<'a, Actions> FindMany<'a, Actions>
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
                        .map(Into::<SerializedWhere>::into)
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
                        .map(|v| PrismaValue::Object(vec![v]))
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
                        .map(Into::<SerializedWhere>::into)
                        .map(SerializedWhere::transform_equals)
                        .collect(),
                ),
            );
        }

        skip.map(|skip| selection.push_argument("skip".to_string(), PrismaValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), PrismaValue::Int(take as i64)));

        selection
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, Vec<S::Data>> {
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
    ) -> Include<'a, Vec<I::Data>> {
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

    pub async fn exec(self) -> super::Result<Vec<Actions::Data>> {
        let (op, ctx) = self.exec_operation();

        ctx.execute(op).await
    }
}

impl<'a, Actions> BatchQuery for FindMany<'a, Actions>
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

#[derive(Clone)]
pub struct ManyArgs<Actions>
where
    Actions: ModelActions,
{
    pub where_params: Vec<Actions::Where>,
    pub with_params: Vec<Actions::With>,
    pub order_by_params: Vec<Actions::OrderBy>,
    pub cursor_params: Vec<Actions::Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<Actions> ManyArgs<Actions>
where
    Actions: ModelActions,
{
    pub fn new(where_params: Vec<Actions::Where>) -> Self {
        Self {
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

    pub fn to_graphql(self) -> (Vec<(String, QueryValue)>, Vec<Selection>) {
        let (mut arguments, mut nested_selections) = (vec![], vec![]);

        if self.with_params.len() > 0 {
            nested_selections = self.with_params.into_iter().map(Into::into).collect()
        }

        if self.where_params.len() > 0 {
            arguments.push((
                "where".to_string(),
                PrismaValue::Object(
                    self.where_params
                        .into_iter()
                        .map(Into::<SerializedWhere>::into)
                        .map(Into::into)
                        .collect(),
                )
                .into(),
            ));
        }

        if self.order_by_params.len() > 0 {
            arguments.push((
                "orderBy".to_string(),
                PrismaValue::List(
                    self.order_by_params
                        .into_iter()
                        .map(Into::into)
                        .map(|v| PrismaValue::Object(vec![v]))
                        .collect(),
                )
                .into(),
            ));
        }

        if self.cursor_params.len() > 0 {
            arguments.push((
                "cursor".to_string(),
                PrismaValue::Object(
                    self.cursor_params
                        .into_iter()
                        .map(Into::into)
                        .map(Into::<SerializedWhere>::into)
                        .map(SerializedWhere::transform_equals)
                        .collect(),
                )
                .into(),
            ));
        }

        self.skip
            .map(|skip| arguments.push(("skip".to_string(), QueryValue::Int(skip))));
        self.take
            .map(|take| arguments.push(("take".to_string(), QueryValue::Int(take))));

        (arguments, nested_selections)
    }
}
