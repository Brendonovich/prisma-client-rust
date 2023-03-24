use crate::generator::prelude::*;

pub fn struct_definition(ty: &dml::CompositeType, module_path: &TokenStream) -> TokenStream {
    let fields = ty.fields.iter().flat_map(|field| {
        let field_name_snake = snake_ident(&field.name);
        let field_ty = field.type_tokens(module_path)?;

        Some(quote!(#field_name_snake: #field_ty))
    });

    quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Data {
            #(#fields),*
        }
    }
}
