use prisma_models::PrismaValue;
use query_core::{Operation, Selection, SelectionBuilder};
use serde::de::DeserializeOwned;

use crate::SerializedWhere;

pub trait ModelActions {
    type Data: DeserializeOwned;
    type Where: Into<SerializedWhere>;
    type Set: Into<(String, PrismaValue)>;
    type With: Into<Selection>;
    type OrderBy: Into<(String, PrismaValue)>;
    type Cursor: Into<Self::Where>;

    const MODEL: &'static str;

    fn scalar_selections() -> Vec<Selection>;
}

pub trait ModelAction {
    type Actions: ModelActions;

    const NAME: &'static str;

    fn base_selection() -> SelectionBuilder {
        let mut selection = Selection::builder(format!("{}{}", Self::NAME, Self::Actions::MODEL));

        selection.alias("result");

        selection
    }
}

pub struct ActionCallbackData {
    pub action: &'static str,
    pub model: &'static str,
}

pub type OperationCallback = Box<dyn Fn(&Operation)>;
pub type ActionCallback = Box<dyn Fn(&ActionCallbackData)>;

pub struct ActionNotifier {
    pub operation_callbacks: Vec<super::OperationCallback>,
    pub action_callbacks: Vec<ActionCallback>,
}

impl ActionNotifier {
    pub fn new() -> Self {
        Self {
            operation_callbacks: vec![],
            action_callbacks: vec![],
        }
    }
}
