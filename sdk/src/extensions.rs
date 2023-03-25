use dml::{
    CompositeTypeField, CompositeTypeFieldType, Field, FieldArity, FieldType, Model, ScalarField,
    ScalarType,
};

use crate::prelude::*;

pub trait ModelExt {
    fn scalar_field_has_relation(&self, scalar: &ScalarField) -> bool;
    fn required_scalar_fields(&self) -> Vec<&Field>;
}

impl ModelExt for Model {
    fn scalar_field_has_relation(&self, scalar: &ScalarField) -> bool {
        self.fields.iter().any(|field| {
            if let FieldType::Relation(info) = field.field_type() {
                field.arity().is_required() && info.fields.iter().any(|f| f == &scalar.name)
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
    fn type_tokens(&self, prefix: &TokenStream) -> Option<TokenStream>;

    fn type_prisma_value(&self, var: &Ident) -> Option<TokenStream>;

    fn relation_methods(&self) -> &'static [&'static str];

    fn required_on_create(&self) -> bool;
}

impl FieldExt for Field {
    fn type_tokens(&self, prefix: &TokenStream) -> Option<TokenStream> {
        self.field_type().to_tokens(prefix, self.arity())
    }

    fn type_prisma_value(&self, var: &Ident) -> Option<TokenStream> {
        self.field_type().to_prisma_value(var, self.arity())
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

impl FieldExt for CompositeTypeField {
    fn type_tokens(&self, prefix: &TokenStream) -> Option<TokenStream> {
        self.r#type.to_tokens(prefix, &self.arity)
    }

    fn type_prisma_value(&self, var: &Ident) -> Option<TokenStream> {
        self.r#type.to_prisma_value(var, &self.arity)
    }

    fn relation_methods(&self) -> &'static [&'static str] {
        todo!()
    }

    fn required_on_create(&self) -> bool {
        self.arity.is_required() && self.default_value.is_none()
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

pub trait FieldTypeExt {
    fn to_tokens(&self, prefix: &TokenStream, arity: &FieldArity) -> Option<TokenStream>;
    fn to_prisma_value(&self, var: &Ident, arity: &FieldArity) -> Option<TokenStream>;
}

impl FieldTypeExt for FieldType {
    fn to_tokens(&self, module_path: &TokenStream, arity: &FieldArity) -> Option<TokenStream> {
        let base = match self {
            Self::Enum(name) => {
                let name = pascal_ident(name);
                quote!(#module_path::#name)
            }
            Self::Relation(info) => {
                let model = snake_ident(&info.referenced_model);
                quote!(#module_path::#model::Data)
            }
            Self::Scalar(typ, _) => typ.to_tokens(),
            Self::Unsupported(_) => return None,
            Self::CompositeType(name) => {
                let ct = snake_ident(&name);
                quote!(#module_path::#ct::Data)
            }
        };

        Some(arity.wrap_type(&base))
    }

    fn to_prisma_value(&self, var: &Ident, arity: &FieldArity) -> Option<TokenStream> {
        let pv = quote!(::prisma_client_rust::PrismaValue);

        let scalar_converter = match self {
            Self::Scalar(typ, _) => typ.to_prisma_value(&var),
            Self::Enum(_) => quote!(#pv::Enum(#var.to_string())),
            Self::Unsupported(_) => return None,
            Self::CompositeType(_) => quote!(#pv::Object(vec![])),
            _ => todo!(),
        };

        Some(arity.wrap_pv(&var, scalar_converter))
    }
}

impl FieldTypeExt for CompositeTypeFieldType {
    fn to_tokens(&self, module_path: &TokenStream, arity: &FieldArity) -> Option<TokenStream> {
        let base = match self {
            Self::Enum(name) => {
                let name = pascal_ident(name);
                quote!(#module_path::#name)
            }
            Self::Scalar(typ, _) => typ.to_tokens(),
            Self::Unsupported(_) => return None,
            Self::CompositeType(name) => {
                let ty = snake_ident(&name);
                quote!(#module_path::#ty::Data)
            }
        };

        Some(arity.wrap_type(&base))
    }

    fn to_prisma_value(&self, var: &Ident, arity: &FieldArity) -> Option<TokenStream> {
        let v = quote!(::prisma_client_rust::PrismaValue);

        let scalar_converter = match self {
            Self::Scalar(typ, _) => typ.to_prisma_value(&var),
            Self::Enum(_) => quote!(#v::Enum(#var.to_string())),
            Self::Unsupported(_) => return None,
            typ => unimplemented!("{:?}", typ),
        };

        Some(arity.wrap_pv(var, scalar_converter))
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
