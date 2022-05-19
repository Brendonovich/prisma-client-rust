Ok(
    (
        Configuration {
            generators: [
                Generator {
                    name: "client",
                    provider: StringFromEnvVar {
                        from_env_var: None,
                        value: Some(
                            "cargo prisma",
                        ),
                    },
                    output: Some(
                        StringFromEnvVar {
                            from_env_var: None,
                            value: Some(
                                "tests/db.rs",
                            ),
                        },
                    ),
                    config: {},
                    binary_targets: [],
                    preview_features: None,
                    documentation: None,
                },
                Generator {
                    name: "gql",
                    provider: StringFromEnvVar {
                        from_env_var: None,
                        value: Some(
                            "cargo prisma-gql",
                        ),
                    },
                    output: Some(
                        StringFromEnvVar {
                            from_env_var: None,
                            value: Some(
                                "tests/db-gql.rs",
                            ),
                        },
                    ),
                    config: {
                        "client_module_prefix": "crate::db",
                    },
                    binary_targets: [],
                    preview_features: None,
                    documentation: None,
                },
            ],
            datasources: [
                Datasource {
                    name: "db",
                    provider: "sqlite",
                    active_provider: "sqlite",
                    url: "<url>",
                    documentation: None,
                    active_connector: "...",
                    shadow_database_url: None,
                    referential_integrity: None,
                },
            ],
        },
        Datamodel {
            enums: [],
            models: [
                Model {
                    name: "Post",
                    fields: [
                        ScalarField(
                            ScalarField {
                                name: "id",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(cuid()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "created_at",
                                field_type: Scalar(
                                    DateTime,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(now()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "updated_at",
                                field_type: Scalar(
                                    DateTime,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: true,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "title",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "published",
                                field_type: Scalar(
                                    Boolean,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "views",
                                field_type: Scalar(
                                    Int,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Single(Int(0)),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "desc",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Optional,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "author",
                                relation_info: RelationInfo {
                                    to: "User",
                                    fields: [
                                        "author_id",
                                    ],
                                    references: [
                                        "id",
                                    ],
                                    name: "posts",
                                    fk_name: Some(
                                        "Post_author_id_fkey",
                                    ),
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: Optional,
                                referential_arity: Optional,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "author_id",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Optional,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "categories",
                                relation_info: RelationInfo {
                                    to: "Category",
                                    fields: [],
                                    references: [
                                        "id",
                                    ],
                                    name: "CategoryToPost",
                                    fk_name: None,
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: List,
                                referential_arity: List,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "favouriters",
                                relation_info: RelationInfo {
                                    to: "User",
                                    fields: [],
                                    references: [
                                        "id",
                                    ],
                                    name: "favouritePosts",
                                    fk_name: None,
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: List,
                                referential_arity: List,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                    ],
                    documentation: None,
                    database_name: None,
                    indices: [
                        IndexDefinition {
                            name: None,
                            db_name: Some(
                                "Post_title_author_id_key",
                            ),
                            fields: [
                                IndexField {
                                    path: [
                                        (
                                            "title",
                                            None,
                                        ),
                                    ],
                                    sort_order: None,
                                    length: None,
                                },
                                IndexField {
                                    path: [
                                        (
                                            "author_id",
                                            None,
                                        ),
                                    ],
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            tpe: Unique,
                            clustered: None,
                            algorithm: None,
                            defined_on_field: false,
                        },
                    ],
                    primary_key: Some(
                        PrimaryKeyDefinition {
                            name: None,
                            db_name: None,
                            fields: [
                                PrimaryKeyField {
                                    name: "id",
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            defined_on_field: true,
                            clustered: None,
                        },
                    ),
                    is_generated: false,
                    is_commented_out: false,
                    is_ignored: false,
                },
                Model {
                    name: "User",
                    fields: [
                        ScalarField(
                            ScalarField {
                                name: "id",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(cuid()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "name",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "email",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Optional,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "created_at",
                                field_type: Scalar(
                                    DateTime,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(now()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "posts",
                                relation_info: RelationInfo {
                                    to: "Post",
                                    fields: [],
                                    references: [],
                                    name: "posts",
                                    fk_name: None,
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: List,
                                referential_arity: List,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "favouritePosts",
                                relation_info: RelationInfo {
                                    to: "Post",
                                    fields: [],
                                    references: [
                                        "id",
                                    ],
                                    name: "favouritePosts",
                                    fk_name: None,
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: List,
                                referential_arity: List,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "profile",
                                relation_info: RelationInfo {
                                    to: "Profile",
                                    fields: [],
                                    references: [],
                                    name: "ProfileToUser",
                                    fk_name: None,
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: Optional,
                                referential_arity: Optional,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                    ],
                    documentation: None,
                    database_name: None,
                    indices: [
                        IndexDefinition {
                            name: None,
                            db_name: Some(
                                "User_email_key",
                            ),
                            fields: [
                                IndexField {
                                    path: [
                                        (
                                            "email",
                                            None,
                                        ),
                                    ],
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            tpe: Unique,
                            clustered: None,
                            algorithm: None,
                            defined_on_field: true,
                        },
                    ],
                    primary_key: Some(
                        PrimaryKeyDefinition {
                            name: None,
                            db_name: None,
                            fields: [
                                PrimaryKeyField {
                                    name: "id",
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            defined_on_field: true,
                            clustered: None,
                        },
                    ),
                    is_generated: false,
                    is_commented_out: false,
                    is_ignored: false,
                },
                Model {
                    name: "Category",
                    fields: [
                        ScalarField(
                            ScalarField {
                                name: "id",
                                field_type: Scalar(
                                    Int,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(autoincrement()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "posts",
                                relation_info: RelationInfo {
                                    to: "Post",
                                    fields: [],
                                    references: [
                                        "id",
                                    ],
                                    name: "CategoryToPost",
                                    fk_name: None,
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: List,
                                referential_arity: List,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "name",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                    ],
                    documentation: None,
                    database_name: None,
                    indices: [],
                    primary_key: Some(
                        PrimaryKeyDefinition {
                            name: None,
                            db_name: None,
                            fields: [
                                PrimaryKeyField {
                                    name: "id",
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            defined_on_field: true,
                            clustered: None,
                        },
                    ),
                    is_generated: false,
                    is_commented_out: false,
                    is_ignored: false,
                },
                Model {
                    name: "Profile",
                    fields: [
                        ScalarField(
                            ScalarField {
                                name: "id",
                                field_type: Scalar(
                                    Int,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(autoincrement()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        RelationField(
                            RelationField {
                                name: "user",
                                relation_info: RelationInfo {
                                    to: "User",
                                    fields: [
                                        "user_id",
                                    ],
                                    references: [
                                        "id",
                                    ],
                                    name: "ProfileToUser",
                                    fk_name: Some(
                                        "Profile_user_id_fkey",
                                    ),
                                    on_delete: None,
                                    on_update: None,
                                },
                                arity: Required,
                                referential_arity: Required,
                                documentation: None,
                                is_generated: false,
                                is_commented_out: false,
                                is_ignored: false,
                                supports_restrict_action: Some(
                                    true,
                                ),
                                emulates_referential_actions: Some(
                                    false,
                                ),
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "user_id",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "bio",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "city",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Optional,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "country",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: None,
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "views",
                                field_type: Scalar(
                                    Int,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Single(Int(0)),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                    ],
                    documentation: None,
                    database_name: None,
                    indices: [
                        IndexDefinition {
                            name: None,
                            db_name: Some(
                                "Profile_user_id_key",
                            ),
                            fields: [
                                IndexField {
                                    path: [
                                        (
                                            "user_id",
                                            None,
                                        ),
                                    ],
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            tpe: Unique,
                            clustered: None,
                            algorithm: None,
                            defined_on_field: true,
                        },
                    ],
                    primary_key: Some(
                        PrimaryKeyDefinition {
                            name: None,
                            db_name: None,
                            fields: [
                                PrimaryKeyField {
                                    name: "id",
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            defined_on_field: true,
                            clustered: None,
                        },
                    ),
                    is_generated: false,
                    is_commented_out: false,
                    is_ignored: false,
                },
                Model {
                    name: "Types",
                    fields: [
                        ScalarField(
                            ScalarField {
                                name: "id",
                                field_type: Scalar(
                                    Int,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(autoincrement()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "bool_",
                                field_type: Scalar(
                                    Boolean,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Single(Boolean(false)),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "string",
                                field_type: Scalar(
                                    String,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Single(String("")),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "integer",
                                field_type: Scalar(
                                    Int,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Single(Int(0)),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "datetime",
                                field_type: Scalar(
                                    DateTime,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Expression(now()[]),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                        ScalarField(
                            ScalarField {
                                name: "float_",
                                field_type: Scalar(
                                    Float,
                                    None,
                                    None,
                                ),
                                arity: Required,
                                database_name: None,
                                default_value: Some(
                                    DefaultValue::Single(Float(BigDecimal("0"))),
                                ),
                                documentation: None,
                                is_generated: false,
                                is_updated_at: false,
                                is_commented_out: false,
                                is_ignored: false,
                            },
                        ),
                    ],
                    documentation: None,
                    database_name: None,
                    indices: [],
                    primary_key: Some(
                        PrimaryKeyDefinition {
                            name: None,
                            db_name: None,
                            fields: [
                                PrimaryKeyField {
                                    name: "id",
                                    sort_order: None,
                                    length: None,
                                },
                            ],
                            defined_on_field: true,
                            clustered: None,
                        },
                    ),
                    is_generated: false,
                    is_commented_out: false,
                    is_ignored: false,
                },
            ],
            composite_types: [],
        },
    ),
)