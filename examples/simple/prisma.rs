use prisma_client_rust::builder::{Field, Input, Output, Query};
use prisma_client_rust::engine::{self, Engine, QueryEngine};
pub struct PrismaActions {}
pub struct PrismaClient {
    pub engine: Box<dyn Engine>,
}
impl PrismaClient {
    pub fn new() -> Self {
        Self { engine : Box :: new (QueryEngine :: new ("datasource db {\n    provider = \"postgresql\"\n    url      = \"postgresql://postgres:postgres@localhost:15432/testing\"\n}\n\ngenerator client {\n    provider = \"prisma-client-rust\"\n}\n\nmodel Post {\n    id         String    @id\n    name       String?\n    comments   Comment[] @relation()\n    category   Category? @relation(fields: [categoryID], references: [id])\n    categoryID String?\n}\n\nmodel Comment {\n    id   String @id\n    post Post   @relation(references: [id], fields: [id])\n}\n\nmodel Category {\n    id     String @id @default(cuid())\n    name   String\n    weight Int?\n    posts  Post[]\n}\n" . to_string () , true)) , }
    }
    pub fn post(&self) -> PostActions {
        PostActions { client: &self }
    }
    pub fn comment(&self) -> CommentActions {
        CommentActions { client: &self }
    }
    pub fn category(&self) -> CategoryActions {
        CategoryActions { client: &self }
    }
}
pub struct PostActions<'a> {
    client: &'a PrismaClient,
}
pub enum PostWhereParam {
    IdContains(String),
    IdStartsWith(String),
    IdEndsWith(String),
    IdEquals(String),
    NameContains(String),
    NameStartsWith(String),
    NameEndsWith(String),
    NameEquals(String),
    CommentsSome(Vec<CommentWhereParam>),
    CommentsEvery(Vec<CommentWhereParam>),
    CategoryIs(Vec<CategoryWhereParam>),
    CategoryIdContains(String),
    CategoryIdStartsWith(String),
    CategoryIdEndsWith(String),
    CategoryIdEquals(String),
    Not(Vec<PostWhereParam>),
    Or(Vec<PostWhereParam>),
    And(Vec<PostWhereParam>),
}
impl PostWhereParam {
    pub fn field(&self) -> Field {
        match self {
            Self::IdContains(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdStartsWith(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEndsWith(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEquals(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameContains(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameStartsWith(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameEndsWith(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameEquals(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CommentsSome(value) => Field {
                name: "comments".into(),
                fields: Some(vec![Field {
                    name: "some".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CommentsEvery(value) => Field {
                name: "comments".into(),
                fields: Some(vec![Field {
                    name: "every".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIs(value) => Field {
                name: "category".into(),
                fields: Some(vec![Field {
                    name: "is".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIdContains(value) => Field {
                name: "categoryID".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIdStartsWith(value) => Field {
                name: "categoryID".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIdEndsWith(value) => Field {
                name: "categoryID".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIdEquals(value) => Field {
                name: "categoryID".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Not(value) => Field {
                name: "Not".into(),
                fields: Some(vec![Field {
                    name: "NOT".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Or(value) => Field {
                name: "Or".into(),
                fields: Some(vec![Field {
                    name: "OR".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::And(value) => Field {
                name: "And".into(),
                fields: Some(vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        }
    }
}
pub struct PostFindMany<'a> {
    query: Query<'a>,
}
impl<'a> PostFindMany<'a> {
    pub async fn exec(self) -> Vec<PostModel> {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct PostFindFirst<'a> {
    query: Query<'a>,
}
impl<'a> PostFindFirst<'a> {
    pub async fn exec(self) -> PostModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct PostFindUnique<'a> {
    query: Query<'a>,
}
impl<'a> PostFindUnique<'a> {
    pub async fn exec(self) -> PostModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
impl<'a> PostActions<'a> {
    pub fn find_many(&self, params: Vec<PostWhereParam>) -> PostFindMany {
        let where_fields: Vec<Field> = params.iter().map(|param| param.field()).collect();
        let inputs = if where_fields.len() > 0 {
            vec![Input {
                name: "where".into(),
                fields: vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(where_fields),
                    ..Default::default()
                }],
                ..Default::default()
            }]
        } else {
            Vec::new()
        };
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findMany".into(),
            model: "Post".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("categoryID"),
            ],
            inputs,
        };
        PostFindMany { query }
    }
    pub fn find_first(&self, params: Vec<PostWhereParam>) -> PostFindFirst {
        let where_fields: Vec<Field> = params.iter().map(|param| param.field()).collect();
        let inputs = if where_fields.len() > 0 {
            vec![Input {
                name: "where".into(),
                fields: vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(where_fields),
                    ..Default::default()
                }],
                ..Default::default()
            }]
        } else {
            Vec::new()
        };
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findFirst".into(),
            model: "Post".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("categoryID"),
            ],
            inputs,
        };
        PostFindFirst { query }
    }
    pub fn find_unique(&self, param: PostWhereParam) -> PostFindUnique {
        let mut field = param.field();
        if let Some(fields) = &field.fields {
            if let Some(inner) = fields.iter().find(|f| f.name == "equals") {
                field.value = inner.value.clone();
                field.fields = None;
            }
        }
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findUnique".into(),
            model: "Post".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("categoryID"),
            ],
            inputs: vec![Input {
                name: "where".into(),
                fields: vec![field],
                ..Default::default()
            }],
        };
        PostFindUnique { query }
    }
}
pub struct CommentActions<'a> {
    client: &'a PrismaClient,
}
pub enum CommentWhereParam {
    IdContains(String),
    IdStartsWith(String),
    IdEndsWith(String),
    IdEquals(String),
    PostIs(Vec<PostWhereParam>),
    Not(Vec<CommentWhereParam>),
    Or(Vec<CommentWhereParam>),
    And(Vec<CommentWhereParam>),
}
impl CommentWhereParam {
    pub fn field(&self) -> Field {
        match self {
            Self::IdContains(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdStartsWith(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEndsWith(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEquals(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::PostIs(value) => Field {
                name: "post".into(),
                fields: Some(vec![Field {
                    name: "is".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Not(value) => Field {
                name: "Not".into(),
                fields: Some(vec![Field {
                    name: "NOT".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Or(value) => Field {
                name: "Or".into(),
                fields: Some(vec![Field {
                    name: "OR".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::And(value) => Field {
                name: "And".into(),
                fields: Some(vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        }
    }
}
pub struct CommentFindMany<'a> {
    query: Query<'a>,
}
impl<'a> CommentFindMany<'a> {
    pub async fn exec(self) -> Vec<CommentModel> {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct CommentFindFirst<'a> {
    query: Query<'a>,
}
impl<'a> CommentFindFirst<'a> {
    pub async fn exec(self) -> CommentModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct CommentFindUnique<'a> {
    query: Query<'a>,
}
impl<'a> CommentFindUnique<'a> {
    pub async fn exec(self) -> CommentModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
impl<'a> CommentActions<'a> {
    pub fn find_many(&self, params: Vec<CommentWhereParam>) -> CommentFindMany {
        let where_fields: Vec<Field> = params.iter().map(|param| param.field()).collect();
        let inputs = if where_fields.len() > 0 {
            vec![Input {
                name: "where".into(),
                fields: vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(where_fields),
                    ..Default::default()
                }],
                ..Default::default()
            }]
        } else {
            Vec::new()
        };
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findMany".into(),
            model: "Comment".into(),
            outputs: vec![Output::new("id")],
            inputs,
        };
        CommentFindMany { query }
    }
    pub fn find_first(&self, params: Vec<CommentWhereParam>) -> CommentFindFirst {
        let where_fields: Vec<Field> = params.iter().map(|param| param.field()).collect();
        let inputs = if where_fields.len() > 0 {
            vec![Input {
                name: "where".into(),
                fields: vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(where_fields),
                    ..Default::default()
                }],
                ..Default::default()
            }]
        } else {
            Vec::new()
        };
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findFirst".into(),
            model: "Comment".into(),
            outputs: vec![Output::new("id")],
            inputs,
        };
        CommentFindFirst { query }
    }
    pub fn find_unique(&self, param: CommentWhereParam) -> CommentFindUnique {
        let mut field = param.field();
        if let Some(fields) = &field.fields {
            if let Some(inner) = fields.iter().find(|f| f.name == "equals") {
                field.value = inner.value.clone();
                field.fields = None;
            }
        }
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findUnique".into(),
            model: "Comment".into(),
            outputs: vec![Output::new("id")],
            inputs: vec![Input {
                name: "where".into(),
                fields: vec![field],
                ..Default::default()
            }],
        };
        CommentFindUnique { query }
    }
}
pub struct CategoryActions<'a> {
    client: &'a PrismaClient,
}
pub enum CategoryWhereParam {
    IdContains(String),
    IdStartsWith(String),
    IdEndsWith(String),
    IdEquals(String),
    NameContains(String),
    NameStartsWith(String),
    NameEndsWith(String),
    NameEquals(String),
    WeightLt(i64),
    WeightGt(i64),
    WeightLte(i64),
    WeightGte(i64),
    WeightEquals(i64),
    PostsSome(Vec<PostWhereParam>),
    PostsEvery(Vec<PostWhereParam>),
    Not(Vec<CategoryWhereParam>),
    Or(Vec<CategoryWhereParam>),
    And(Vec<CategoryWhereParam>),
}
impl CategoryWhereParam {
    pub fn field(&self) -> Field {
        match self {
            Self::IdContains(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdStartsWith(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEndsWith(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEquals(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameContains(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameStartsWith(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameEndsWith(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameEquals(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightLt(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "lt".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightGt(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "gt".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightLte(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "lte".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightGte(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "gte".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightEquals(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(serde_json::to_value(value).unwrap()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::PostsSome(value) => Field {
                name: "posts".into(),
                fields: Some(vec![Field {
                    name: "some".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::PostsEvery(value) => Field {
                name: "posts".into(),
                fields: Some(vec![Field {
                    name: "every".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Not(value) => Field {
                name: "Not".into(),
                fields: Some(vec![Field {
                    name: "NOT".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Or(value) => Field {
                name: "Or".into(),
                fields: Some(vec![Field {
                    name: "OR".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::And(value) => Field {
                name: "And".into(),
                fields: Some(vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(value.into_iter().map(|f| f.field()).collect()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        }
    }
}
pub struct CategoryFindMany<'a> {
    query: Query<'a>,
}
impl<'a> CategoryFindMany<'a> {
    pub async fn exec(self) -> Vec<CategoryModel> {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct CategoryFindFirst<'a> {
    query: Query<'a>,
}
impl<'a> CategoryFindFirst<'a> {
    pub async fn exec(self) -> CategoryModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct CategoryFindUnique<'a> {
    query: Query<'a>,
}
impl<'a> CategoryFindUnique<'a> {
    pub async fn exec(self) -> CategoryModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
impl<'a> CategoryActions<'a> {
    pub fn find_many(&self, params: Vec<CategoryWhereParam>) -> CategoryFindMany {
        let where_fields: Vec<Field> = params.iter().map(|param| param.field()).collect();
        let inputs = if where_fields.len() > 0 {
            vec![Input {
                name: "where".into(),
                fields: vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(where_fields),
                    ..Default::default()
                }],
                ..Default::default()
            }]
        } else {
            Vec::new()
        };
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findMany".into(),
            model: "Category".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("weight"),
            ],
            inputs,
        };
        CategoryFindMany { query }
    }
    pub fn find_first(&self, params: Vec<CategoryWhereParam>) -> CategoryFindFirst {
        let where_fields: Vec<Field> = params.iter().map(|param| param.field()).collect();
        let inputs = if where_fields.len() > 0 {
            vec![Input {
                name: "where".into(),
                fields: vec![Field {
                    name: "AND".into(),
                    list: true,
                    wrap_list: true,
                    fields: Some(where_fields),
                    ..Default::default()
                }],
                ..Default::default()
            }]
        } else {
            Vec::new()
        };
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findFirst".into(),
            model: "Category".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("weight"),
            ],
            inputs,
        };
        CategoryFindFirst { query }
    }
    pub fn find_unique(&self, param: CategoryWhereParam) -> CategoryFindUnique {
        let mut field = param.field();
        if let Some(fields) = &field.fields {
            if let Some(inner) = fields.iter().find(|f| f.name == "equals") {
                field.value = inner.value.clone();
                field.fields = None;
            }
        }
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findUnique".into(),
            model: "Category".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("weight"),
            ],
            inputs: vec![Input {
                name: "where".into(),
                fields: vec![field],
                ..Default::default()
            }],
        };
        CategoryFindUnique { query }
    }
}
#[derive(serde :: Deserialize, Debug)]
pub struct PostModel {
    pub id: String,
    pub name: Option<String>,
    pub category_id: Option<String>,
    comments: Option<Vec<CommentModel>>,
    pub category: Option<CategoryModel>,
}
impl PostModel {
    pub fn comments(&self) -> Result<&Vec<CommentModel>, String> {
        match &self.comments {
            Some(v) => Ok(v),
            None => Err(
                "attempted to access comments but did not fetch it using the .with() syntax"
                    .to_string(),
            ),
        }
    }
}
#[derive(serde :: Deserialize, Debug)]
pub struct CommentModel {
    pub id: String,
    post: Option<PostModel>,
}
impl CommentModel {
    pub fn post(&self) -> Result<&PostModel, String> {
        match &self.post {
            Some(v) => Ok(v),
            None => Err(
                "attempted to access post but did not fetch it using the .with() syntax"
                    .to_string(),
            ),
        }
    }
}
#[derive(serde :: Deserialize, Debug)]
pub struct CategoryModel {
    pub id: String,
    pub name: String,
    pub weight: Option<i64>,
    posts: Option<Vec<PostModel>>,
}
impl CategoryModel {
    pub fn posts(&self) -> Result<&Vec<PostModel>, String> {
        match &self.posts {
            Some(v) => Ok(v),
            None => Err(
                "attempted to access posts but did not fetch it using the .with() syntax"
                    .to_string(),
            ),
        }
    }
}
pub struct Post {}
impl Post {
    pub fn id() -> PostId {
        PostId {}
    }
    pub fn name() -> PostName {
        PostName {}
    }
    pub fn comments() -> PostComments {
        PostComments {}
    }
    pub fn category() -> PostCategory {
        PostCategory {}
    }
    pub fn category_id() -> PostCategoryId {
        PostCategoryId {}
    }
    pub fn not(params: Vec<PostWhereParam>) -> PostWhereParam {
        PostWhereParam::Not(params)
    }
    pub fn or(params: Vec<PostWhereParam>) -> PostWhereParam {
        PostWhereParam::Or(params)
    }
    pub fn and(params: Vec<PostWhereParam>) -> PostWhereParam {
        PostWhereParam::And(params)
    }
}
pub struct PostId {}
impl PostId {
    pub fn contains(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdContains(value)
    }
    pub fn starts_with(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdStartsWith(value)
    }
    pub fn ends_with(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdEndsWith(value)
    }
    pub fn equals(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdEquals(value)
    }
}
pub struct PostName {}
impl PostName {
    pub fn contains(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameContains(value)
    }
    pub fn starts_with(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameStartsWith(value)
    }
    pub fn ends_with(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameEndsWith(value)
    }
    pub fn equals(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameEquals(value)
    }
}
pub struct PostComments {}
impl PostComments {
    pub fn some(&self, value: Vec<CommentWhereParam>) -> PostWhereParam {
        PostWhereParam::CommentsSome(value)
    }
    pub fn every(&self, value: Vec<CommentWhereParam>) -> PostWhereParam {
        PostWhereParam::CommentsEvery(value)
    }
}
pub struct PostCategory {}
impl PostCategory {
    pub fn is(&self, value: Vec<CategoryWhereParam>) -> PostWhereParam {
        PostWhereParam::CategoryIs(value)
    }
}
pub struct PostCategoryId {}
impl PostCategoryId {
    pub fn contains(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdContains(value)
    }
    pub fn starts_with(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdStartsWith(value)
    }
    pub fn ends_with(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdEndsWith(value)
    }
    pub fn equals(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdEquals(value)
    }
}
pub struct Comment {}
impl Comment {
    pub fn id() -> CommentId {
        CommentId {}
    }
    pub fn post() -> CommentPost {
        CommentPost {}
    }
    pub fn not(params: Vec<CommentWhereParam>) -> CommentWhereParam {
        CommentWhereParam::Not(params)
    }
    pub fn or(params: Vec<CommentWhereParam>) -> CommentWhereParam {
        CommentWhereParam::Or(params)
    }
    pub fn and(params: Vec<CommentWhereParam>) -> CommentWhereParam {
        CommentWhereParam::And(params)
    }
}
pub struct CommentId {}
impl CommentId {
    pub fn contains(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdContains(value)
    }
    pub fn starts_with(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdStartsWith(value)
    }
    pub fn ends_with(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdEndsWith(value)
    }
    pub fn equals(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdEquals(value)
    }
}
pub struct CommentPost {}
impl CommentPost {
    pub fn is(&self, value: Vec<PostWhereParam>) -> CommentWhereParam {
        CommentWhereParam::PostIs(value)
    }
}
pub struct Category {}
impl Category {
    pub fn id() -> CategoryId {
        CategoryId {}
    }
    pub fn name() -> CategoryName {
        CategoryName {}
    }
    pub fn weight() -> CategoryWeight {
        CategoryWeight {}
    }
    pub fn posts() -> CategoryPosts {
        CategoryPosts {}
    }
    pub fn not(params: Vec<CategoryWhereParam>) -> CategoryWhereParam {
        CategoryWhereParam::Not(params)
    }
    pub fn or(params: Vec<CategoryWhereParam>) -> CategoryWhereParam {
        CategoryWhereParam::Or(params)
    }
    pub fn and(params: Vec<CategoryWhereParam>) -> CategoryWhereParam {
        CategoryWhereParam::And(params)
    }
}
pub struct CategoryId {}
impl CategoryId {
    pub fn contains(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdContains(value)
    }
    pub fn starts_with(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdStartsWith(value)
    }
    pub fn ends_with(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdEndsWith(value)
    }
    pub fn equals(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdEquals(value)
    }
}
pub struct CategoryName {}
impl CategoryName {
    pub fn contains(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameContains(value)
    }
    pub fn starts_with(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameStartsWith(value)
    }
    pub fn ends_with(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameEndsWith(value)
    }
    pub fn equals(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameEquals(value)
    }
}
pub struct CategoryWeight {}
impl CategoryWeight {
    pub fn lt(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightLt(value)
    }
    pub fn gt(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightGt(value)
    }
    pub fn lte(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightLte(value)
    }
    pub fn gte(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightGte(value)
    }
    pub fn equals(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightEquals(value)
    }
}
pub struct CategoryPosts {}
impl CategoryPosts {
    pub fn some(&self, value: Vec<PostWhereParam>) -> CategoryWhereParam {
        CategoryWhereParam::PostsSome(value)
    }
    pub fn every(&self, value: Vec<PostWhereParam>) -> CategoryWhereParam {
        CategoryWhereParam::PostsEvery(value)
    }
}
