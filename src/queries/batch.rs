use std::collections::VecDeque;

use query_core::Operation;
use schema::QuerySchemaRef;
use serde::{
    de::{DeserializeOwned, IntoDeserializer},
    Deserialize,
};

use crate::{prisma_value, Executor, QueryError};

pub async fn batch<T: BatchContainer<Marker>, Marker>(
    container: T,
    executor: &Executor,
    query_schema: &QuerySchemaRef,
) -> super::Result<T::ReturnType> {
    let response = executor
        .execute_all(None, container.graphql(), true, query_schema.clone(), None)
        .await;

    let response = response.map_err(|e| QueryError::Execute(e.into()))?;

    let data = response
        .into_iter()
        .map(|result| {
            let data: prisma_value::Item = result
                .map_err(|e| QueryError::Execute(e.into()))?
                .data
                .into();

            let val = serde_value::to_value(data)?;

            Ok(<T::RawType as Deserialize>::deserialize(
                val.into_deserializer(),
            )?)
        })
        .collect::<super::Result<VecDeque<_>>>()?;

    Ok(T::convert(data))
}

/// A query that can be used within a transaction
pub trait BatchQuery {
    type RawType: DeserializeOwned;
    type ReturnType;

    fn graphql(self) -> Operation;

    /// Function for converting between raw database data and the type expected by the user.
    /// Necessary for things like raw queries
    fn convert(raw: Self::RawType) -> Self::ReturnType;
}

/// A container that can hold queries to batch into a transaction
pub trait BatchContainer<Marker> {
    type RawType: DeserializeOwned;
    type ReturnType;

    fn graphql(self) -> Vec<Operation>;
    fn convert(raw: VecDeque<Self::RawType>) -> Self::ReturnType;
}

impl<T: BatchQuery, I: IntoIterator<Item = T>> BatchContainer<()> for I {
    type RawType = T::RawType;
    type ReturnType = Vec<T::ReturnType>;

    fn graphql(self) -> Vec<Operation> {
        self.into_iter().map(BatchQuery::graphql).collect()
    }

    fn convert(raw: VecDeque<Self::RawType>) -> Self::ReturnType {
        raw.into_iter().map(T::convert).collect()
    }
}

pub enum TupleMarker {}

macro_rules! impl_tuple {
    ($($generic:ident),+) => {
        #[allow(warnings)]
        impl<$($generic: BatchQuery),+> BatchContainer<TupleMarker> for ($($generic),+) {
            type RawType = serde_json::Value;
            type ReturnType = ($($generic::ReturnType),+);

            fn graphql(self) -> Vec<$crate::query_core::Operation> {
                let ($($generic),+) = self;

                vec![$($generic.graphql()),+]
            }

            fn convert(mut raw: VecDeque<Self::RawType>) -> Self::ReturnType {
                ($($generic::convert(raw
                    .pop_front()
                    .map(|v| serde_json::from_value(v).unwrap())
                    .unwrap()
                )),+)
            }
        }
    }
}

impl_tuple!(T1);
impl_tuple!(T1, T2);
impl_tuple!(T1, T2, T3);
impl_tuple!(T1, T2, T3, T4);
impl_tuple!(T1, T2, T3, T4, T5);
impl_tuple!(T1, T2, T3, T4, T5, T6);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
