use serde::{Deserialize, Serialize};
use serde_json::Map;

/// Provided by Prisma CLI to generators
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorCtx {
    pub generator: Generator,
    pub schema_path: String,
    #[serde(rename = "datamodel")]
    pub datamodel_str: String,
    pub datasources: Vec<Datasource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    pub output: EnvValue,
    pub name: String,
    pub binary_targets: Vec<String>,
    pub provider: EnvValue,
    pub is_custom_output: bool,
    pub preview_features: Vec<String>,
    pub config: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub name: String,
    pub provider: String,
    pub url: EnvValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvValue {
    from_env_var: Option<String>,
    value: Option<String>,
}

impl EnvValue {
    pub fn get_value(&self) -> String {
        match &self.from_env_var {
            Some(env_var) => match std::env::var(env_var) {
                Ok(val) => val,
                Err(_) => panic!("env var {} not found", env_var),
            },
            None => match &self.value {
                Some(val) => val.clone(),
                None => panic!("value not found"),
            },
        }
    }
}
