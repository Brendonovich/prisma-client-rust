pub mod batch;
pub mod count;
pub mod create;
pub mod create_many;
pub mod delete;
pub mod delete_many;
pub mod execute_raw;
pub mod find_first;
pub mod find_many;
pub mod find_unique;
pub mod include;
pub mod query_raw;
pub mod select;
pub mod update;
pub mod update_many;
pub mod upsert;

pub use batch::*;
pub use count::*;
pub use create::*;
pub use create_many::*;
pub use delete::*;
pub use delete_many::*;
pub use execute_raw::*;
pub use find_first::*;
pub use find_many::*;
pub use find_unique::*;
pub use query_raw::*;
pub use update::*;
pub use update_many::*;
pub use upsert::*;

pub use query_core::{schema::QuerySchemaRef, Operation, Selection};

#[derive(Debug)]
pub enum ModelQueryType {
    FindUnique,
    FindFirst,
    FindMany,
    Count,
}

impl ModelQueryType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FindUnique => "findUnique",
            Self::FindFirst => "findFirst",
            Self::FindMany => "findMany",
            Self::Count => "aggregate",
        }
    }
}

#[derive(Debug)]
pub enum ModelMutationType {
    Create,
    CreateMany,
    Update,
    UpdateMany,
    Delete,
    DeleteMany,
    Upsert,
}

impl ModelMutationType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "createOne",
            Self::CreateMany => "createMany",
            Self::Update => "update",
            Self::UpdateMany => "updateMany",
            Self::Delete => "delete",
            Self::DeleteMany => "deleteMany",
            Self::Upsert => "upsertOne",
        }
    }
}

#[derive(Debug)]
pub enum ModelActionType {
    Query(ModelQueryType),
    Mutation(ModelMutationType),
}

impl ModelActionType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Query(q) => q.name(),
            Self::Mutation(q) => q.name(),
        }
    }
}

pub enum SerializedWhereValue {
    Object(Vec<(String, prisma_models::PrismaValue)>),
    List(Vec<prisma_models::PrismaValue>),
}

impl Into<prisma_models::PrismaValue> for SerializedWhereValue {
    fn into(self) -> prisma_models::PrismaValue {
        match self {
            Self::Object(v) => prisma_models::PrismaValue::Object(v),
            Self::List(v) => prisma_models::PrismaValue::List(v),
        }
    }
}

pub struct SerializedWhere {
    field: String,
    value: SerializedWhereValue,
}

impl SerializedWhere {
    pub fn new(field: &str, value: SerializedWhereValue) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
        }
    }

    /// If the parameter is an 'equals' parameter, collapses the value provided directly
    /// into the where clause. This is necessary for unique queries that have no filters,
    /// only direct value comparisons.
    pub fn transform_equals(self) -> (String, prisma_models::PrismaValue) {
        let Self { field, value } = self;

        (
            field,
            match value {
                SerializedWhereValue::Object(mut params) => match params
                    .iter()
                    .position(|(key, _)| key == "equals")
                    .map(|i| params.swap_remove(i))
                {
                    Some((_, value)) => value,
                    None => prisma_models::PrismaValue::Object(params),
                },
                SerializedWhereValue::List(values) => prisma_models::PrismaValue::List(values),
            },
        )
    }
}

impl Into<(String, prisma_models::PrismaValue)> for SerializedWhere {
    fn into(self) -> (String, prisma_models::PrismaValue) {
        let SerializedWhere { field, value } = self;
        (field, value.into())
    }
}
