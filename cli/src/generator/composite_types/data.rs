use crate::generator::prelude::*;

pub fn struct_definition(ty: &dml::CompositeType) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let fields = ty.fields.iter().map(|field| {
        let field_name_snake = snake_ident(&field.name);
        let field_ty = field.type_tokens(quote!());

        quote!(#field_name_snake: #field_ty)
    });

    quote! {
        pub struct Data {
            #(#fields),*
        }
    }
}
