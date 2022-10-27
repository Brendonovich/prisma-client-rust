use prisma_models::PrismaValue;
use query_core::{Selection, SelectionBuilder};
use serde::de::DeserializeOwned;

use crate::{ModelActionType, ModelMutationType, SerializedWhereInput};

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

    fn base_selection() -> SelectionBuilder {
        let mut selection =
            Selection::builder(format!("{}{}", Self::TYPE.name(), Self::Actions::MODEL));

        selection.alias("result");

        selection
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
