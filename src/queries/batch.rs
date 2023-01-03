use std::collections::VecDeque;

use query_core::Operation;
use serde::{
    de::{DeserializeOwned, IntoDeserializer},
    Deserialize,
};

use crate::{PrismaClientInternals, Query};

pub async fn batch<'a, T: BatchContainer<'a, Marker>, Marker>(
    container: T,
    client: &PrismaClientInternals,
) -> super::Result<<T as BatchContainer<'a, Marker>>::ReturnType> {
    let data = client
        .engine
        .execute_all(container.graphql())
        .await?
        .into_iter()
        .map(|result| Ok(T::RawType::deserialize(result?.into_deserializer())?))
        .collect::<super::Result<VecDeque<_>>>()?;

    Ok(T::convert(data))
}

/// A container that can hold queries to batch into a transaction
pub trait BatchContainer<'a, Marker> {
    type RawType: DeserializeOwned;
    type ReturnType;

    fn graphql(self) -> Vec<Operation>;
    fn convert(raw: VecDeque<Self::RawType>) -> Self::ReturnType;
}

impl<'a, T: Query<'a>, I: IntoIterator<Item = T>> BatchContainer<'a, ()> for I {
    type RawType = T::RawType;
    type ReturnType = Vec<T::ReturnType>;

    fn graphql(self) -> Vec<Operation> {
        self.into_iter()
            .map(Query::graphql)
            .map(|(o, _)| o)
            .collect()
    }

    fn convert(raw: VecDeque<Self::RawType>) -> Self::ReturnType {
        raw.into_iter().map(T::convert).collect()
    }
}

pub enum TupleMarker {}

macro_rules! impl_tuple {
    ($($generic:ident),+) => {
        #[allow(warnings)]
        impl<'a, $($generic: Query<'a>),+> BatchContainer<'a, TupleMarker> for ($($generic),+) {
            type RawType = serde_json::Value;
            type ReturnType = ($($generic::ReturnType),+);

            fn graphql(self) -> Vec<$crate::query_core::Operation> {
                let ($($generic),+) = self;

                vec![$($generic.graphql().0),+]
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
