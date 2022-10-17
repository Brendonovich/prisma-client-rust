pub trait ExampleTrait {
    fn scalar_fields() -> Vec<&'static str>;
    fn relation_fields() -> Vec<&'static str>;
    fn id_fields() -> Vec<&'static str>;
}
