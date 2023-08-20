use serde::{Deserialize, Serialize};
use serde_json::Map;

/// Provided by Prisma CLI to generators
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineDMMF {
    pub generator: Generator,
    pub schema_path: String,
    pub datamodel: String,
    pub datasources: Vec<Datasource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    pub provider: EnvValue,
    pub output: EnvValue,
    pub name: String,
    #[serde(default)]
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
        self.from_env_var
            .as_ref()
            .and_then(|o| match o.as_str() {
                // dmmf is cringe apparently?
                "null" => None,
                env_var => {
                    Some(std::env::var(env_var).expect(&format!("env var {env_var} not found")))
                }
            })
            .unwrap_or_else(|| self.value.clone().expect("value not found"))
    }
}
