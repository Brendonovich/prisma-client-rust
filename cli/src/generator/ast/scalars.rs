use super::{dmmf::Document, AST};

impl<'a> AST<'a> {
    pub fn scalars(&self) -> Vec<String> {
        let mut scalars = Vec::new();
        for scalar in &self.dmmf.schema.input_object_types.prisma {
            for field in scalar.fields {
                for input in field.input_types {
                    if input.location != "scalar" {
                        continue;
                    }

                    let name = input.typ.string();

                    if let Some(_) = scalars.iter().find(|s| s == &name) {
                        continue;
                    }

                    scalars.push(name.to_string());
                }
            }
        }
        scalars
    }
}
