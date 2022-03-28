use crate::generator::GraphQLType;

#[derive(Default, Clone, Debug)]
pub struct Method {
    pub name: String,
    pub action: String,
    pub is_list: bool,
    pub deprecated: String,
    pub typ: GraphQLType,
}

#[derive(Debug)]
pub struct Filter {
    pub name: String,
    pub methods: Vec<Method>
}
