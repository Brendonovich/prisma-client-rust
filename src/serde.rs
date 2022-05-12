use std::fmt::Debug;

#[derive(Clone)]
pub struct FieldNotFetchedError;

impl Debug for FieldNotFetchedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Attempted to access a field that was not fetched using the .with() syntax"
        )
    }
}

pub type RelationResult<T> = Result<T, FieldNotFetchedError>;

pub fn default_field_not_fetched<T>() -> Result<T, FieldNotFetchedError> {
    Err(FieldNotFetchedError)
}

pub mod optional_single_relation {
    use super::FieldNotFetchedError;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> Result<Result<Option<T>, FieldNotFetchedError>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
            .map(Some)
            .map(|op| op.ok_or(FieldNotFetchedError))
    }

    pub fn serialize<S, T>(
        values: &Result<Option<T>, FieldNotFetchedError>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match values {
            Err(_) => serializer.serialize_unit(),
            Ok(None) => serializer.serialize_none(),
            Ok(Some(v)) => serializer.serialize_some(&v),
        }
    }
}
pub mod required_relation {
    use super::FieldNotFetchedError;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> Result<Result<T, FieldNotFetchedError>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
            .map(Some)
            .map(|op| op.ok_or(FieldNotFetchedError))
    }

    pub fn serialize<S, T>(
        values: &Result<T, FieldNotFetchedError>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match values {
            Err(_) => serializer.serialize_unit(),
            Ok(v) => v.serialize(serializer),
        }
    }
}
