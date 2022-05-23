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
    pub output: EnvValue,
    pub name: String,
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
    pub from_env_var: Option<String>,
    pub value: String,
}

impl EnvValue {
    pub fn get_value(&self) -> String {
        match &self.from_env_var {
            Some(env_var) => match std::env::var(env_var) {
                Ok(val) => val,
                Err(_) => unreachable!("env var {} not found", env_var),
            },
            None => match &self.value {
                Some(val) => val.clone(),
                None => unreachable!("value not found"),
            },
        }
    }
}
