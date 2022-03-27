use convert_case::{Casing, Case};
use serde::{Deserialize, Serialize};

use super::dmmf::{self, Model};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    name: String,
    internal_name: String,
    fields: Vec<String>,
}

impl Model {
    pub fn indexes(&self) -> Vec<Index> {
        let mut idx = self
            .unique_indexes
            .iter()
            .map(|i| {
                let internal_name = match i.internal_name.as_str() {
                    "" => i.fields.join("_"),
                    name => name.to_string(),
                };

                Index {
                    name: get_name(&i.internal_name, &i.fields),
                    internal_name: internal_name.clone(),
                    fields: i.fields.clone(),
                }
            })
            .collect::<Vec<_>>();

        if self.primary_key.fields.len() > 0 {
            idx.push(Index {
                name: get_name(
                    &self.primary_key.fields.join("_"),
                    &self.primary_key.fields,
                ),
                internal_name: model.primary_key.fields.join("_"),
                fields: model.primary_key.fields.clone(),
            });
        }

        idx
    }
}

fn get_name(field: &str, fields: &Vec<String>) -> String {
    if field != "" {
        return field.to_string();
    }

    fields
        .iter()
        .map(|s| s.to_case(Case::Pascal))
        .collect::<Vec<String>>()
        .join("")
}
