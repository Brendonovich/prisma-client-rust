pub trait FromOptionalUniqueArg<Field> {
    type Arg;

    fn from_arg(arg: Self::Arg) -> Self
    where
        Self: Sized;
}
