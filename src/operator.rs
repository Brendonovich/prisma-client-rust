pub enum Operator<T> {
    Not(Vec<T>),
    And(Vec<T>),
    Or(Vec<T>),
}

pub fn not<T: From<Operator<T>>>(params: Vec<T>) -> T {
    Operator::Not(params).into()
}

pub fn and<T: From<Operator<T>>>(params: Vec<T>) -> T {
    Operator::And(params).into()
}

pub fn or<T: From<Operator<T>>>(params: Vec<T>) -> T {
    Operator::Or(params).into()
}
