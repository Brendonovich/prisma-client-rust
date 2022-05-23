use chrono::SecondsFormat;
use prisma_models::PrismaValue;
use serde_json::{Value, json};

#[macro_export]
macro_rules! raw {
    ($e: expr) => {
        $crate::raw::Raw::new($e, vec![]);
    };
    ($e: expr, $($params:expr),+) => {
        $crate::raw::Raw::new($e, vec![$($params),+])
    };
}

pub struct Raw {
    pub(crate) query: String,
    pub values: Vec<Value>,
}

impl Raw {
    pub fn new(query: &str, values: Vec<PrismaValue>) -> Self {
        Self {
            query: query.to_string(),
            values: values
                .into_iter()
                .map(|v| match v {
                    PrismaValue::DateTime(dt) => json!({
                        "prisma__type": "date",
                        "prisma__value": dt.to_rfc3339_opts(SecondsFormat::Millis, true)
                    }),
                    v => serde_json::to_value(v).unwrap(),
                })
                .collect(),
        }
    }

    pub fn convert(self, database: &'static str) -> (String, Vec<Value>) {
        let Self { mut query, values } = self;

        for i in 0..values.len() {
            let variable_indicator = match database {
                "postgres" | "cockroachdb" => format!("${}", i),
                "sqlite" | "mysql" => "?".to_string(),
                _ => panic!("Raw queries are not supported with database '{database}'"),
            };

            query = query.replacen("{}", &variable_indicator, 1);
        }

        // TODO: do this at compile time
        if query.contains("{}") {
            panic!("Raw query has not been given enough parameters");
        }

        (query, values)
    }
}
