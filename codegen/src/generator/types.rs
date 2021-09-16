use serde::{Serialize, Deserialize};
use serde_json::{Value as SerdeValue};
use std::collections::HashMap;
use super::dmmf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub generator: Generator,
    pub other_generators: Vec<Generator>,
    pub schema_path: String,
    pub dmmf: dmmf::Document,
    pub datasources: Vec<Datasource>,
    pub binary_paths: Option<BinaryPaths>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub package: Option<String>,
    pub disable_gitignores: Option<String>,
    pub disable_go_binaries: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    pub output: Value,
    pub name: String,
    pub provider: Value,
    pub config: HashMap<String, String>,
    pub binary_targets: Vec<String>,
    // pub pinned_binary_target: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub from_env_var: Option<String>,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    MySQL,
    Mongo,
    SQLite,
    PostgreSQL
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub name: String,
    pub active_provider: ProviderType,
    pub provider: Vec<ProviderType>,
    pub url: EnvValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnvValue {
    pub from_env_var: Option<String>,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BinaryPaths {
    pub migration_engine: HashMap<String, String>,
    pub query_engine: HashMap<String, String>,
    pub introspection_engine: HashMap<String, String>
}