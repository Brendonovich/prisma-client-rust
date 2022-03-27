use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::{dmmf, AST, index::Index};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
    pub indexes: Vec<Index>,

    #[serde(rename = "-")]
    old_model: dmmf::Model,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub prisma: bool,
    #[serde(flatten)]
    pub field: dmmf::Field,
}

impl Deref for Field {
    type Target = dmmf::Field;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> AST<'a> {
    pub fn models(&self) -> Vec<Model> {
        self.dmmf
            .datamodel
            .models
            .iter()
            .map(|m| Model {
                name: m.name.clone(),
                fields: m.fields.iter().map(|f| Field {
                    prisma: false,
                    field: f.clone(),
                }).collect(),
                indexes: m.indexes(),
                old_model: (*m).clone(),
            })
            .collect()
    }
}
