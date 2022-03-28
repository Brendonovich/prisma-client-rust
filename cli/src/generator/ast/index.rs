use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use super::dmmf::Model;

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

        if self
            .primary_key
            .as_ref()
            .map(|p| p.fields.clone())
            .unwrap_or_default()
            .len()
            > 0
        {
            let primary_key_fields = self
                .primary_key
                .as_ref()
                .map(|p| p.fields.clone())
                .unwrap_or_default();

            idx.push(Index {
                name: get_name(&primary_key_fields.join("_"), &primary_key_fields),
                internal_name: primary_key_fields.join("_"),
                fields: primary_key_fields,
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
