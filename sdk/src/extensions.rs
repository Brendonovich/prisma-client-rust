use datamodel::dml::{Field, FieldArity, FieldType, Model, ScalarField, ScalarType};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{Case, Casing};

pub trait ModelExt {
    fn scalar_field_has_relation(&self, scalar: &ScalarField) -> bool;
}

impl ModelExt for Model {
    fn scalar_field_has_relation(&self, scalar: &ScalarField) -> bool {
        self.fields.iter().any(|field| {
            if let FieldType::Relation(info) = field.field_type() {
                info.fields.iter().any(|f| f == &scalar.name)
            } else {
                false
            }
        })
    }
}

pub trait FieldExt {
    fn type_tokens(&self) -> TokenStream;

    fn type_prisma_value(&self, var: &Ident) -> TokenStream;

    fn relation_methods(&self) -> &'static [&'static str];

    fn required_on_create(&self) -> bool;
}

impl FieldExt for Field {
    fn type_tokens(&self) -> TokenStream {
        let single_type = self.field_type().to_tokens();

        match self.arity() {
            FieldArity::Required => single_type,
            FieldArity::Optional => quote! { Option<#single_type> },
            FieldArity::List => quote! { Vec<#single_type> },
        }
    }

    fn type_prisma_value(&self, var: &Ident) -> TokenStream {
        self.field_type()
            .to_prisma_value(var, self.arity().is_list())
    }

    fn relation_methods(&self) -> &'static [&'static str] {
        if self.arity().is_list() {
            &["some", "every", "none"]
        } else {
            &["is", "is_not"]
        }
    }

    fn required_on_create(&self) -> bool {
        self.arity().is_required()
            && !self.is_updated_at()
            && self.default_value().is_none()
            && !matches!(self, Field::RelationField(r) if r.arity.is_list())
    }
}
pub trait FieldTypeExt {
    fn to_tokens(&self) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident, is_list: bool) -> TokenStream;
}

impl FieldTypeExt for FieldType {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Self::Enum(name) => {
                let name = format_ident!("{}", name.to_case(Case::Pascal));
                quote!(#name)
            }
            Self::Relation(info) => {
                let model = format_ident!("{}", info.to.to_case(Case::Snake));
                quote!(#model::Data)
            }
            Self::Scalar(typ, _) => typ.to_tokens(),
            _ => unimplemented!(),
        }
    }

    fn to_prisma_value(&self, var: &Ident, is_list: bool) -> TokenStream {
        let scalar_identifier = if is_list {
            format_ident!("v")
        } else {
            var.clone()
        };

        let scalar_converter = match self {
            Self::Scalar(typ, _) => typ.to_prisma_value(&scalar_identifier),
            Self::Enum(_) => quote!(PrismaValue::Enum(#scalar_identifier.to_string())),
            typ => unimplemented!("{:?}", typ),
        };

        if is_list {
            quote!(PrismaValue::List(#var.into_iter().map(|v| #scalar_converter).collect()))
        } else {
            scalar_converter
        }
    }
}

pub trait ScalarTypeExt {
    fn to_tokens(&self) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident) -> TokenStream;
}

impl ScalarTypeExt for ScalarType {
    fn to_tokens(&self) -> TokenStream {
        match self {
            ScalarType::Int => quote!(i32),
            ScalarType::BigInt => quote!(i64),
            ScalarType::Float | ScalarType::Decimal => quote!(f64),
            ScalarType::Boolean => quote!(bool),
            ScalarType::String => quote!(String),
            ScalarType::Json => quote!(serde_json::Value),
            ScalarType::DateTime => quote!(chrono::DateTime<chrono::FixedOffset>),
            ScalarType::Bytes => quote!(Vec<u8>),
        }
    }

    fn to_prisma_value(&self, var: &Ident) -> TokenStream {
        match self {
            ScalarType::Int => quote!(PrismaValue::Int(#var as i64)),
            ScalarType::BigInt => quote!(PrismaValue::BigInt(#var)),
            ScalarType::Float | ScalarType::Decimal => {
                quote!(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(#var).unwrap().normalized()))
            }
            ScalarType::Boolean => quote!(PrismaValue::Boolean(#var)),
            ScalarType::String => quote!(PrismaValue::String(#var)),
            ScalarType::Json => quote!(PrismaValue::Json(serde_json::to_string(&#var).unwrap())),
            ScalarType::DateTime => quote!(PrismaValue::DateTime(#var)),
            ScalarType::Bytes => quote!(PrismaValue::Bytes(#var)),
        }
    }
}
