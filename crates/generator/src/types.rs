use indexmap::IndexMap;
use std::collections::BTreeMap;
use syn::{parse_quote, ItemEnum, ItemStruct, Path};

use super::prelude::*;

const PRISMA: &str = "prisma";
const TYPES: &str = "_types";

pub fn modules(args: &GenerateArgs) -> Vec<Module> {
    let schema = &args.dmmf.schema;

    let input_types = schema.input_object_types.get(PRISMA).unwrap();
    let models = schema.output_object_types.get("model").unwrap();

    let model_names = {
        let mut ret = models.iter().map(|m| &m.name).collect::<Vec<_>>();

        ret.sort_by(|l, r| {
            if l.len() > r.len() {
                std::cmp::Ordering::Less
            } else if l.len() < r.len() {
                std::cmp::Ordering::Greater
            } else {
                l.cmp(r)
            }
        });

        ret
    };

    let item_name_mappings = {
        struct Mapping<'a> {
            parents: Vec<&'a str>,
            ident: Ident,
            raw: &'a str,
        }

        impl<'a> Mapping<'a> {
            pub fn raw(&self) -> &str {
                self.raw
            }

            pub fn path(&self) -> Path {
                let Self { ident, parents, .. } = &self;

                let parents = (!parents.is_empty())
                    .then(|| parents.iter().map(|p| snake_ident(p)))
                    .map(|i| quote!(#(#i)::*::));

                parse_quote!(#parents #ident)
            }
        }

        let mut ret = BTreeMap::<&str, Mapping>::new();

        for model in &model_names {
            for input_type in input_types {
                let type_name = input_type.name.as_str();

                if ret.contains_key(type_name) {
                    continue;
                }

                if !(type_name.starts_with(model.as_str())
                    && (type_name.ends_with("Input")
                        || type_name.ends_with("RelationFilter")
                        || type_name.ends_with("InputEnvelope")))
                {
                    continue;
                }

                let new_ident = format_ident!("{}", type_name.trim_start_matches(model.as_str()));

                ret.insert(
                    type_name,
                    Mapping {
                        parents: vec![model],
                        ident: new_ident,
                        raw: type_name,
                    },
                );
            }
        }

        for input_type in input_types {
            let type_name = input_type.name.as_str();

            ret.entry(type_name).or_insert_with(|| Mapping {
                parents: vec![TYPES],
                ident: format_ident!("{}", type_name),
                raw: type_name,
            });
        }

        ret
    };

    enum InputType {
        Enum(ItemEnum),
        Struct(ItemStruct),
    }

    impl ToTokens for InputType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                InputType::Enum(e) => e.to_tokens(tokens),
                InputType::Struct(s) => s.to_tokens(tokens),
            }
        }
    }

    let input_types = input_types
        .iter()
        .map(|t| {
            let mapping = item_name_mappings.get(t.name.as_str()).unwrap();
            let ident = &mapping.ident;

            let item = if t.name.ends_with("CreateInput")
                || t.name.ends_with("CreateManyInput")
                // Enums can't have required fields so it must be a struct!
                || t.fields.iter().any(|f| f.is_required)
            {
                let fields = t.fields.iter().map(|f| {
                    let name = snake_ident(&f.name);
                    let typ = item_name_mappings
                        .get(f.input_type().typ.as_str())
                        .map(|m| m.path())
                        .unwrap_or_else(|| {
                            let ident = format_ident!("{}", &f.input_type().typ);
                            parse_quote!(#ident)
                        });

                    quote!(#name: #typ)
                });

                InputType::Struct(parse_quote! {
                    pub struct #ident {
                        #(pub #fields),*
                    }
                })
            } else {
                let fields = t.fields.iter().fold(IndexMap::new(), |mut map, f| {
                    if !map.contains_key(&f.name) {
                        let name = pascal_ident(&f.name);
                        let typ = item_name_mappings
                            .get(f.input_type().typ.as_str())
                            .map(|m| {
                                let path = m.path();
                                quote!(super::#path)
                            })
                            .unwrap_or_else(|| {
                                let ident = format_ident!("{}", &f.input_type().typ);
                                quote!(#ident)
                            });

                        map.insert(&f.name, quote!(#name(#typ)));
                    }

                    map
                });
                let fields = fields.into_values();

                InputType::Enum(parse_quote! {
                    pub enum #ident {
                        #(#fields),*
                    }
                })
            };

            (&t.name, item)
        })
        .collect::<BTreeMap<_, _>>();

    let mut types_module = Module::new("_types", quote!());
    let mut root_module = Module::new("", quote!());

    for (name, typ) in input_types {
        let mapping = item_name_mappings.get(name.as_str()).unwrap();

        let mut module = &mut root_module;

        for parent in &mapping.parents {
            module = module.add_submodule(Module::new(parent, quote!()));
        }

        module.contents.extend(quote!(#typ));
    }

    let enums = schema.enum_types.get(PRISMA).unwrap().iter().map(|e| {
        let name = format_ident!("{}", e.name);

        let values = e.values.iter().map(|v| {
            let name = pascal_ident(v);

            quote!(#name)
        });

        quote! {
           pub enum #name {
               #(#values),*
           }
        }
    });

    root_module.add_submodule(Module::new(TYPES, quote!(#(#enums)*)));

    root_module.submodules.into_values().collect()
}
