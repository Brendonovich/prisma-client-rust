use prisma_client_rust_sdk::prelude::*;

#[derive(serde::Deserialize)]
struct ExampleGenerator {
    client_path: String,
}

#[derive(thiserror::Error, serde::Serialize, Debug)]
#[error("Example Generator Error")]
struct Error;

impl PrismaGenerator for ExampleGenerator {
    const NAME: &'static str = "Example Generator";
    const DEFAULT_OUTPUT: &'static str = "./prisma-example-generator.rs";

    type Error = Error;

    fn generate(self, args: GenerateArgs) -> Result<Module, Self::Error> {
        let client_path = ident(&self.client_path);

        let mut module = Module::new(
            "",
            quote! {
                use crate::#client_path as prisma;
            },
        );

        args.schema
            .db
            .walk_models()
            .map(|model| {
                let model_name_snake = snake_ident(model.name());

                let scalar_fields = model
                    .scalar_fields()
                    .map(|sf| snake_ident(sf.name()).to_string());
                let relation_fields = model
                    .relation_fields()
                    .map(|rf| snake_ident(rf.name()).to_string());
                let id_fields = model
                    .primary_key()
                    .map(|pk| pk.fields().map(|f| snake_ident(f.name()).to_string()))
                    .unwrap();

                Module::new(
                    model.name(),
                    quote! {
                        use super::prisma;

                        impl sdk_example_lib::ExampleTrait for prisma::#model_name_snake::Data {
                            fn scalar_fields() -> Vec<&'static str> {
                                vec![#(#scalar_fields),*]
                            }
                            fn relation_fields() -> Vec<&'static str> {
                                vec![#(#relation_fields),*]
                            }
                            fn id_fields() -> Vec<&'static str> {
                                vec![#(#id_fields),*]
                            }
                        }
                    },
                )
            })
            .for_each(|model| module.add_submodule(model));

        Ok(module)
    }
}

pub fn run() {
    ExampleGenerator::run();
}
