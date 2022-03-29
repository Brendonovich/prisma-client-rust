use convert_case::{Case, Casing};

use crate::generator::{
    ast::{filters::Filter, models::Field},
    GraphQLType,
};

use super::{
    dmmf::{self, FieldKind},
    filters::Method,
    AST,
};

const LIST: &str = "List";

impl<'a> AST<'a> {
    pub fn read_filters(&mut self) -> Vec<Filter> {
        let mut filters = vec![];

        for scalar in &self.scalars {
            let combinations = vec![
                vec![
                    scalar.to_string() + "ListFilter",
                    scalar.to_string() + "NullableListFilter",
                ],
                vec![
                    scalar.to_string() + "Filter",
                    scalar.to_string() + "NullableFilter",
                ],
            ];
            for c in combinations {
                let p = match self.pick(c) {
                    Some(p) => p,
                    None => continue,
                };

                let mut fields = vec![];

                for field in &p.fields {
                    if let Some(method) = convert_field(field) {
                        fields.push(method);
                    }
                }

                let mut s = scalar.clone();
                if p.name.contains("ListFilter") {
                    s += LIST
                }

                filters.push(Filter {
                    name: s,
                    methods: fields,
                })
            }
        }

        for e in &self.enums {
            let p = match self.pick(vec![
                "Enum".to_string() + &e.name + "Filter",
                "Enum".to_string() + &e.name + "NullableFilter",
            ]) {
                Some(t) => t,
                None => continue,
            };

            let mut fields = vec![];

            for field in &p.fields {
                if let Some(method) = convert_field(field) {
                    fields.push(method);
                }
            }

            filters.push(Filter {
                name: e.name.clone(),
                methods: fields,
            });
        }

        for i in 0..self.models.len() {
            let m = &self.models[i];
            let p = match self.pick(vec![m.name.to_string() + "OrderByRelevanceInput"]) {
                Some(p) => p,
                None => continue,
            };

            let mut methods = vec![];

            for field in &p.fields {
                if let Some(method) = convert_field(field) {
                    methods.push(method);
                }
            }

            filters.push(Filter {
                name: m.name.clone(),
                methods,
            });

            let field_type = GraphQLType(p.name.to_case(Case::Pascal));

            self.models[i].fields.push(Field {
                prisma: true,
                field: dmmf::Field {
                    name: "relevance".to_string(),
                    kind: FieldKind::Scalar,
                    field_type,
                    ..Default::default()
                },
            })
        }

        filters
    }

    pub fn read_filter(&self, scalar: &str, is_list: bool) -> Option<&Filter> {
        let scalar = scalar.replacen("NullableFilter", "", 1);
        let mut scalar = scalar.replacen("ReadFilter", "", 1);

        if is_list {
            scalar += LIST;
        }

        self.read_filters.iter().find(|f| f.name == scalar)
    }
}

fn convert_field(field: &dmmf::OuterInputType) -> Option<Method> {
    if field.name == "equals" {
        return None;
    }

    if let Some((type_name, is_list)) = {
        let mut ret = None;
        for input_type in &field.input_types {
            if (input_type.location == "scalar" || input_type.location == "enumTypes")
                && input_type.typ.string() != "Null"
            {
                ret = Some((input_type.typ.clone(), input_type.is_list))
            }
        }
        ret
    } {
        Some(Method {
            // 'in' is a reserved keyword in Rust
            name: match field.name.as_str() {
                "in" => "InVec".to_string(),
                "notIn" => "NotInVec".to_string(),
                name => name.to_case(Case::Pascal),
            },
            action: field.name.clone(),
            typ: type_name,
            is_list,
            ..Default::default()
        })
    } else {
        None
    }
}
