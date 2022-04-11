use graphql_parser::parse_query;
use query_core::QuerySchemaRef;
use request_handlers::GraphQLProtocolAdapter;
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

use crate::Executor;

#[derive(Debug, Default)]

pub struct Input {
    pub name: String,
    pub fields: Vec<Field>,
    pub value: Option<Value>,
}

#[derive(Debug, Default)]
pub struct Output {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Output {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Field {
    pub name: String,
    pub list: bool,
    pub wrap_list: bool,
    pub fields: Option<Vec<Field>>,
    pub value: Option<serde_json::Value>,
}

pub struct QueryContext<'a> {
    executor: &'a Executor,
    schema: QuerySchemaRef,
}

impl<'a> QueryContext<'a> {
    pub fn new(executor: &'a Executor, schema: QuerySchemaRef) -> Self {
        Self { executor, schema }
    }
}

pub struct Query<'a> {
    pub ctx: QueryContext<'a>,
    pub operation: String,
    pub name: String,
    pub method: String,
    pub model: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal error parsing GraphQL: {0}")]
    GraphQLParse(#[from] graphql_parser::query::ParseError),

    #[error("Internal error converting GraphQL: {0}")]
    GraphQLConvert(#[from] request_handlers::HandlerError),

    #[error("Error executing query: {0}")]
    Execute(#[from] query_core::CoreError),

    #[error("Error parsing query result: {0}")]
    Parse(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl<'a> Query<'a> {
    pub async fn perform<T: DeserializeOwned>(self) -> Result<T> {
        let query_string = self.build();

        let document = parse_query(&query_string)?;
        let operation = GraphQLProtocolAdapter::convert(document, None)?;

        let data = self
            .ctx
            .executor
            .execute(None, operation, self.ctx.schema, None)
            .await?;

        let value = serde_json::to_value(data.data)?; // TODO: serialize without json conversion
        let ret = serde_json::from_value(value)?;

        Ok(ret)
    }

    pub fn build(&self) -> String {
        let mut string = String::new();

        string.push_str(&format!("{} {}", self.operation, self.name));
        string.push_str("{");
        string.push_str("result: ");

        string.push_str(&self.build_inner());

        string.push_str("}");

        return string;
    }

    fn build_inner(&self) -> String {
        let mut string = String::new();

        string.push_str(&format!("{}{}", self.method, self.model));

        if self.inputs.len() > 0 {
            string.push_str(&self.build_inputs(&self.inputs));
        }

        string.push_str(" ");

        if self.outputs.len() > 0 {
            string.push_str(&self.build_outputs(&self.outputs));
        }

        string
    }

    fn build_inputs(&self, inputs: &Vec<Input>) -> String {
        let mut string = String::new();

        string.push_str("(");

        for input in inputs {
            string.push_str(&input.name);

            string.push_str(":");

            let next = match &input.value {
                Some(value) => serde_json::to_string(value)
                    .expect(&format!("Failed to build input {}", input.name)),
                None => self.build_fields(false, false, &input.fields),
            };

            string.push_str(&next);

            string.push_str(",");
        }

        string.push_str(")");

        string
    }

    fn build_outputs(&self, outputs: &Vec<Output>) -> String {
        let mut string = String::new();

        string.push_str("{");

        for output in outputs {
            string.push_str(&output.name);
            string.push_str(" ");

            if output.inputs.len() > 0 {
                string.push_str(&self.build_inputs(&output.inputs));
            }

            if output.outputs.len() > 0 {
                string.push_str(&self.build_outputs(&output.outputs));
            }
        }

        string.push_str("}");

        string
    }

    fn build_fields(&self, list: bool, wrap_list: bool, fields: &Vec<Field>) -> String {
        let mut string = String::new();

        if !list {
            string.push_str("{");
        }

        for field in fields {
            if wrap_list {
                string.push_str("{");
            }

            if field.name != "" {
                string.push_str(&field.name);
                string.push_str(":")
            }

            if field.list {
                string.push_str("[");
            }

            if let Some(fields) = &field.fields {
                string.push_str(&self.build_fields(field.list, field.wrap_list, &fields));
            }

            if let Some(value) = &field.value {
                string.push_str(
                    &serde_json::to_string(&value)
                        .expect(&format!("Failed to build field {}", field.name)),
                );
            }

            if field.list {
                string.push_str("]");
            }

            if wrap_list {
                string.push_str("}");
            }

            string.push_str(",");
        }

        if !list {
            string.push_str("}");
        }

        string
    }
}

pub fn transform_equals(mut fields: Vec<Field>) -> Vec<Field> {
    for mut field in &mut fields {
        if let Some(fields) = &field.fields {
            if let Some(inner) = fields.iter().find(|f| f.name == "equals") {
                field.value = inner.value.clone();
                field.fields = None;
            }
        }
    }

    fields
}
