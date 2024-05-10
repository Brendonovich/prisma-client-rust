use dmmf::{DmmfTypeReference, TypeLocation};
use prisma_models::walkers::{
    CompositeTypeFieldWalker, FieldWalker, ModelWalker, RefinedFieldWalker, ScalarFieldWalker,
};
use prisma_models::FieldArity;
use psl::parser_database::{ParserDatabase, ScalarFieldType, ScalarType};

use crate::prelude::*;

pub trait ModelExt<'a> {
    fn scalar_field_has_relation(self, scalar: ScalarFieldWalker) -> bool;
    fn required_scalar_fields(self) -> Vec<FieldWalker<'a>>;
}

impl<'a> ModelExt<'a> for ModelWalker<'a> {
    fn scalar_field_has_relation(self, scalar: ScalarFieldWalker) -> bool {
        self.relation_fields().any(|relation_field| {
            relation_field
                .fields()
                .map(|mut fields| fields.any(|f| f.field_id() == scalar.field_id()))
                .unwrap_or(false)
        })
    }

    fn required_scalar_fields(self) -> Vec<FieldWalker<'a>> {
        self.fields()
            .filter(|&f| {
                f.required_on_create() && matches!(f.refine(), RefinedFieldWalker::Relation(_))
            })
            .collect()
    }
}

pub trait FieldExt<'a> {
    fn type_tokens(self, prefix: &TokenStream) -> Option<TokenStream>;

    fn type_prisma_value(self, var: &Ident) -> Option<TokenStream>;

    fn relation_methods(self) -> &'static [&'static str];

    fn required_on_create(self) -> bool;
}

impl<'a> FieldExt<'a> for FieldWalker<'a> {
    fn type_tokens(self, prefix: &TokenStream) -> Option<TokenStream> {
        match self.refine() {
            RefinedFieldWalker::Scalar(scalar_field) => scalar_field.scalar_field_type().to_tokens(
                prefix,
                &self.ast_field().arity,
                &self.db,
            ),
            RefinedFieldWalker::Relation(relation_field) => {
                let related_model_name_snake = snake_ident(relation_field.related_model().name());

                Some(
                    self.ast_field()
                        .arity
                        .wrap_type(&quote!(#prefix::#related_model_name_snake::Data)),
                )
            }
        }
    }

    fn type_prisma_value(self, var: &Ident) -> Option<TokenStream> {
        match self.refine() {
            RefinedFieldWalker::Scalar(scalar_field) => scalar_field.type_prisma_value(var),
            RefinedFieldWalker::Relation(_) => None,
        }
    }

    fn relation_methods(self) -> &'static [&'static str] {
        if self.ast_field().arity.is_list() {
            &["some", "every", "none"]
        } else {
            &["is", "is_not"]
        }
    }

    fn required_on_create(self) -> bool {
        self.ast_field().arity.is_required()
            && match self.refine() {
                RefinedFieldWalker::Scalar(scalar_field) => scalar_field.required_on_create(),
                RefinedFieldWalker::Relation(_) => true,
            }
    }
}

impl<'a> FieldExt<'a> for CompositeTypeFieldWalker<'a> {
    fn type_tokens(self, prefix: &TokenStream) -> Option<TokenStream> {
        self.r#type().to_tokens(prefix, &self.arity(), &self.db)
    }

    fn type_prisma_value(self, var: &Ident) -> Option<TokenStream> {
        self.r#type().to_prisma_value(var, &self.arity())
    }

    fn relation_methods(self) -> &'static [&'static str] {
        todo!()
    }

    fn required_on_create(self) -> bool {
        self.ast_field().arity.is_required() && self.default_value().is_none()
    }
}

pub trait FieldArityExt {
    fn wrap_type(&self, ty: &TokenStream) -> TokenStream;

    fn wrap_pv(&self, var: &Ident, pv: TokenStream) -> TokenStream;
}

impl FieldArityExt for FieldArity {
    fn wrap_type(&self, ty: &TokenStream) -> TokenStream {
        match self {
            FieldArity::List => quote!(Vec<#ty>),
            FieldArity::Optional => quote!(Option<#ty>),
            FieldArity::Required => quote!(#ty),
        }
    }

    fn wrap_pv(&self, var: &Ident, value: TokenStream) -> TokenStream {
        let pv = quote!(::prisma_client_rust::PrismaValue);

        match self {
            FieldArity::List => {
                quote!(#pv::List(#var.into_iter().map(|#var| #value).collect()))
            }
            FieldArity::Optional => {
                quote!(#var.map(|#var| #value).unwrap_or_else(|| #pv::Null))
            }
            FieldArity::Required => value,
        }
    }
}

pub trait ScalarFieldWalkerExt {
    fn is_in_required_relation(&self) -> bool;
}

impl<'a> ScalarFieldWalkerExt for ScalarFieldWalker<'a> {
    fn is_in_required_relation(&self) -> bool {
        self.model().relation_fields().any(|relation_field| {
            relation_field
                .fields()
                .map(|mut fields| fields.any(|sf| sf.field_id() == self.field_id()))
                .unwrap_or(false)
        })
    }
}

impl<'a> FieldExt<'a> for ScalarFieldWalker<'a> {
    fn type_tokens(self, prefix: &TokenStream) -> Option<TokenStream> {
        self.scalar_field_type()
            .to_tokens(prefix, &self.ast_field().arity, self.db)
    }

    fn type_prisma_value(self, var: &Ident) -> Option<TokenStream> {
        self.scalar_field_type()
            .to_prisma_value(var, &self.ast_field().arity)
    }

    fn relation_methods(self) -> &'static [&'static str] {
        &[]
    }

    fn required_on_create(self) -> bool {
        self.ast_field().arity.is_required()
            && !self.is_updated_at()
            && self.default_value().is_none()
    }
}

pub trait ScalarFieldTypeExt {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: &FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream>;
    fn to_prisma_value(&self, var: &Ident, arity: &FieldArity) -> Option<TokenStream>;
}

impl ScalarFieldTypeExt for ScalarFieldType {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: &FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream> {
        let base = match *self {
            Self::Enum(id) => {
                let name = pascal_ident(db.walk(id).name());
                quote!(#prefix #name)
            }
            Self::BuiltInScalar(typ) => typ.to_tokens(),
            Self::Unsupported(_) => return None,
            Self::CompositeType(id) => {
                let name = snake_ident(db.walk(id).name());
                quote!(#prefix #name::Data)
            }
        };

        Some(arity.wrap_type(&base))
    }

    fn to_prisma_value(&self, var: &Ident, arity: &FieldArity) -> Option<TokenStream> {
        let pv = quote!(::prisma_client_rust::PrismaValue);

        let scalar_converter = match self {
            Self::BuiltInScalar(typ) => typ.to_prisma_value(&var),
            Self::Enum(_) => quote!(#pv::Enum(#var.to_string())),
            Self::Unsupported(_) => return None,
            Self::CompositeType(_) => quote!(#pv::Object(vec![])),
        };

        Some(arity.wrap_pv(&var, scalar_converter))
    }
}

pub trait ScalarTypeExt {
    fn to_tokens(&self) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident) -> TokenStream;
    fn to_dmmf_string(&self) -> String;
}

impl ScalarTypeExt for ScalarType {
    fn to_tokens(&self) -> TokenStream {
        let ident = format_ident!("{}", self.as_str());

        quote!(#ident)
    }

    fn to_prisma_value(&self, var: &Ident) -> TokenStream {
        let pcr = quote!(::prisma_client_rust);
        let v = quote!(#pcr::PrismaValue);

        match self {
            ScalarType::Int => quote!(#v::Int(#var)),
            ScalarType::BigInt => quote!(#v::BigInt(#var)),
            ScalarType::Float => quote!(#v::Float(#var)),
            ScalarType::Decimal => quote!(#v::String(#var.to_string())),
            ScalarType::Boolean => quote!(#v::Boolean(#var)),
            ScalarType::String => quote!(#v::String(#var)),
            ScalarType::Json => quote!(#v::Json(#pcr::serde_json::to_string(&#var).unwrap())),
            ScalarType::DateTime => quote!(#v::DateTime(#var)),
            ScalarType::Bytes => quote!(#v::Bytes(#var)),
        }
    }

    fn to_dmmf_string(&self) -> String {
        match self {
            Self::Boolean => "Bool".to_string(),
            _ => self.as_str().to_string(),
        }
    }
}

pub trait DmmfTypeReferenceExt {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: &FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream>;
}

impl DmmfTypeReferenceExt for DmmfTypeReference {
    fn to_tokens(
        &self,
        prefix: &TokenStream,
        arity: &FieldArity,
        db: &ParserDatabase,
    ) -> Option<TokenStream> {
        Some(match self.location {
            TypeLocation::Scalar => {
                ScalarFieldType::BuiltInScalar(ScalarType::try_from_str(&self.typ).unwrap())
                    .to_tokens(prefix, arity, db)?
            }
            TypeLocation::EnumTypes => {
                let enum_name_pascal = pascal_ident(&self.typ);
                quote!(#prefix #enum_name_pascal)
            }
            TypeLocation::InputObjectTypes => {
                let typ = match &self.typ {
                    t if t.ends_with("OrderByWithRelationInput") => {
                        let model_name = t.replace("OrderByWithRelationInput", "");
                        let model_name_snake = snake_ident(&model_name);

                        quote!(#model_name_snake::OrderByWithRelationParam)
                    }
                    t if t.ends_with("OrderByRelationAggregateInput") => {
                        let model_name = t.replace("OrderByRelationAggregateInput", "");
                        let model_name_snake = snake_ident(&model_name);

                        quote!(#model_name_snake::OrderByRelationAggregateParam)
                    }
                    t if t.ends_with("OrderByInput") => {
                        let model_name = t.replace("OrderByInput", "");
                        let model_name_snake = snake_ident(&model_name);

                        quote!(#model_name_snake::OrderByParam)
                    }
                    _ => return None,
                };

                quote!(Vec<#prefix #typ>)
            }
            _ => return None,
        })
    }
}
