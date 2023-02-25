use dml::{Field, FieldArity, FieldType, Model, ScalarField, ScalarType};

use crate::prelude::*;

pub trait ModelExt {
    fn scalar_field_has_relation(&self, scalar: &ScalarField) -> bool;
    fn required_scalar_fields(&self) -> Vec<&Field>;
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

    fn required_scalar_fields(&self) -> Vec<&Field> {
        self.fields()
            .filter(|f| f.required_on_create() && f.is_scalar_field())
            .collect()
    }
}

pub trait FieldExt {
    fn type_tokens(&self, prefix: TokenStream) -> TokenStream;

    fn type_prisma_value(&self, var: &Ident) -> TokenStream;

    fn relation_methods(&self) -> &'static [&'static str];

    fn required_on_create(&self) -> bool;
}

impl FieldExt for Field {
    fn type_tokens(&self, prefix: TokenStream) -> TokenStream {
        self.field_type().to_tokens(prefix, self.arity())
    }

    fn type_prisma_value(&self, var: &Ident) -> TokenStream {
        self.field_type().to_prisma_value(var, &self.arity())
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
    fn to_tokens(&self, prefix: TokenStream, arity: &FieldArity) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident, arity: &FieldArity) -> TokenStream;
}

impl FieldTypeExt for FieldType {
    fn to_tokens(&self, prefix: TokenStream, arity: &FieldArity) -> TokenStream {
        let base = match self {
            Self::Enum(name) => {
                let name = pascal_ident(&name);
                quote!(#prefix #name)
            }
            Self::Relation(info) => {
                let model = snake_ident(&info.referenced_model);
                quote!(#prefix #model::Data)
            }
            Self::Scalar(typ, _) => typ.to_tokens(),
            Self::Unsupported(_) => unreachable!("Unsupported fields are not generated in Prisma Client, so this should never be reached"),
            _ => unimplemented!(),
        };

        match arity {
            FieldArity::List => quote!(Vec<#base>),
            FieldArity::Optional => quote!(Option<#base>),
            FieldArity::Required => quote!(#base),
        }
    }

    fn to_prisma_value(&self, var: &Ident, arity: &FieldArity) -> TokenStream {
        let scalar_identifier = if arity.is_list() {
            format_ident!("v")
        } else {
            var.clone()
        };

        let v = quote!(::prisma_client_rust::PrismaValue);

        let scalar_converter = match self {
            Self::Scalar(typ, _) => typ.to_prisma_value(&scalar_identifier),
            Self::Enum(_) => quote!(#v::Enum(#scalar_identifier.to_string())),
            typ => unimplemented!("{:?}", typ),
        };

        match arity {
            FieldArity::List => {
                quote!(#v::List(#var.into_iter().map(|v| #scalar_converter).collect()))
            }
            FieldArity::Optional => {
                quote!(#var.map(|#var| #scalar_converter).unwrap_or_else(|| #v::Null))
            }
            FieldArity::Required => scalar_converter,
        }
    }
}

pub trait ScalarTypeExt {
    fn to_tokens(&self) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident) -> TokenStream;
}

impl ScalarTypeExt for ScalarType {
    fn to_tokens(&self) -> TokenStream {
        let pcr = quote!(::prisma_client_rust);

        match self {
            ScalarType::Int => quote!(i32),
            ScalarType::BigInt => quote!(i64),
            ScalarType::Float | ScalarType::Decimal => quote!(f64),
            ScalarType::Boolean => quote!(bool),
            ScalarType::String => quote!(String),
            ScalarType::Json => quote!(#pcr::serde_json::Value),
            ScalarType::DateTime => {
                quote!(
                    #pcr::chrono::DateTime<
                        #pcr::chrono::FixedOffset,
                    >
                )
            }
            ScalarType::Bytes => quote!(Vec<u8>),
        }
    }

    fn to_prisma_value(&self, var: &Ident) -> TokenStream {
        let pcr = quote!(::prisma_client_rust);
        let v = quote!(#pcr::PrismaValue);

        match self {
            ScalarType::Int => quote!(#v::Int(#var as i64)),
            ScalarType::BigInt => quote!(#v::BigInt(#var)),
            ScalarType::Float | ScalarType::Decimal => {
                quote!(#v::Float(<#pcr::bigdecimal::BigDecimal as #pcr::bigdecimal::FromPrimitive>::from_f64(#var).unwrap().normalized()))
            }
            ScalarType::Boolean => quote!(#v::Boolean(#var)),
            ScalarType::String => quote!(#v::String(#var)),
            ScalarType::Json => quote!(#v::Json(#pcr::serde_json::to_string(&#var).unwrap())),
            ScalarType::DateTime => quote!(#v::DateTime(#var)),
            ScalarType::Bytes => quote!(#v::Bytes(#var)),
        }
    }
}
