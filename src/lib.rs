pub mod operator;
pub mod query;
pub mod serde;
pub mod traits;

use ::serde::{Deserialize, Serialize};
pub use chrono;
pub use datamodel;
use datamodel::datamodel_connector::Diagnostics;
pub use prisma_models;
pub use query_core;
use query_core::{CoreError, Operation, QueryValue, Selection};
pub use serde_json;
use thiserror::Error;

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

#[derive(Deserialize)]
pub struct BatchResult {
    pub count: i64,
}

impl BatchResult {
    pub fn selection() -> Selection {
        let selection = Selection::builder("count");
        selection.build()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Asc => "asc".to_string(),
            Direction::Desc => "desc".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum NewClientError {
    #[error("Error configuring database connection: {0}")]
    Configuration(Diagnostics),

    #[error("Error loading database executor: {0}")]
    Executor(#[from] CoreError),

    #[error("Error getting database connection: {0}")]
    Connection(#[from] query_connector::error::ConnectorError),
}

impl From<Diagnostics> for NewClientError {
    fn from(diagnostics: Diagnostics) -> Self {
        NewClientError::Configuration(diagnostics)
    }
}

#[macro_export]
macro_rules! not {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::not(vec![$($x),+])
    };
}

#[macro_export]
macro_rules! and {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::and(vec![$($x),+])
    };
}

#[macro_export]
macro_rules! or {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::or(vec![$($x),+])
    };
}

pub enum SerializedWhereValue {
    Object(Vec<(String, QueryValue)>),
    List(Vec<QueryValue>),
}

pub type SerializedWhere = (String, SerializedWhereValue);
pub fn transform_equals(
    params: impl Iterator<Item = SerializedWhere>,
) -> Vec<(String, QueryValue)> {
    params
        .map(|(field, value)| {
            (
                field,
                match value {
                    SerializedWhereValue::Object(mut params) => match params
                        .iter()
                        .position(|(key, _)| key == "equals")
                        .map(|i| params.swap_remove(i))
                    {
                        Some((_, value)) => value,
                        None => QueryValue::Object(params.into_iter().collect()),
                    },
                    SerializedWhereValue::List(values) => QueryValue::List(values),
                },
            )
        })
        .collect()
}

pub struct Args<With>
where
    With: Into<Selection>,
{
    pub with_params: Vec<With>,
}

impl<With> Args<With>
where
    With: Into<Selection>,
{
    pub fn new() -> Self {
        Self {
            with_params: vec![],
        }
    }

    pub fn with(mut self, with: impl Into<With>) -> Self {
        self.with_params.push(with.into());
        self
    }
}

pub struct FindUniqueArgs<Where, With>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
{
    pub where_param: Where,
    pub with_params: Vec<With>,
}

impl<Where, With> FindUniqueArgs<Where, With>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
{
    pub fn new(where_param: Where) -> Self {
        Self {
            where_param,
            with_params: vec![],
        }
    }

    pub fn with(mut self, param: With) -> Self {
        self.with_params.push(param);
        self
    }

    pub fn to_operation(self, model: &str, mut scalar_selections: Vec<Selection>) -> Operation {
        let Self {
            where_param,
            with_params,
        } = self;

        let mut selection = Selection::builder(format!("findUnique{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            QueryValue::Object(
                transform_equals(vec![where_param].into_iter().map(Into::into))
                    .into_iter()
                    .collect(),
            ),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        Operation::Read(selection.build())
    }
}

pub struct FindManySelectionArgs {
    pub arguments: Vec<(String, QueryValue)>,
    pub nested_selections: Vec<Selection>,
}

impl FindManySelectionArgs {
    pub fn new() -> Self {
        Self {
            arguments: vec![],
            nested_selections: vec![],
        }
    }
}
pub struct FindManyArgs<Where, With, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, QueryValue)>,
    Cursor: Into<(String, QueryValue)>,
{
    pub where_params: Vec<Where>,
    pub with_params: Vec<With>,
    pub order_by_params: Vec<OrderBy>,
    pub cursor_params: Vec<Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<Where, With, OrderBy, Cursor> FindManyArgs<Where, With, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, QueryValue)>,
    Cursor: Into<(String, QueryValue)>,
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

    pub fn to_operation(self, model: &str, mut scalar_selections: Vec<Selection>) -> Operation {
        let Self {
            where_params,
            with_params,
            order_by_params,
            cursor_params,
            skip,
            take,
        } = self;

        let mut selection = Selection::builder(format!("findMany{}", model));

        selection.alias("result");

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                QueryValue::Object(
                    transform_equals(where_params.into_iter().map(Into::into))
                        .into_iter()
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
                QueryValue::Object(
                    order_by_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            );
        }

        if cursor_params.len() > 0 {
            selection.push_argument(
                "cursor".to_string(),
                QueryValue::Object(
                    cursor_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            );
        }

        skip.map(|skip| selection.push_argument("skip".to_string(), QueryValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), QueryValue::Int(take as i64)));

        Operation::Read(selection.build())
    }
}
impl<Where, With, OrderBy, Cursor> From<FindManyArgs<Where, With, OrderBy, Cursor>>
    for FindManySelectionArgs
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, QueryValue)>,
    Cursor: Into<(String, QueryValue)>,
{
    fn from(args: FindManyArgs<Where, With, OrderBy, Cursor>) -> Self {
        let FindManyArgs {
            where_params,
            with_params,
            order_by_params,
            cursor_params,
            skip,
            take,
        } = args;

        let mut selection_args = Self::new();

        if with_params.len() > 0 {
            selection_args.nested_selections = with_params.into_iter().map(Into::into).collect()
        }

        if where_params.len() > 0 {
            selection_args.arguments.push((
                "where".to_string(),
                QueryValue::Object(
                    transform_equals(where_params.into_iter().map(Into::into))
                        .into_iter()
                        .collect(),
                ),
            ));
        }

        if order_by_params.len() > 0 {
            selection_args.arguments.push((
                "orderBy".to_string(),
                QueryValue::Object(
                    order_by_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            ));
        }

        if cursor_params.len() > 0 {
            selection_args.arguments.push((
                "cursor".to_string(),
                QueryValue::Object(
                    cursor_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            ));
        }

        skip.map(|skip| {
            selection_args
                .arguments
                .push(("skip".to_string(), QueryValue::Int(skip)))
        });

        take.map(|take| {
            selection_args
                .arguments
                .push(("take".to_string(), QueryValue::Int(take)))
        });

        selection_args
    }
}
pub struct FindFirstArgs<Where, With, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, QueryValue)>,
    Cursor: Into<(String, QueryValue)>,
{
    pub where_params: Vec<Where>,
    pub with_params: Vec<With>,
    pub order_by_params: Vec<OrderBy>,
    pub cursor_params: Vec<Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<Where, With, OrderBy, Cursor> FindFirstArgs<Where, With, OrderBy, Cursor>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
    OrderBy: Into<(String, QueryValue)>,
    Cursor: Into<(String, QueryValue)>,
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

    pub fn with(mut self, param: With) -> Self {
        self.with_params.push(param);
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

    pub fn to_operation(self, model: &str, mut scalar_selections: Vec<Selection>) -> Operation {
        let Self {
            where_params,
            with_params,
            order_by_params,
            cursor_params,
            skip,
            take,
        } = self;

        let mut selection = Selection::builder(format!("findFirst{}", model));

        selection.alias("result");

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                QueryValue::Object(
                    transform_equals(where_params.into_iter().map(Into::into))
                        .into_iter()
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
                QueryValue::Object(
                    order_by_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            );
        }

        if cursor_params.len() > 0 {
            selection.push_argument(
                "cursor".to_string(),
                QueryValue::Object(
                    cursor_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            );
        }

        skip.map(|skip| selection.push_argument("skip".to_string(), QueryValue::Int(skip as i64)));
        take.map(|take| selection.push_argument("take".to_string(), QueryValue::Int(take as i64)));

        Operation::Read(selection.build())
    }
}

pub struct CreateArgs<Set, With>
where
    Set: Into<(String, QueryValue)>,
    With: Into<Selection>,
{
    pub set_params: Vec<Set>,
    pub with_params: Vec<With>,
}

impl<Set, With> CreateArgs<Set, With>
where
    Set: Into<(String, QueryValue)>,
    With: Into<Selection>,
{
    pub fn new(set_params: Vec<Set>) -> Self {
        Self {
            set_params,
            with_params: vec![],
        }
    }

    pub fn with(mut self, param: With) -> Self {
        self.with_params.push(param);
        self
    }

    pub fn to_operation(self, model: &str, mut scalar_selections: Vec<Selection>) -> Operation {
        let Self {
            set_params,
            with_params,
        } = self;

        let mut selection = Selection::builder(format!("createOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "data",
            QueryValue::Object(
                set_params
                    .into_iter()
                    .map(Into::<(String, QueryValue)>::into)
                    .collect(),
            ),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        Operation::Write(selection.build())
    }
}

pub struct UpdateArgs<Where, Set, With>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, QueryValue)>,
    With: Into<Selection>,
{
    pub where_param: Where,
    pub set_params: Vec<Set>,
    pub with_params: Vec<With>,
}
impl<Where, Set, With> UpdateArgs<Where, Set, With>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, QueryValue)>,
    With: Into<Selection>,
{
    pub fn new(where_param: Where, set_params: Vec<Set>, with_params: Vec<With>) -> Self {
        Self {
            where_param,
            set_params,
            with_params,
        }
    }

    pub fn with(mut self, param: With) -> Self {
        self.with_params.push(param);
        self
    }

    pub fn to_operation(self, model: &str, mut scalar_selections: Vec<Selection>) -> Operation {
        let Self {
            where_param,
            set_params,
            with_params,
        } = self;

        let mut selection = Selection::builder(format!("updateOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            QueryValue::Object(
                transform_equals(vec![where_param].into_iter().map(Into::into))
                    .into_iter()
                    .collect(),
            ),
        );

        selection.push_argument(
            "data",
            QueryValue::Object(
                set_params
                    .into_iter()
                    .map(Into::<(String, QueryValue)>::into)
                    .collect(),
            ),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        Operation::Write(selection.build())
    }
}

pub struct UpdateManyArgs<Where, Set>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, QueryValue)>,
{
    pub where_params: Vec<Where>,
    pub set_params: Vec<Set>,
}
impl<Where, Set> UpdateManyArgs<Where, Set>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, QueryValue)>,
{
    pub fn new(where_params: Vec<Where>, set_params: Vec<Set>) -> Self {
        Self {
            where_params,
            set_params,
        }
    }

    pub fn to_operation(self, model: &str) -> Operation {
        let Self {
            where_params,
            set_params,
        } = self;

        let mut selection = Selection::builder(format!("updateMany{}", model));

        selection.alias("result");

        selection.push_argument(
            "data",
            QueryValue::Object(
                set_params
                    .into_iter()
                    .map(Into::<(String, QueryValue)>::into)
                    .collect(),
            ),
        );

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                QueryValue::Object(
                    transform_equals(where_params.into_iter().map(Into::into))
                        .into_iter()
                        .collect(),
                ),
            );
        }

        selection.push_nested_selection(BatchResult::selection());

        Operation::Write(selection.build())
    }
}

pub struct UpsertArgs<Where, Set, With>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, QueryValue)>,
    With: Into<Selection>,
{
    pub where_param: Where,
    pub create_params: Vec<Set>,
    pub update_params: Vec<Set>,
    pub with_params: Vec<With>,
}

impl<Where, Set, With> UpsertArgs<Where, Set, With>
where
    Where: Into<SerializedWhere>,
    Set: Into<(String, QueryValue)>,
    With: Into<Selection>,
{
    pub fn new(where_param: Where) -> Self {
        Self {
            where_param,
            create_params: vec![],
            update_params: vec![],
            with_params: vec![],
        }
    }

    pub fn with(mut self, param: With) -> Self {
        self.with_params.push(param);
        self
    }
    
    pub fn create(mut self, params: Vec<Set>) -> Self {
        self.create_params = params;
        self
    }
    
    pub fn update(mut self, params: Vec<Set>) -> Self {
        self.update_params = params;
        self
    }

    pub fn to_operation(self, model: &str, mut scalar_selections: Vec<Selection>) -> Operation {
        let Self {
            where_param,
            create_params,
            update_params,
            with_params,
        } = self;

        let mut selection = Selection::builder(format!("upsertOne{}", model));

        selection.alias("result");
        
        if create_params.len() > 0 {
            selection.push_argument(
                "create",
                QueryValue::Object(
                    create_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            );
        }
        
        if update_params.len() > 0 {
            selection.push_argument(
                "update",
                QueryValue::Object(
                    update_params
                        .into_iter()
                        .map(Into::<(String, QueryValue)>::into)
                        .collect(),
                ),
            );
        }

        selection.push_argument(
            "where",
            QueryValue::Object(
                transform_equals(vec![where_param].into_iter().map(Into::into))
                    .into_iter()
                    .collect(),
            ),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        Operation::Write(selection.build())
    }
}

pub struct DeleteArgs<Where, With> {
    pub where_param: Where,
    pub with_params: Vec<With>,
}
impl<Where, With> DeleteArgs<Where, With>
where
    Where: Into<SerializedWhere>,
    With: Into<Selection>,
{
    pub fn new(where_param: Where, with_params: Vec<With>) -> Self {
        Self {
            where_param,
            with_params,
        }
    }

    pub fn with(mut self, param: With) -> Self {
        self.with_params.push(param);
        self
    }

    pub fn to_operation(self, model: &str, mut scalar_selections: Vec<Selection>) -> Operation {
        let Self {
            where_param,
            with_params,
        } = self;

        let mut selection = Selection::builder(format!("deleteOne{}", model));

        selection.alias("result");

        selection.push_argument(
            "where",
            QueryValue::Object(
                transform_equals(vec![where_param].into_iter().map(Into::into))
                    .into_iter()
                    .collect(),
            ),
        );

        if with_params.len() > 0 {
            scalar_selections.append(&mut with_params.into_iter().map(Into::into).collect());
        }
        selection.nested_selections(scalar_selections);

        Operation::Write(selection.build())
    }
}

pub struct DeleteManyArgs<Where>
where
    Where: Into<SerializedWhere>,
{
    pub where_params: Vec<Where>,
}

impl<Where> DeleteManyArgs<Where>
where
    Where: Into<SerializedWhere>,
{
    pub fn new(where_params: Vec<Where>) -> Self {
        Self { where_params }
    }

    pub fn to_operation(self, model: &str) -> Operation {
        let Self { where_params } = self;

        let mut selection = Selection::builder(format!("deleteMany{}", model));

        selection.alias("result");

        if where_params.len() > 0 {
            selection.push_argument(
                "where",
                QueryValue::Object(
                    transform_equals(where_params.into_iter().map(Into::into))
                        .into_iter()
                        .collect(),
                ),
            );
        }

        selection.push_nested_selection(BatchResult::selection());

        Operation::Write(selection.build())
    }
}
