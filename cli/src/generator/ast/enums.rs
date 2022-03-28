use serde::{Deserialize, Serialize};

use super::AST;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

impl<'a> AST<'a> {
    pub fn enums(&self) -> Vec<Enum> {
        self.dmmf
            .schema
            .enum_types
            .model
            .iter()
            .map(|e| Enum {
                name: e.name.clone(),
                values: e.values.clone(),
            })
            .collect()
    }
}
