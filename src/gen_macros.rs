#[macro_export]
macro_rules! scalar_where_param_fns {
    (
        $filter_enum:ty,
        $variant:ident,
        { $(fn $name:ident(_: $typ:ty $(,)?) -> $filter_variant:ident;)+ }
    ) => { $(
        pub fn $name(value: $typ) -> WhereParam {
            WhereParam::$variant(<$filter_enum>::$filter_variant(value))
        }
    )+}
}
