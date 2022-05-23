use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub generator: Generator,
    pub schema_path: String,
    pub datamodel: String,
    pub datasources: Vec<Datasource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    pub output: Value,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub name: String,
    pub provider: String,
    pub url: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub from_env_var: Option<String>,
    pub value: String,
}
