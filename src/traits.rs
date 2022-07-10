use query_core::Selection;
use serde::de::DeserializeOwned;

pub trait FromOptionalUniqueArg<Field> {
    type Arg;

    fn from_arg(arg: Self::Arg) -> Self
    where
        Self: Sized;
}

pub trait Select<T> {
    type Data: DeserializeOwned;

    fn to_selections(self) -> Vec<Selection>;
}
