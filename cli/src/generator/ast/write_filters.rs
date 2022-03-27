use convert_case::{Case, Casing};

use super::{filters::Filter, AST};

impl<'a> AST<'a> {
    pub fn write_filters(&self) -> Vec<Filter> {
        let mut filters = vec![];

        for scalar in self.scalars {
            let p = match self.pick(vec![
                scalar.clone() + "FieldUpdateOperationsInput",
                "Nullable".to_string() + &scalar + "FieldUpdateOperationsInput",
            ]) {
                Some(p) => p,
                None => continue,
            };

            let mut fields = vec![];

            for field in p.fields {
                if field.name == "set" {
                    continue;
                }

                if let Some((type_name, is_list)) = {
                    let mut ret = None;
                    for input_type in field.input_types {
                        if input_type.location == "scalar" && input_type.typ.string() != "Null" {
                            ret = Some((input_type.typ.clone(), input_type.is_list))
                        }
                    }
                    ret
                } {
                    fields.append(Method {
                        name: field.name.to_case(Case::Pascal),
                        action: field.name.clone(),
                        typ: type_name,
                        is_list,
                    });
                }
            }
            filters.push(filter {
                name: scalar.clone(),
                methods: fields,
            });
        }

        for model in self.models {
            for field in model.fields {
                let p = match self.pick(vec![model.name + "Update" + &field.name + "Input"]) {
                    Some(p) => p,
                    None => continue,
                };

                if let Some(scalar_name) = {
                    let mut scalar_name = None;

                    for field in p.fields {
                        if field.name == "set" {
                            for input_type in field.input_types {
                                if input_type.location == "scalar"
                                    && input_type.typ.string() != "null"
                                {
                                    scalar_name =
                                        Some(input_type.typ.string().to_string() + "List");
                                }
                            }

                            continue;
                        }

                        if let Some((type_name, is_list)) = {
                            let ret = None;

                            for input_type in field.input_types {
                                if input_type.location == "scalar"
                                    && input_type.typ.string() != "null"
                                {
                                    ret = Some((input_type.typ.clone(), input_type.is_list))
                                }
                            }

                            ret
                        } {
                            fields.push(Method {
                                name: field.name.to_case(Case::Pascal),
                                action: field.name.clone(),
                                typ: type_name,
                                is_list,
                            })
                        };
                    }

                    scalar_name
                } {
                    filters.push(Filter {
                        name: scalar_name,
                        methods: fields,
                    })
                }
            }
        }

        filters
    }

    pub fn write_filter(&self, scalar: &str, is_list: bool) -> Option<&Filter> {
        let scalar = if is_list {
            format!("{}List", scalar)
        } else {
            scalar.to_string()
        };

        self.write_filters.iter().find(|f| f.name == scalar)
    }
}
