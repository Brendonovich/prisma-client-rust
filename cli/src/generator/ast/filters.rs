use crate::generator::GraphQLType;

#[derive(Default)]
pub struct Method {
    pub name: String,
    pub action: String,
    pub is_list: bool,
    pub deprecated: String,
    pub typ: GraphQLType,
}

pub struct Filter {
    pub name: String,
    pub methods: Vec<Method>
}
