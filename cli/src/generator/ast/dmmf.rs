use quote::{__private::TokenStream, format_ident, quote};
use serde::{Deserialize, Serialize};
use syn::Ident;

use crate::generator::GraphQLType;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum FieldKind {
    Scalar,
    Object,
    Enum,
}

impl Default for FieldKind {
    fn default() -> Self {
        FieldKind::Scalar
    }
}

impl FieldKind {
    pub fn include_in_struct(&self) -> bool {
        self == &FieldKind::Scalar || self == &FieldKind::Enum
    }

    pub fn is_relation(&self) -> bool {
        self == &FieldKind::Object
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DatamodelFieldKind {
    Scalar,
    Relation,
    Enum,
}

impl DatamodelFieldKind {
    pub fn include_in_struct(self) -> bool {
        self == DatamodelFieldKind::Scalar || self == DatamodelFieldKind::Enum
    }

    pub fn relation(self) -> bool {
        self == DatamodelFieldKind::Relation
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub datamodel: Datamodel,
    pub schema: Schema,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    pub name: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    type_name: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionType {
    name: String,
    inner_name: String,
    list: bool,
    return_list: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub name: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub name: String,
    pub methods: Vec<Method>,
}

impl Document {
    pub fn operators() -> Vec<Operator> {
        vec![
            Operator {
                name: "Not".into(),
                action: "NOT".into(),
            },
            Operator {
                name: "Or".into(),
                action: "OR".into(),
            },
            Operator {
                name: "And".into(),
                action: "AND".into(),
            },
        ]
    }

    pub fn variations() -> [ActionType; 3] {
        return [
            ActionType {
                name: "Unique".to_string(),
                inner_name: "One".to_string(),
                list: false,
                return_list: false,
            },
            ActionType {
                name: "First".to_string(),
                inner_name: "One".to_string(),
                list: true,
                return_list: false,
            },
            ActionType {
                name: "Many".to_string(),
                inner_name: "Many".to_string(),
                list: true,
                return_list: true,
            },
        ];
    }

    pub fn actions() -> [Action; 4] {
        [
            Action {
                type_name: "query".to_string(),
                name: "Find".to_string(),
            },
            Action {
                type_name: "mutation".to_string(),
                name: "Create".to_string(),
            },
            Action {
                type_name: "mutation".to_string(),
                name: "Update".to_string(),
            },
            Action {
                type_name: "mutation".to_string(),
                name: "Delete".to_string(),
            },
        ]
    }

    pub fn write_types() -> [Type; 2] {
        let number = vec![
            Method {
                name: "Increment".to_string(),
                action: "increment".to_string(),
            },
            Method {
                name: "Decrement".to_string(),
                action: "decrement".to_string(),
            },
            Method {
                name: "Multiply".to_string(),
                action: "multiply".to_string(),
            },
            Method {
                name: "Divide".to_string(),
                action: "divide".to_string(),
            },
        ];

        return [
            Type {
                name: "Int".to_string(),
                methods: number.to_vec(),
            },
            Type {
                name: "Float".to_string(),
                methods: number.to_vec(),
            },
        ];
    }

    pub fn read_types() -> Vec<Type> {
        let number = vec![
            Method {
                name: "LT".to_string(),
                action: "lt".to_string(),
            },
            Method {
                name: "GT".to_string(),
                action: "gt".to_string(),
            },
            Method {
                name: "LTE".to_string(),
                action: "lte".to_string(),
            },
            Method {
                name: "GTE".to_string(),
                action: "gte".to_string(),
            },
        ];

        vec![
            Type {
                name: "String".to_string(),
                methods: vec![
                    Method {
                        name: "Contains".to_string(),
                        action: "contains".to_string(),
                    },
                    Method {
                        name: "HasPrefix".to_string(),
                        action: "starts_with".to_string(),
                    },
                    Method {
                        name: "HasSuffix".to_string(),
                        action: "ends_with".to_string(),
                    },
                ],
            },
            Type {
                name: "Boolean".to_string(),
                methods: vec![],
            },
            Type {
                name: "Int".to_string(),
                methods: number.clone(),
            },
            Type {
                name: "Float".to_string(),
                methods: number.clone(),
            },
            Type {
                name: "DateTime".to_string(),
                methods: vec![
                    Method {
                        name: "Before".to_string(),
                        action: "lt".to_string(),
                    },
                    Method {
                        name: "After".to_string(),
                        action: "gt".to_string(),
                    },
                    Method {
                        name: "BeforeEquals".to_string(),
                        action: "lte".to_string(),
                    },
                    Method {
                        name: "AfterEquals".to_string(),
                        action: "gte".to_string(),
                    },
                ],
            },
        ]
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaEnum {
    pub name: String,
    pub values: Vec<String>,
    #[serde(rename = "dBName")]
    pub db_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub db_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub db_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datamodel {
    pub models: Vec<Model>,
    pub enums: Vec<Enum>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniqueIndex {
    #[serde(default)]
    pub internal_name: String,
    pub fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub db_name: Option<String>,
    pub fields: Vec<Field>,
    pub is_generated: Option<bool>,
    pub documentation: Option<String>,
    pub primary_key: Option<PrimaryKey>,
    pub unique_fields: Vec<Vec<String>>,
    pub unique_indexes: Vec<UniqueIndex>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub kind: FieldKind,
    pub name: String,
    pub is_list: bool,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_read_only: bool,
    #[serde(rename = "type")]
    pub field_type: GraphQLType,
    pub has_default_value: bool,
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub relation_name: String,
    pub relation_from_fields: Option<Vec<String>>,
    pub relation_to_fields: Option<Vec<String>>,
    pub relation_on_delete: Option<String>,
    pub is_generated: Option<bool>,
    pub is_updated_at: bool,
    pub documentation: Option<String>,
}

pub struct RelationMethod {
    pub name: String,
    pub action: String,
}

impl Field {
    pub fn required_on_create(&self) -> bool {
        if !self.is_required || self.is_updated_at || self.has_default_value || self.is_read_only {
            return false;
        }

        if &self.relation_name != "" && self.is_list {
            return false;
        }

        true
    }

    pub fn relation_methods(&self) -> Vec<RelationMethod> {
        if self.is_list {
            vec![
                RelationMethod {
                    name: "some".to_string(),
                    action: "some".to_string(),
                },
                RelationMethod {
                    name: "every".to_string(),
                    action: "every".to_string(),
                },
                RelationMethod {
                    name: "none".to_string(),
                    action: "none".to_string(),
                },
            ]
        } else {
            vec![
                RelationMethod {
                    name: "is".to_string(),
                    action: "is".to_string(),
                },
                RelationMethod {
                    name: "is_not".to_string(),
                    action: "isNot".to_string(),
                },
            ]
        }
    }

    pub fn type_as_query_value(&self, var: &Ident) -> TokenStream {
        if self.is_list {
            let converter = self.field_type.to_prisma_value(&format_ident!("v"));
            quote!(QueryValue::List(#var.into_iter().map(|v| #converter).collect()))
        } else {
            let t = self.field_type.to_prisma_value(var);

            quote!(#t.into())
        }
    }
}

impl Model {
    pub fn relation_fields_plus_one(self) -> Vec<Field> {
        let mut fields = self
            .fields
            .to_vec()
            .iter()
            .filter(|&f| f.kind.is_relation())
            .map(|f| f.clone())
            .collect::<Vec<Field>>();

        fields.push(Field {
            ..Default::default()
        });

        fields
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaInputType {
    pub is_required: Option<bool>,
    pub is_list: bool,
    #[serde(rename = "type")]
    pub typ: GraphQLType,
    #[serde(default)]
    pub kind: FieldKind,
    #[serde(default)]
    pub namespace: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaOutputType {
    type_: String,
    is_list: bool,
    is_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaField {
    name: String,
    output_type: SchemaOutputType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OuterInputType {
    pub name: String,
    pub input_types: Vec<SchemaInputType>,
    pub is_relation_filter: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputType {
    name: String,
    fields: Vec<SchemaField>,
    is_embedded: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaArg {
    pub name: String,
    pub input_types: Vec<SchemaInputType>,
    pub is_relation_filter: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputObjectType {
    pub prisma: Vec<CoreType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputObjectType {
    prisma: Vec<OutputType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumTypes {
    pub prisma: Vec<SchemaEnum>,
    #[serde(default)]
    pub model: Vec<SchemaEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    // root_query_type: String,
    // root_mutation_type: String,
    pub input_object_types: InputObjectType,
    pub output_object_types: OutputObjectType,
    pub enum_types: EnumTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoreType {
    pub name: String,
    pub is_where_type: Option<bool>,
    pub is_order_type: Option<bool>,
    pub at_least_one: Option<bool>,
    pub at_most_one: Option<bool>,
    pub fields: Vec<OuterInputType>,
}
