use prisma_models::PrismaValue;
use query_core::{Operation, Selection, SelectionBuilder};
use serde::de::DeserializeOwned;

use crate::{ModelActionType, SerializedWhereInput};

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

#[derive(Debug)]
pub struct ModelActionCallbackData {
    pub action: ModelActionType,
    pub model: &'static str,
}

pub type OperationCallback = Box<dyn Fn(&Operation)>;
pub type ModelActionCallback = Box<dyn Fn(ModelActionCallbackData)>;

pub struct ActionNotifier {
    pub operation_callbacks: Vec<super::OperationCallback>,
    pub model_action_callbacks: Vec<ModelActionCallback>,
}

impl ActionNotifier {
    pub fn new() -> Self {
        Self {
            operation_callbacks: vec![],
            model_action_callbacks: vec![],
        }
    }
}
