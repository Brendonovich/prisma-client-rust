use crate::{ModelWriteOperation, SerializedWhereInput};

pub trait WhereInput {
    fn serialize(self) -> SerializedWhereInput;
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModelMutationCallbackData {
    pub action: ModelWriteOperation,
    pub model: &'static str,
}

pub type ModelMutationCallback = Box<dyn Fn(ModelMutationCallbackData) + Sync + Send>;

pub struct ActionNotifier {
    pub model_mutation_callbacks: Vec<ModelMutationCallback>,
}

impl ActionNotifier {
    pub fn new() -> Self {
        Self {
            model_mutation_callbacks: vec![],
        }
    }
}

impl Default for ActionNotifier {
    fn default() -> Self {
        Self::new()
    }
}
