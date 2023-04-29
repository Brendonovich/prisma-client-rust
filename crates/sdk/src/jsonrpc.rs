use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::default::Default;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub pretty_name: String,
    pub default_output: String,
    pub denylist: Option<Vec<String>>,
    pub requires_generators: Option<Vec<String>>,
    pub requires_engines: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestResponse {
    pub manifest: Manifest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub id: i32,
    pub method: String,
    pub params: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResponseData {
    Result(Value),
    Error { code: i32, message: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub jsonrpc: String,
    pub id: i32,
    #[serde(flatten)]
    pub data: ResponseData,
}
