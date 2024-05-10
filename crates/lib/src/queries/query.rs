use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use crate::{PrismaClientInternals, PrismaValue, WhereInput};

pub trait QueryConvert {
    type RawType: Data;
    type ReturnValue: Data;

    /// Function for converting between raw database data and the type expected by the user.
    /// Necessary for things like raw queries
    fn convert(raw: Self::RawType) -> super::Result<Self::ReturnValue>;
}

pub trait Query<'a>: QueryConvert {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals);
}

pub trait ModelTypes {
    type Data: Data;
    type Where: WhereInput;
    type WhereUnique: WhereInput;
    type UncheckedSet: Into<(String, PrismaValue)>;
    type Set: Into<(String, PrismaValue)>;
    type With: Into<Selection>;
    type OrderBy: Into<(String, PrismaValue)>;
    type Cursor: WhereInput;

    const MODEL: &'static str;

    fn scalar_selections() -> Vec<Selection>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelReadOperation {
    FindUnique,
    FindFirst,
    FindMany,
    Count,
}

impl ModelReadOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FindUnique => "findUnique",
            Self::FindFirst => "findFirst",
            Self::FindMany => "findMany",
            Self::Count => "aggregate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelWriteOperation {
    Create,
    CreateMany,
    Update,
    UpdateMany,
    Delete,
    DeleteMany,
    Upsert,
}

impl ModelWriteOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "createOne",
            Self::CreateMany => "createMany",
            Self::Update => "updateOne",
            Self::UpdateMany => "updateMany",
            Self::Delete => "deleteOne",
            Self::DeleteMany => "deleteMany",
            Self::Upsert => "upsertOne",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelOperation {
    Read(ModelReadOperation),
    Write(ModelWriteOperation),
}

impl ModelOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Read(q) => q.name(),
            Self::Write(q) => q.name(),
        }
    }
}

pub trait ModelQuery<'a>: Query<'a> {
    type Types: ModelTypes;

    const TYPE: ModelOperation;

    fn base_selection(
        arguments: impl IntoIterator<Item = (String, PrismaValue)>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Selection::new(
            format!("{}{}", Self::TYPE.name(), Self::Types::MODEL),
            None,
            arguments
                .into_iter()
                .map(|(k, v)| (k, prisma_models::PrismaValue::from(v).into()))
                .collect::<Vec<_>>(),
            nested_selections.into_iter().collect::<Vec<_>>(),
        )
    }
}

pub trait WhereQuery<'a>: ModelQuery<'a> {
    fn add_where(&mut self, param: <<Self as ModelQuery<'a>>::Types as ModelTypes>::Where);
}

pub trait WithQuery<'a>: ModelQuery<'a> {
    fn add_with(&mut self, param: impl Into<<<Self as ModelQuery<'a>>::Types as ModelTypes>::With>);
}

pub trait OrderByQuery<'a>: ModelQuery<'a> {
    fn add_order_by(&mut self, param: <<Self as ModelQuery<'a>>::Types as ModelTypes>::OrderBy);
}

pub trait PaginatedQuery<'a>: ModelQuery<'a> {
    fn add_cursor(&mut self, param: <<Self as ModelQuery<'a>>::Types as ModelTypes>::Cursor);
    fn set_skip(&mut self, skip: i64);
    fn set_take(&mut self, take: i64);
}

pub trait UncheckedSetQuery<'a>: ModelQuery<'a> {
    fn add_unchecked_set(
        &mut self,
        param: <<Self as ModelQuery<'a>>::Types as ModelTypes>::UncheckedSet,
    );
}

pub trait SetQuery<'a>: ModelQuery<'a> {
    fn add_set(&mut self, param: <<Self as ModelQuery<'a>>::Types as ModelTypes>::Set);
}

pub trait Data: DeserializeOwned + 'static {}

impl<T: DeserializeOwned + 'static> Data for T {}
