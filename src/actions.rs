use prisma_models::PrismaValue;
use query_core::{Selection, SelectionBuilder};
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

pub trait Action {
    type Actions: ModelActions;

    const NAME: &'static str;

    fn base_selection() -> SelectionBuilder {
        let mut selection = Selection::builder(format!("{}{}", Self::NAME, Self::Actions::MODEL));

        selection.alias("result");

        selection
    }
}
