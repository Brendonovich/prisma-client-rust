use prisma_models::PrismaValue;
use query_core::{Selection, SelectionArgument};
use serde::de::DeserializeOwned;

use crate::SerializedWhereInput;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
            Self::Update => "updateOne",
            Self::UpdateMany => "updateMany",
            Self::Delete => "deleteOne",
            Self::DeleteMany => "deleteMany",
            Self::Upsert => "upsertOne",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

pub trait ModelActions {
    type Data: DeserializeOwned;
    type Where: WhereInput;
    type Set: Into<(String, PrismaValue)>;
    type With: Into<Selection>;
    type OrderBy: Into<(String, PrismaValue)>;
    type Cursor: Into<Self::Where>;

    const MODEL: &'static str;

    fn scalar_selections() -> Vec<Selection>;
}

pub trait WhereInput {
    fn serialize(self) -> SerializedWhereInput;
}

pub trait ModelAction {
    type Actions: ModelActions;

    const TYPE: ModelActionType;

    fn base_selection(
        arguments: impl IntoIterator<Item = SelectionArgument>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Selection::new(
            format!("{}{}", Self::TYPE.name(), Self::Actions::MODEL),
            Some("result".to_string()),
            arguments.into_iter().collect::<Vec<_>>(),
            nested_selections.into_iter().collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModelMutationCallbackData {
    pub action: ModelMutationType,
    pub model: &'static str,
}

pub type ModelMutationCallback = Box<dyn Fn(ModelMutationCallbackData) + Sync + Send>;

pub struct ActionNotifier {
    pub model_mutation_callbacks: Vec<ModelMutationCallback>,
}

impl ActionNotifier {
    pub fn new() -> Self {
        Self {
            model_mutation_callbacks: vec![],
        }
    }
}
