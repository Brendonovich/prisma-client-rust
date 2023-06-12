#[macro_export]
macro_rules! scalar_where_param_fns {
    (
        $filter_enum:ty,
        { $(fn $name:ident(_: $typ:ty $(,)?) -> $filter_variant:ident;)+ }
    ) => { $(
        pub fn $name(value: $typ) -> WhereInput {
            _where_identity(<$filter_enum>::$filter_variant(value))
        }
    )+}
}
