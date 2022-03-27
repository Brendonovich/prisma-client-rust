use serde::{Deserialize, Serialize};

use self::{dmmf::Document, enums::Enum};

pub mod dmmf;
pub mod enums;
pub mod filters;
pub mod index;
pub mod models;
pub mod read_filters;
pub mod scalars;
pub mod write_filters;

pub use enums::*;
pub use filters::*;
pub use index::*;
pub use models::*;
pub use read_filters::*;
pub use scalars::*;
pub use write_filters::*;

pub struct AST<'a> {
    dmmf: &'a Document,

    pub scalars: Vec<String>,
    pub enums: Vec<Enum>,
    pub models: Vec<Model>,
    pub read_filters: Vec<Filter>,
    pub write_filters: Vec<Filter>,
    pub order_bys: Vec<OrderBy>,
}

impl<'a> AST<'a> {
    pub fn new(dmmf: &'a Document) -> Self {
        let mut ast = AST {
            dmmf,
            scalars: Vec::new(),
            enums: Vec::new(),
            models: Vec::new(),
            read_filters: Vec::new(),
            write_filters: Vec::new(),
            order_bys: Vec::new(),
        };

        ast.scalars = ast.scalars();
        ast.enums = ast.enums();
        
        ast.models = ast.models();
        
        ast.read_filters = ast.read_filters();
        ast.write_filters = ast.write_filters();
        
        for filter in ast.deprecated_read_filters() {
            for (i, f) in ast.read_filters.iter().enumerate() {
                if f.name == filter.name {
                    ast.read_filters[i].methods.extend(filter.methods);
                }
            }
        }
        
        ast
    }

    pub fn pick(&self, names: Vec<String>) -> Option<&dmmf::CoreType> {
        for name in names {
            for i in &self.dmmf.schema.input_object_types.prisma {
                if &i.name == &name {
                    return Some(&i);
                }
            }
        }
        
        None
    }
}
