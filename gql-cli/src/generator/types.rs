use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn::Ident;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub generator: Generator,
    pub schema_path: String,
    pub datamodel: String,
}

fn default_package() -> String {
    "./db".into()
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    pub output: Value,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub from_env_var: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GraphQLType(pub String);

impl GraphQLType {
    pub fn string(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> String {
        self.tokens().to_string()
    }

    pub fn tokens(&self) -> TokenStream {
        let string = self.string();

        match string {
            "Int" => quote!(i32),
            "BigInt" => quote!(i64),
            "Float" | "Decimal" => quote!(f64),
            "Boolean" => quote!(bool),
            "Bytes" => quote!(Vec<u8>),
            "DateTime" => quote!(chrono::DateTime<chrono::FixedOffset>),
            "Json" => quote!(serde_json::Value),
            "String" => quote!(String),
            "QueryMode" => quote!(super::QueryMode),
            _ => {
                let ident = format_ident!("{}", string.to_case(Case::Pascal));
                quote!(#ident)
            }
        }
    }

    pub fn to_prisma_value(&self, var: &Ident) -> TokenStream {
        match self.string() {
            "Int" => quote!(PrismaValue::Int(#var as i64)),
            "BigInt" => quote!(PrismaValue::BigInt(#var)),
            "Float" | "Decimal" => {
                quote!(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(#var).unwrap().normalized()))
            }
            "Boolean" => quote!(PrismaValue::Boolean(#var)),
            "Bytes" => quote!(PrismaValue::Bytes(#var)),
            "DateTime" => quote!(PrismaValue::DateTime(#var)),
            "Json" => quote!(PrismaValue::Json(serde_json::to_string(&#var).unwrap())),
            "String" => quote!(PrismaValue::String(#var)),
            "QueryMode" => quote!(PrismaValue::String(#var.to_string())),
            t => panic!("Unsupported type: {t}"),
        }
    }

    pub fn to_query_value(&self, var: &Ident, is_list: bool) -> TokenStream {
        if is_list {
            let converter = self.to_prisma_value(&format_ident!("v"));
            quote!(QueryValue::List(#var.into_iter().map(|v| #converter.into()).collect()))
        } else {
            let t = self.to_prisma_value(var);

            quote!(#t.into())
        }
    }
}
