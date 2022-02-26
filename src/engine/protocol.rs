use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct GQLResponse {
    pub data: Option<Data>,
    pub errors: Option<Vec<GQLError>>,
    pub extensions: Option<HashMap<String, Value>>,
}

#[derive(Deserialize, Debug)]

pub struct Data {
    pub result: Value,
}

#[derive(Serialize)]
pub struct GQLRequest {
    pub query: String,
    pub variables: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct GQLError {
    pub error: String,
    pub path: Option<Vec<String>>,
    pub query: Option<HashMap<String, Value>>,
}
