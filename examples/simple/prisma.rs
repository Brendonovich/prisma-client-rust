use prisma_client_rust::builder::{self, Field, Input, Output, Query};
use prisma_client_rust::engine::{self, Engine, QueryEngine};
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
    IdHasPrefix(String),
    IdHasSuffix(String),
    IdEquals(String),
    NameContains(String),
    NameHasPrefix(String),
    NameHasSuffix(String),
    NameEquals(String),
    CommentsSome(Vec<CommentWhereParam>),
    CommentsEvery(Vec<CommentWhereParam>),
    CommentsLink(Box<CommentWhereParam>),
    CategoryIs(Vec<CategoryWhereParam>),
    CategoryLink(Box<CategoryWhereParam>),
    CategoryIdContains(String),
    CategoryIdHasPrefix(String),
    CategoryIdHasSuffix(String),
    CategoryIdEquals(String),
    Not(Vec<PostWhereParam>),
    Or(Vec<PostWhereParam>),
    And(Vec<PostWhereParam>),
}
impl PostWhereParam {
    pub fn field(self) -> Field {
        match self {
            Self::IdContains(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdHasPrefix(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdHasSuffix(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEquals(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameContains(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameHasPrefix(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameHasSuffix(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameEquals(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(value.into()),
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
            Self::CommentsLink(value) => Field {
                name: "comments".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        fields: Some(builder::transform_equals(vec![value.field()])),
                        list: true,
                        wrap_list: true,
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
            Self::CategoryLink(value) => Field {
                name: "category".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        fields: Some(builder::transform_equals(vec![value.field()])),
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
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIdHasPrefix(value) => Field {
                name: "categoryID".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIdHasSuffix(value) => Field {
                name: "categoryID".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryIdEquals(value) => Field {
                name: "categoryID".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Not(value) => Field {
                name: "NOT".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
                ..Default::default()
            },
            Self::Or(value) => Field {
                name: "OR".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
                ..Default::default()
            },
            Self::And(value) => Field {
                name: "AND".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
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
    pub fn delete(self) -> PostDeleteUnique<'a> {
        PostDeleteUnique {
            query: Query {
                operation: "mutation".into(),
                method: "deleteOne".into(),
                model: "Post".into(),
                ..self.query
            },
        }
    }
}
pub struct PostCreateOne<'a> {
    query: Query<'a>,
}
impl<'a> PostCreateOne<'a> {
    pub async fn exec(self) -> PostModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct PostDeleteUnique<'a> {
    query: Query<'a>,
}
impl<'a> PostDeleteUnique<'a> {
    pub async fn exec(self) -> PostModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
impl<'a> PostActions<'a> {
    pub fn find_unique(&self, param: PostWhereParam) -> PostFindUnique {
        let fields = builder::transform_equals(vec![param.field()]);
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
                fields,
                ..Default::default()
            }],
        };
        PostFindUnique { query }
    }
    pub fn find_first(&self, params: Vec<PostWhereParam>) -> PostFindFirst {
        let where_fields: Vec<Field> = params.into_iter().map(|param| param.field()).collect();
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
    pub fn find_many(&self, params: Vec<PostWhereParam>) -> PostFindMany {
        let where_fields: Vec<Field> = params.into_iter().map(|param| param.field()).collect();
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
    pub fn create_one(&self, id: PostSetId, params: Vec<PostSetParam>) -> PostCreateOne {
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "mutation".into(),
            method: "createOne".into(),
            model: "Post".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("categoryID"),
            ],
            inputs: vec![Input {
                name: "data".into(),
                fields: params.into_iter().map(|p| p.field()).collect(),
                ..Default::default()
            }],
        };
        PostCreateOne { query }
    }
}
pub struct CommentActions<'a> {
    client: &'a PrismaClient,
}
pub enum CommentWhereParam {
    IdContains(String),
    IdHasPrefix(String),
    IdHasSuffix(String),
    IdEquals(String),
    PostIs(Vec<PostWhereParam>),
    PostLink(Box<PostWhereParam>),
    Not(Vec<CommentWhereParam>),
    Or(Vec<CommentWhereParam>),
    And(Vec<CommentWhereParam>),
}
impl CommentWhereParam {
    pub fn field(self) -> Field {
        match self {
            Self::IdContains(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdHasPrefix(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdHasSuffix(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEquals(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(value.into()),
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
            Self::PostLink(value) => Field {
                name: "post".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        fields: Some(builder::transform_equals(vec![value.field()])),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Not(value) => Field {
                name: "NOT".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
                ..Default::default()
            },
            Self::Or(value) => Field {
                name: "OR".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
                ..Default::default()
            },
            Self::And(value) => Field {
                name: "AND".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
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
    pub fn delete(self) -> CommentDeleteUnique<'a> {
        CommentDeleteUnique {
            query: Query {
                operation: "mutation".into(),
                method: "deleteOne".into(),
                model: "Comment".into(),
                ..self.query
            },
        }
    }
}
pub struct CommentCreateOne<'a> {
    query: Query<'a>,
}
impl<'a> CommentCreateOne<'a> {
    pub async fn exec(self) -> CommentModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct CommentDeleteUnique<'a> {
    query: Query<'a>,
}
impl<'a> CommentDeleteUnique<'a> {
    pub async fn exec(self) -> CommentModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
impl<'a> CommentActions<'a> {
    pub fn find_unique(&self, param: CommentWhereParam) -> CommentFindUnique {
        let fields = builder::transform_equals(vec![param.field()]);
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "query".into(),
            method: "findUnique".into(),
            model: "Comment".into(),
            outputs: vec![Output::new("id")],
            inputs: vec![Input {
                name: "where".into(),
                fields,
                ..Default::default()
            }],
        };
        CommentFindUnique { query }
    }
    pub fn find_first(&self, params: Vec<CommentWhereParam>) -> CommentFindFirst {
        let where_fields: Vec<Field> = params.into_iter().map(|param| param.field()).collect();
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
    pub fn find_many(&self, params: Vec<CommentWhereParam>) -> CommentFindMany {
        let where_fields: Vec<Field> = params.into_iter().map(|param| param.field()).collect();
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
    pub fn create_one(
        &self,
        post: CommentSetPost,
        params: Vec<CommentSetParam>,
    ) -> CommentCreateOne {
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "mutation".into(),
            method: "createOne".into(),
            model: "Comment".into(),
            outputs: vec![Output::new("id")],
            inputs: vec![Input {
                name: "data".into(),
                fields: params.into_iter().map(|p| p.field()).collect(),
                ..Default::default()
            }],
        };
        CommentCreateOne { query }
    }
}
pub struct CategoryActions<'a> {
    client: &'a PrismaClient,
}
pub enum CategoryWhereParam {
    IdContains(String),
    IdHasPrefix(String),
    IdHasSuffix(String),
    IdEquals(String),
    NameContains(String),
    NameHasPrefix(String),
    NameHasSuffix(String),
    NameEquals(String),
    WeightLT(i64),
    WeightGT(i64),
    WeightLTE(i64),
    WeightGTE(i64),
    WeightEquals(i64),
    PostsSome(Vec<PostWhereParam>),
    PostsEvery(Vec<PostWhereParam>),
    PostsLink(Box<PostWhereParam>),
    Not(Vec<CategoryWhereParam>),
    Or(Vec<CategoryWhereParam>),
    And(Vec<CategoryWhereParam>),
}
impl CategoryWhereParam {
    pub fn field(self) -> Field {
        match self {
            Self::IdContains(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdHasPrefix(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdHasSuffix(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::IdEquals(value) => Field {
                name: "id".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameContains(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "contains".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameHasPrefix(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "starts_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameHasSuffix(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "ends_with".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::NameEquals(value) => Field {
                name: "name".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightLT(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "lt".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightGT(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "gt".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightLTE(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "lte".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightGTE(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "gte".into(),
                    value: Some(value.into()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::WeightEquals(value) => Field {
                name: "weight".into(),
                fields: Some(vec![Field {
                    name: "equals".into(),
                    value: Some(value.into()),
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
            Self::PostsLink(value) => Field {
                name: "posts".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(vec![Field {
                        name: "AND".into(),
                        fields: Some(builder::transform_equals(vec![value.field()])),
                        list: true,
                        wrap_list: true,
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Not(value) => Field {
                name: "NOT".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
                ..Default::default()
            },
            Self::Or(value) => Field {
                name: "OR".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
                ..Default::default()
            },
            Self::And(value) => Field {
                name: "AND".into(),
                list: true,
                wrap_list: true,
                fields: Some(value.into_iter().map(|f| f.field()).collect()),
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
    pub fn delete(self) -> CategoryDeleteUnique<'a> {
        CategoryDeleteUnique {
            query: Query {
                operation: "mutation".into(),
                method: "deleteOne".into(),
                model: "Category".into(),
                ..self.query
            },
        }
    }
}
pub struct CategoryCreateOne<'a> {
    query: Query<'a>,
}
impl<'a> CategoryCreateOne<'a> {
    pub async fn exec(self) -> CategoryModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
pub struct CategoryDeleteUnique<'a> {
    query: Query<'a>,
}
impl<'a> CategoryDeleteUnique<'a> {
    pub async fn exec(self) -> CategoryModel {
        let request = engine::GQLRequest {
            query: self.query.build(),
            variables: std::collections::HashMap::new(),
        };
        self.query.perform(request).await
    }
}
impl<'a> CategoryActions<'a> {
    pub fn find_unique(&self, param: CategoryWhereParam) -> CategoryFindUnique {
        let fields = builder::transform_equals(vec![param.field()]);
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
                fields,
                ..Default::default()
            }],
        };
        CategoryFindUnique { query }
    }
    pub fn find_first(&self, params: Vec<CategoryWhereParam>) -> CategoryFindFirst {
        let where_fields: Vec<Field> = params.into_iter().map(|param| param.field()).collect();
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
    pub fn find_many(&self, params: Vec<CategoryWhereParam>) -> CategoryFindMany {
        let where_fields: Vec<Field> = params.into_iter().map(|param| param.field()).collect();
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
    pub fn create_one(
        &self,
        name: CategorySetName,
        params: Vec<CategorySetParam>,
    ) -> CategoryCreateOne {
        let query = Query {
            engine: self.client.engine.as_ref(),
            name: String::new(),
            operation: "mutation".into(),
            method: "createOne".into(),
            model: "Category".into(),
            outputs: vec![
                Output::new("id"),
                Output::new("name"),
                Output::new("weight"),
            ],
            inputs: vec![Input {
                name: "data".into(),
                fields: params.into_iter().map(|p| p.field()).collect(),
                ..Default::default()
            }],
        };
        CategoryCreateOne { query }
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
pub enum PostSetParam {
    Id(String),
    Name(String),
    Comments(Vec<CommentWhereParam>),
    Category(CategoryWhereParam),
    CategoryId(String),
}
impl PostSetParam {
    pub fn field(self) -> Field {
        match self {
            Self::Id(value) => Field {
                name: "id".into(),
                value: Some(value.into()),
                ..Default::default()
            },
            Self::Name(value) => Field {
                name: "name".into(),
                value: Some(value.into()),
                ..Default::default()
            },
            Self::Comments(value) => Field {
                name: "comments".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(builder::transform_equals(
                        value.into_iter().map(|item| item.field()).collect(),
                    )),
                    list: true,
                    wrap_list: true,
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::Category(value) => Field {
                name: "category".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(builder::transform_equals(vec![value.field()])),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Self::CategoryId(value) => Field {
                name: "category_id".into(),
                value: Some(value.into()),
                ..Default::default()
            },
        }
    }
}
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
pub struct PostSetId(String);
impl From<PostSetId> for PostSetParam {
    fn from(value: PostSetId) -> Self {
        Self::Id(value.0)
    }
}
impl PostId {
    pub fn contains(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdContains(value)
    }
    pub fn has_prefix(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdHasPrefix(value)
    }
    pub fn has_suffix(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdHasSuffix(value)
    }
    pub fn equals(&self, value: String) -> PostWhereParam {
        PostWhereParam::IdEquals(value)
    }
    pub fn set<T: From<PostSetId>>(&self, value: String) -> T {
        PostSetId(value).into()
    }
}
pub struct PostName {}
pub struct PostSetName(String);
impl From<PostSetName> for PostSetParam {
    fn from(value: PostSetName) -> Self {
        Self::Name(value.0)
    }
}
impl PostName {
    pub fn contains(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameContains(value)
    }
    pub fn has_prefix(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameHasPrefix(value)
    }
    pub fn has_suffix(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameHasSuffix(value)
    }
    pub fn equals(&self, value: String) -> PostWhereParam {
        PostWhereParam::NameEquals(value)
    }
    pub fn set<T: From<PostSetName>>(&self, value: String) -> T {
        PostSetName(value).into()
    }
}
pub struct PostComments {}
pub struct PostSetComments(Vec<CommentWhereParam>);
impl From<PostSetComments> for PostSetParam {
    fn from(value: PostSetComments) -> Self {
        Self::Comments(value.0.into_iter().map(|v| v.into()).collect())
    }
}
impl PostComments {
    pub fn some(&self, value: Vec<CommentWhereParam>) -> PostWhereParam {
        PostWhereParam::CommentsSome(value)
    }
    pub fn every(&self, value: Vec<CommentWhereParam>) -> PostWhereParam {
        PostWhereParam::CommentsEvery(value)
    }
    pub fn link<T: From<PostSetComments>>(&self, value: Vec<CommentWhereParam>) -> T {
        PostSetComments(value).into()
    }
}
pub struct PostCategory {}
pub struct PostSetCategory(CategoryWhereParam);
impl From<PostSetCategory> for PostSetParam {
    fn from(value: PostSetCategory) -> Self {
        Self::Category(value.0)
    }
}
impl PostCategory {
    pub fn is(&self, value: Vec<CategoryWhereParam>) -> PostWhereParam {
        PostWhereParam::CategoryIs(value)
    }
    pub fn link<T: From<PostSetCategory>>(&self, value: CategoryWhereParam) -> T {
        PostSetCategory(value).into()
    }
}
pub struct PostCategoryId {}
pub struct PostSetCategoryId(String);
impl From<PostSetCategoryId> for PostSetParam {
    fn from(value: PostSetCategoryId) -> Self {
        Self::CategoryId(value.0)
    }
}
impl PostCategoryId {
    pub fn contains(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdContains(value)
    }
    pub fn has_prefix(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdHasPrefix(value)
    }
    pub fn has_suffix(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdHasSuffix(value)
    }
    pub fn equals(&self, value: String) -> PostWhereParam {
        PostWhereParam::CategoryIdEquals(value)
    }
    pub fn set<T: From<PostSetCategoryId>>(&self, value: String) -> T {
        PostSetCategoryId(value).into()
    }
}
pub struct Comment {}
pub enum CommentSetParam {
    Id(String),
    Post(PostWhereParam),
}
impl CommentSetParam {
    pub fn field(self) -> Field {
        match self {
            Self::Id(value) => Field {
                name: "id".into(),
                value: Some(value.into()),
                ..Default::default()
            },
            Self::Post(value) => Field {
                name: "post".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(builder::transform_equals(vec![value.field()])),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        }
    }
}
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
pub struct CommentSetId(String);
impl From<CommentSetId> for CommentSetParam {
    fn from(value: CommentSetId) -> Self {
        Self::Id(value.0)
    }
}
impl CommentId {
    pub fn contains(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdContains(value)
    }
    pub fn has_prefix(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdHasPrefix(value)
    }
    pub fn has_suffix(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdHasSuffix(value)
    }
    pub fn equals(&self, value: String) -> CommentWhereParam {
        CommentWhereParam::IdEquals(value)
    }
    pub fn set<T: From<CommentSetId>>(&self, value: String) -> T {
        CommentSetId(value).into()
    }
}
pub struct CommentPost {}
pub struct CommentSetPost(PostWhereParam);
impl From<CommentSetPost> for CommentSetParam {
    fn from(value: CommentSetPost) -> Self {
        Self::Post(value.0)
    }
}
impl CommentPost {
    pub fn is(&self, value: Vec<PostWhereParam>) -> CommentWhereParam {
        CommentWhereParam::PostIs(value)
    }
    pub fn link<T: From<CommentSetPost>>(&self, value: PostWhereParam) -> T {
        CommentSetPost(value).into()
    }
}
pub struct Category {}
pub enum CategorySetParam {
    Id(String),
    Name(String),
    Weight(i64),
    Posts(Vec<PostWhereParam>),
}
impl CategorySetParam {
    pub fn field(self) -> Field {
        match self {
            Self::Id(value) => Field {
                name: "id".into(),
                value: Some(value.into()),
                ..Default::default()
            },
            Self::Name(value) => Field {
                name: "name".into(),
                value: Some(value.into()),
                ..Default::default()
            },
            Self::Weight(value) => Field {
                name: "weight".into(),
                value: Some(value.into()),
                ..Default::default()
            },
            Self::Posts(value) => Field {
                name: "posts".into(),
                fields: Some(vec![Field {
                    name: "connect".into(),
                    fields: Some(builder::transform_equals(
                        value.into_iter().map(|item| item.field()).collect(),
                    )),
                    list: true,
                    wrap_list: true,
                    ..Default::default()
                }]),
                ..Default::default()
            },
        }
    }
}
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
pub struct CategorySetId(String);
impl From<CategorySetId> for CategorySetParam {
    fn from(value: CategorySetId) -> Self {
        Self::Id(value.0)
    }
}
impl CategoryId {
    pub fn contains(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdContains(value)
    }
    pub fn has_prefix(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdHasPrefix(value)
    }
    pub fn has_suffix(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdHasSuffix(value)
    }
    pub fn equals(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::IdEquals(value)
    }
    pub fn set<T: From<CategorySetId>>(&self, value: String) -> T {
        CategorySetId(value).into()
    }
}
pub struct CategoryName {}
pub struct CategorySetName(String);
impl From<CategorySetName> for CategorySetParam {
    fn from(value: CategorySetName) -> Self {
        Self::Name(value.0)
    }
}
impl CategoryName {
    pub fn contains(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameContains(value)
    }
    pub fn has_prefix(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameHasPrefix(value)
    }
    pub fn has_suffix(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameHasSuffix(value)
    }
    pub fn equals(&self, value: String) -> CategoryWhereParam {
        CategoryWhereParam::NameEquals(value)
    }
    pub fn set<T: From<CategorySetName>>(&self, value: String) -> T {
        CategorySetName(value).into()
    }
}
pub struct CategoryWeight {}
pub struct CategorySetWeight(i64);
impl From<CategorySetWeight> for CategorySetParam {
    fn from(value: CategorySetWeight) -> Self {
        Self::Weight(value.0)
    }
}
impl CategoryWeight {
    pub fn lt(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightLT(value)
    }
    pub fn gt(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightGT(value)
    }
    pub fn lte(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightLTE(value)
    }
    pub fn gte(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightGTE(value)
    }
    pub fn equals(&self, value: i64) -> CategoryWhereParam {
        CategoryWhereParam::WeightEquals(value)
    }
    pub fn set<T: From<CategorySetWeight>>(&self, value: i64) -> T {
        CategorySetWeight(value).into()
    }
}
pub struct CategoryPosts {}
pub struct CategorySetPosts(Vec<PostWhereParam>);
impl From<CategorySetPosts> for CategorySetParam {
    fn from(value: CategorySetPosts) -> Self {
        Self::Posts(value.0.into_iter().map(|v| v.into()).collect())
    }
}
impl CategoryPosts {
    pub fn some(&self, value: Vec<PostWhereParam>) -> CategoryWhereParam {
        CategoryWhereParam::PostsSome(value)
    }
    pub fn every(&self, value: Vec<PostWhereParam>) -> CategoryWhereParam {
        CategoryWhereParam::PostsEvery(value)
    }
    pub fn link<T: From<CategorySetPosts>>(&self, value: Vec<PostWhereParam>) -> T {
        CategorySetPosts(value).into()
    }
}
