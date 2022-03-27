use convert_case::{Casing, Case};

use crate::generator::ast::{filters::Filter, models::Field};

use super::{
    dmmf::{self, Document, FieldKind},
    filters::Method,
    AST,
};

const LIST: &str = "List";

impl<'a> AST<'a> {
    pub fn read_filters(&mut self) -> Vec<Filter> {
        let mut filters = vec![];

        for scalar in self.scalars {
            let combinations = vec![
                vec![scalar + "ListFilter", scalar + "NullableListFilter"],
                vec![scalar + "Filter", scalar + "NullableFilter"],
            ];
            for c in combinations {
                let p = match self.pick(c) {
                    Some(p) => p,
                    None => continue,
                };

                let mut fields = vec![];

                for field in p.fields {
                    if let Some(method) = convert_field(field) {
                        fields.push(method);
                    }
                }

                let mut s = String::new();
                if p.name.contains("ListFilter") {
                    s += list
                }
                filters.push(Filter {
                    name: s,
                    methods: fields,
                })
            }
        }

        for e in self.enums {
            let p = match self.pick(vec![
                "Enum".to_string() + &e.name + "Filter",
                "Enum".to_string() + &e.name + "NullableFilter",
            ]) {
                Some(t) => t,
                None => continue,
            };

            let mut fields = vec![];

            for field in p.fields {
                if let Some(method) = convert_field(field) {
                    fields.push(method);
                }
            }

            filters.push(Filter {
                name: e.name.clone(),
                methods: fields,
            });
        }

        for (i, m) in self.models.iter().enumerate() {
            let p = match self.pick(vec![m.name.to_string() + "OrderByRelevanceInput"]) {
                Some(p) => p,
                None => continue,
            };

            let mut methods = vec![];

            for field in p.fields {
                if let Some(method) = convert_field(field) {
                    methods.push(method);
                }
            }

            filters.push(Filter {
                name: m.name.clone(),
                methods,
            });

            self.models[i].fields.push(Field {
                prisma: true,
                field: dmmf::Field {
                    name: "relevance".to_string(),
                    kind: FieldKind::Scalar,
                    field_type: GraphQLType(p.name.to_case(Case::Pascal)),
                    ..Default::default()
                },
            })
        }

        filters
    }

    pub fn read_filter(&self, scalar: &str, is_list: bool) -> Option<&Filter> {
        let scalar = scalar.replacen("NullableFilter", "", 1);
        let scalar = scalar.replacen("ReadFilter", "", 1);

        if is_list {
            scalar += LIST;
        }

        self.read_filters.iter().find(|f| f.name == scalar)
    }

    pub fn deprecated_read_filters(&self) -> Vec<Filter> {
        let number_filters = vec![
            Method {
                name: "LT".to_string(),
                action: "lt".to_string(),
                deprecated: "Lt".to_string(),
                ..Default::default()
            },
            Method {
                name: "LTE".to_string(),
                action: "lte".to_string(),
                deprecated: "Lte".to_string(),
                ..Default::default()
            },
            Method {
                name: "GT".to_string(),
                action: "gt".to_string(),
                deprecated: "Gt".to_string(),
                ..Default::default()
            },
            Method {
                name: "GTE".to_string(),
                action: "gte".to_string(),
                deprecated: "Gte".to_string(),
                ..Default::default()
            },
        ];

        vec![
            Filter {
                name: "Int".to_string(),
                methods: numberFilters.clone(),
            },
            Filter {
                name: "Float".to_string(),
                methods: numberFilters,
            },
            Filter {
                name: "String".to_string(),
                methods: vec![
                    Method {
                        name: "HasPrefix".to_string(),
                        action: "starts_with".to_string(),
                        deprecated: "StartsWith".to_string(),
                        ..Default::default()
                    },
                    Method {
                        name: "HasSuffix".to_string(),
                        action: "ends_with".to_string(),
                        deprecated: "EndsWith".to_string(),
                        ..Default::default()
                    },
                ],
            },
            Filter {
                name: "DateTime".to_string(),
                methods: vec![
                    Method {
                        name: "Before".to_string(),
                        action: "lt".to_string(),
                        deprecated: "Lt".to_string(),
                        ..Default::default()
                    },
                    Method {
                        name: "After".to_string(),
                        action: "gt".to_string(),
                        deprecated: "Gt".to_string(),
                        ..Default::default()
                    },
                    Method {
                        name: "BeforeEquals".to_string(),
                        action: "lte".to_string(),
                        deprecated: "Lte".to_string(),
                        ..Default::default()
                    },
                    Method {
                        name: "AfterEquals".to_string(),
                        action: "gte".to_string(),
                        deprecated: "Gte".to_string(),
                        ..Default::default()
                    },
                ],
            },
        ]
    }
}

fn convert_field(field: dmmf::OuterInputType) -> Option<Method> {
    if field.name == "equals" {
        return None;
    }

    if let Some((type_name, is_list)) = {
        let mut ret = None;
        for input_type in field.input_types {
            if (input_type.location == "scalar" || input_type.location == "enumTypes")
                && input_type.typ.string() != "Null"
            {
                ret = Some((input_type.typ, input_type.is_list))
            }
        }
        ret
    } {
        Some(Method {
            name: field.name.to_case(Case::Pascal),
            action: field.name.clone(),
            typ: type_name,
            is_list,
            ..Default::default()
        })
    } else {
        None
    }
}
