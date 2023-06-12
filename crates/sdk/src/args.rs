use prisma_models::{walkers::ScalarFieldWalker, FieldArity};
use psl::{
    builtin_connectors,
    datamodel_connector::Connector,
    parser_database::{ScalarFieldType, ScalarType},
    ValidatedSchema,
};
use std::collections::HashSet;

use dmmf::{
    DataModelMetaFormat, DmmfInputField, DmmfInputType, DmmfSchema, DmmfTypeReference, TypeLocation,
};
use proc_macro2::TokenStream;

use crate::{dmmf::EngineDMMF, prelude::*};

pub struct GenerateArgs<'a> {
    pub schema: &'a ValidatedSchema,
    pub engine_dmmf: EngineDMMF,
    pub dmmf: &'a DataModelMetaFormat,
    pub read_filters: Vec<Filter<'a>>,
    pub write_params: Vec<Filter<'a>>,
    pub connector: &'static dyn Connector,
}

impl<'a> GenerateArgs<'a> {
    pub fn new(
        schema: &'a ValidatedSchema,
        dmmf: &'a DataModelMetaFormat,
        engine_dmmf: EngineDMMF,
    ) -> Self {
        std::fs::write("./bruh.json", serde_json::to_string(dmmf).unwrap()).ok();

        let scalars = dmmf
            .schema
            .input_object_types
            .get("prisma")
            .unwrap()
            .iter()
            .flat_map(|scalar| {
                scalar.fields.iter().flat_map(|field| {
                    field.input_types.iter().flat_map(|input| {
                        matches!(input.location, TypeLocation::Scalar)
                            .then(|| ScalarType::try_from_str(&input.typ))
                            .flatten()
                    })
                })
            })
            .collect::<HashSet<_>>();

        let read_filters = {
            let mut filters = vec![];

            for scalar in &scalars {
                let possible_filters = [
                    scalar.to_dmmf_string() + "ListFilter",
                    scalar.to_dmmf_string() + "NullableListFilter",
                    scalar.to_dmmf_string() + "Filter",
                    scalar.to_dmmf_string() + "NullableFilter",
                ];

                filters.extend(possible_filters.iter().filter_map(|filter| {
                    let filter_type = dmmf.schema.find_input_type(&filter)?;

                    let mut s = scalar.as_str().to_string();

                    // checking for both is invalid - fields can be list or null but not both
                    // TODO: make this more typesafe to correspond with fields
                    if filter_type.name.contains("List") {
                        s += "List";
                    } else if filter_type.name.contains("Nullable") {
                        s += "Nullable";
                    }

                    Some(Filter {
                        name: s,
                        root: filter_type,
                        fields: filter_type.fields.iter().collect(),
                    })
                }));
            }

            for enm in schema.db.walk_enums() {
                let possible_filters = [
                    "Enum".to_string() + &enm.ast_enum().name.name + "Filter",
                    "Enum".to_string() + &enm.ast_enum().name.name + "NullableFilter",
                ];

                filters.extend(possible_filters.iter().filter_map(|filter| {
                    let filter_type = dmmf.schema.find_input_type(&filter)?;

                    let mut name = enm.ast_enum().name.name.clone();

                    // checking for both is invalid - fields can be list or null but not both
                    // TODO: make this more typesafe to correspond with fields
                    if filter_type.name.contains("List") {
                        name += "List";
                    } else if filter_type.name.contains("Nullable") {
                        name += "Nullable";
                    }

                    Some(Filter {
                        name,
                        root: filter_type,
                        fields: filter_type.fields.iter().collect(),
                    })
                }));
            }

            // for i in 0..dml.models.len() {
            //     let m = &dml.models[i];
            //     let p =
            //         match schema.find_input_type(&(m.name.to_string() + "OrderByRelevanceInput")) {
            //             Some(p) => p,
            //             None => continue,
            //         };

            //     let mut methods = vec![];

            //     for field in &p.fields {
            //         if let Some(method) = input_field_as_method(field) {
            //             methods.push(method);
            //         }
            //     }

            //     filters.push(Filter {
            //         name: m.name.clone(),
            //         methods,
            //     });

            //     dml.models[i]
            //         .fields
            //         .push(Field::ScalarField(ScalarField::new(
            //             "relevance",
            //             FieldArity::Optional,
            //             FieldType::Enum(p.name.clone()),
            //         )));
            // }

            filters
        };

        let write_filters = {
            let mut filters = vec![];

            filters.extend(scalars.iter().flat_map(|scalar| {
                let possible_inputs = [
                    format!("{}FieldUpdateOperationsInput", scalar.to_dmmf_string()),
                    format!(
                        "Nullable{}FieldUpdateOperationsInput",
                        scalar.to_dmmf_string()
                    ),
                ];

                possible_inputs.into_iter().filter_map(|input| {
                    let input_type = dmmf.schema.find_input_type(&input)?;

                    let mut s = scalar.as_str().to_string();

                    if input_type.name.contains("List") {
                        s += "List";
                    } else if input_type.name.contains("Nullable") {
                        s += "Nullable";
                    }

                    Some(Filter {
                        name: s,
                        root: input_type,
                        fields: input_type
                            .fields
                            .iter()
                            .filter_map(|field| {
                                field.input_types.iter().find(|input_type| {
                                    match input_type.location {
                                        TypeLocation::Scalar if input_type.typ != "Null" => true,
                                        _ => false,
                                    }
                                })?;

                                Some(field)
                            })
                            .collect(),
                    })
                })
            }));

            filters.extend(schema.db.walk_enums().flat_map(|enm| {
                let possible_inputs = [
                    format!("Enum{}FieldUpdateOperationsInput", enm.name()),
                    format!("NullableEnum{}FieldUpdateOperationsInput", enm.name()),
                ];

                possible_inputs.into_iter().filter_map(move |input| {
                    let input_type = dmmf.schema.find_input_type(&input)?;

                    let mut name = enm.name().to_string();

                    if input_type.name.contains("List") {
                        name += "List";
                    } else if input_type.name.contains("Nullable") {
                        name += "Nullable";
                    }

                    Some(Filter {
                        name,
                        root: input_type,
                        fields: input_type
                            .fields
                            .iter()
                            .filter_map(|field| {
                                field.input_types.iter().find(|input_type| {
                                    match input_type.location {
                                        TypeLocation::Scalar if input_type.typ != "Null" => true,
                                        _ => true,
                                    }
                                })?;

                                Some(field)
                            })
                            .collect(),
                    })
                })
            }));

            filters.extend(schema.db.walk_models().flat_map(|model| {
                model
                    .fields()
                    .filter_map(|field| {
                        let input_type = dmmf.schema.find_input_type(&format!(
                            "{}Update{}Input",
                            model.name(),
                            field.name()
                        ))?;

                        let mut fields = vec![];

                        let scalar_name = {
                            let mut scalar_name = None;

                            fields.extend(input_type.fields.iter().filter_map(|field| {
                                if field.name == "set" {
                                    for input_type in &field.input_types {
                                        match input_type.location {
                                            TypeLocation::Scalar if input_type.typ != "null" => {
                                                scalar_name = Some(input_type.typ.clone() + "List");
                                            }
                                            _ => {}
                                        }
                                    }

                                    return None;
                                }

                                field
                                    .input_types
                                    .iter()
                                    .find(|input_type| match input_type.location {
                                        TypeLocation::Scalar if input_type.typ != "null" => true,
                                        _ => false,
                                    })
                                    .map(|_| field)
                            }));

                            scalar_name
                        }?;

                        Some(Filter {
                            name: scalar_name,
                            root: input_type,
                            fields,
                        })
                    })
                    .collect::<Vec<_>>()
            }));

            filters
        };

        use builtin_connectors::*;
        let connector = match &engine_dmmf.datasources[0].provider {
            #[cfg(feature = "sqlite")]
            p if SQLITE.is_provider(p) => SQLITE,
            #[cfg(feature = "postgresql")]
            p if POSTGRES.is_provider(p) => POSTGRES,
            #[cfg(feature = "postgresql")]
            p if COCKROACH.is_provider(p) => COCKROACH,
            #[cfg(feature = "mssql")]
            p if MSSQL.is_provider(p) => MSSQL,
            #[cfg(feature = "mysql")]
            p if MYSQL.is_provider(p) => MYSQL,
            #[cfg(feature = "mongodb")]
            p if MONGODB.is_provider(p) => MONGODB,
            p => panic!(
                "Database provider {p} is not available. Have you enabled its Cargo.toml feature?"
            ),
        };

        Self {
            schema,
            dmmf,
            engine_dmmf,
            read_filters,
            write_params: write_filters,
            connector,
        }
    }

    pub fn read_filter(&self, field: ScalarFieldWalker) -> Option<&Filter> {
        let postfix = match field.ast_field().arity {
            FieldArity::List => "List",
            FieldArity::Optional => "Nullable",
            _ => "",
        };

        let base = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(typ) => typ.as_str(),
            ScalarFieldType::Enum(e) => field.db.walk(e).name(),
            _ => return None,
        };

        self.read_filters
            .iter()
            .find(|f| f.name == format!("{base}{postfix}"))
    }

    pub fn write_param(&self, field: ScalarFieldWalker) -> Option<&Filter> {
        let postfix = match field.ast_field().arity {
            FieldArity::List => "List",
            FieldArity::Optional => "Nullable",
            _ => "",
        };

        let base = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(typ) => typ.as_str(),
            ScalarFieldType::Enum(e) => field.db.walk(e).name(),
            _ => return None,
        };

        self.write_params
            .iter()
            .find(|f| f.name == format!("{base}{postfix}"))
    }
}

pub trait DmmfSchemaExt {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType>;
}

impl DmmfSchemaExt for DmmfSchema {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType> {
        self.input_object_types
            .get("prisma")
            .and_then(|t| t.iter().find(|i| i.name == name))
    }
}

pub trait DmmfInputTypeExt {
    fn is_enum(&self) -> bool;
}

impl DmmfInputTypeExt for DmmfInputType {
    fn is_enum(&self) -> bool {
        self.fields.iter().all(|f| !f.is_required)
    }
}

pub trait DmmfInputFieldExt {
    fn arity(&self) -> FieldArity;
    fn raw_type_tokens(&self, prefix: &TokenStream, args: &GenerateArgs) -> TokenStream;
    fn type_tokens(
        &self,
        prefix: &TokenStream,
        parent: &DmmfInputType,
        args: &GenerateArgs,
    ) -> TokenStream;
    fn raw_prisma_value(&self, var: &Ident) -> TokenStream;
    fn to_prisma_value(
        &self,
        var: &Ident,
        parent: &DmmfInputType,
        args: &GenerateArgs,
    ) -> TokenStream;
    fn primary_input_type(&self) -> &DmmfTypeReference;
    fn extra_data(&self, parent: &DmmfInputType, args: &GenerateArgs) -> FieldExtraData;
}

impl DmmfInputFieldExt for DmmfInputField {
    fn primary_input_type(&self) -> &DmmfTypeReference {
        self.input_types
            .iter()
            .fold(&self.input_types[0], |prev, next| {
                if matches!(next.location, TypeLocation::InputObjectTypes)
                    && !matches!(prev.location, TypeLocation::InputObjectTypes)
                {
                    return next;
                }

                if matches!(next.location, TypeLocation::InputObjectTypes)
                    && next.typ.ends_with("Filter")
                {
                    return next;
                }

                if next.is_list && !prev.is_list {
                    return next;
                }

                prev
            })
    }

    fn arity(&self) -> FieldArity {
        let input_type = self.primary_input_type();

        if input_type.is_list {
            FieldArity::List
        } else if self.is_nullable {
            FieldArity::Optional
        } else {
            FieldArity::Required
        }
    }

    fn raw_type_tokens(&self, prefix: &TokenStream, args: &GenerateArgs) -> TokenStream {
        let input_type = self.primary_input_type();

        match input_type.location {
            TypeLocation::Scalar => ScalarType::try_from_str(&input_type.typ)
                .unwrap()
                .to_tokens(),
            TypeLocation::EnumTypes => {
                let typ: TokenStream = input_type.typ.parse().unwrap();
                quote!(#prefix #typ)
            }
            TypeLocation::InputObjectTypes => {
                let input_type = args.dmmf.schema.find_input_type(&input_type.typ).unwrap();
                let typ: TokenStream = input_type.name.parse().unwrap();

                quote!(#prefix #typ)
            }
            _ => todo!(),
        }
    }

    fn type_tokens(
        &self,
        prefix: &TokenStream,
        parent: &DmmfInputType,
        args: &GenerateArgs,
    ) -> TokenStream {
        let extra_data = self.extra_data(parent, args);

        extra_data.arity.wrap_type(
            &extra_data
                .meta_wrapper
                .wrap_type(self.raw_type_tokens(prefix, args)),
        )
    }

    fn raw_prisma_value(&self, var: &Ident) -> TokenStream {
        let pv = quote!(::prisma_client_rust::PrismaValue);

        let input_type = self.primary_input_type();

        match input_type.location {
            TypeLocation::Scalar => ScalarType::try_from_str(&input_type.typ)
                .unwrap()
                .to_prisma_value(var),
            TypeLocation::EnumTypes => quote!(#pv::Enum(#var.to_string())),
            TypeLocation::InputObjectTypes => {
                quote!(#var.into())
            }
            _ => todo!(),
        }
    }

    fn to_prisma_value(
        &self,
        var: &Ident,
        parent: &DmmfInputType,
        args: &GenerateArgs,
    ) -> TokenStream {
        let extra_data = self.extra_data(parent, args);

        extra_data.arity.wrap_pv(
            var,
            extra_data
                .meta_wrapper
                .wrap_pv(var, self.raw_prisma_value(var)),
        )

        // let inner = if input_type.is_list
        //     || input_type.typ.ends_with("RelationInput")
        //     || input_type.typ.ends_with("AggregateInput")
        //     || (input_type.typ.starts_with("Nested") && input_type.typ.ends_with("Filter"))
        //     || input_type.typ.ends_with("WhereInput")
        // {
        //     quote!(#var.into_iter().map(|value| value.into()).collect())
        // } else if input_type.typ.ends_with("UniqueInput") {
        //     return;
        // } else {
        //     quote!(vec![#var.into()])
        // };

        // let object = quote!(::prisma_client_rust::PrismaValue::Object(#inner));

        // if self.is_nullable {
        //     arity.wrap_pv(var, object)
        // } else {
        //     object
        // }
    }

    fn extra_data(&self, parent: &DmmfInputType, args: &GenerateArgs) -> FieldExtraData {
        let arity = self.arity();
        let input_type = self.primary_input_type();

        match input_type.location {
            TypeLocation::Scalar | TypeLocation::EnumTypes => FieldExtraData {
                arity,
                meta_wrapper: MetaWrapper::None,
            },
            TypeLocation::InputObjectTypes => {
                return FieldExtraData {
                    meta_wrapper: {
                        if parent.name.ends_with("RelationFilter")
                            || parent.name.ends_with("NullableFilter")
                            || (parent.name.ends_with("Filter"))
                            || (parent.name.ends_with("Input")
                                && (input_type.typ.ends_with("RelationInput")
                                    || input_type.typ.ends_with("AggregateInput")))
                        {
                            MetaWrapper::Vec
                        } else if (input_type.typ.ends_with("Input")
                            && !input_type.typ.ends_with("CompoundUniqueInput"))
                            || input_type.typ.ends_with("Filter")
                        {
                            MetaWrapper::Object
                        } else {
                            MetaWrapper::None
                        }
                    },
                    arity: {
                        if input_type.typ.ends_with("NullableFilter")
                            && parent.name.ends_with("Input")
                        {
                            FieldArity::Required
                        } else {
                            arity
                        }
                    },
                };
            }
            _ => todo!(),
        }
    }
}

pub enum MetaWrapper {
    Box,
    Vec,
    Object,
    None,
}

impl MetaWrapper {
    pub fn wrap_type(&self, typ: TokenStream) -> TokenStream {
        match self {
            Self::Box => quote!(Box<#typ>),
            Self::Vec => quote!(Vec<#typ>),
            Self::Object => typ,
            Self::None => typ,
        }
    }

    pub fn wrap_pv(&self, var: &Ident, value: TokenStream) -> TokenStream {
        let pv = quote!(::prisma_client_rust::PrismaValue);

        match self {
            Self::Box => value,
            Self::Vec => quote!(#pv::Object(#var.into_iter().map(|#var| #value).collect())),
            Self::Object => quote!(#pv::Object(vec![#value])),
            Self::None => value,
        }
    }
}

pub struct FieldExtraData {
    pub meta_wrapper: MetaWrapper,
    pub arity: FieldArity,
}

#[derive(Debug)]
pub struct Filter<'a> {
    pub name: String,
    pub root: &'a DmmfInputType,
    pub fields: Vec<&'a DmmfInputField>,
}
