use std::collections::VecDeque;

use query_core::Operation;

use crate::{PrismaClientInternals, Query};

pub enum BatchItemDataMeta {
    Query,
    Vec(usize, Box<Self>),
    Tuple(Vec<Self>),
}

pub enum BatchItemData {
    Query(Operation),
    Vec(Vec<Self>),
    Tuple(Vec<Self>),
}

impl BatchItemData {
    fn meta(&self) -> BatchItemDataMeta {
        match self {
            Self::Query(_) => BatchItemDataMeta::Query,
            Self::Vec(v) => BatchItemDataMeta::Vec(v.len(), Box::new(v[0].meta())),
            Self::Tuple(v) => BatchItemDataMeta::Tuple(v.iter().map(BatchItemData::meta).collect()),
        }
    }

    fn operations(self, v: &mut Vec<Operation>) {
        match self {
            Self::Query(op) => v.push(op),
            Self::Vec(items) => items.into_iter().for_each(|i| i.operations(v)),
            Self::Tuple(items) => items.into_iter().for_each(|i| i.operations(v)),
        }
    }
}

pub enum BatchDataMeta {
    Iterator(usize, BatchItemDataMeta),
    Tuple(Vec<BatchItemDataMeta>),
}

pub enum BatchData {
    Iterator(Vec<BatchItemData>),
    Tuple(Vec<BatchItemData>),
}

impl BatchData {
    fn meta(&self) -> BatchDataMeta {
        match self {
            Self::Iterator(v) => BatchDataMeta::Iterator(v.len(), v[0].meta()),
            Self::Tuple(v) => BatchDataMeta::Tuple(v.iter().map(BatchItemData::meta).collect()),
        }
    }

    fn operations(self) -> Vec<Operation> {
        let items = match self {
            Self::Tuple(items) => items,
            Self::Iterator(items) => items,
        };

        let mut ops = vec![];

        items.into_iter().for_each(|i| i.operations(&mut ops));

        ops
    }
}

pub async fn batch<'a, T: BatchContainer<'a, Marker>, Marker>(
    container: T,
    client: &'a PrismaClientInternals,
) -> super::Result<<T as BatchContainer<Marker>>::ReturnType> {
    let data = container.data();
    let meta = data.meta();

    let operations = data.operations();

    let values = client
        .engine
        .execute_all(operations)
        .await?
        .into_iter()
        .collect::<super::Result<VecDeque<_>>>()?;

    Ok(T::resolve(meta, values))
}

pub trait BatchItem<'a> {
    type ReturnValue;

    fn data(self) -> BatchItemData;

    fn resolve(
        meta: &BatchItemDataMeta,
        values: &mut VecDeque<serde_value::Value>,
    ) -> Self::ReturnValue;
}

impl<'a, Q: Query<'a>> BatchItem<'a> for Q {
    type ReturnValue = Q::ReturnValue;

    fn data(self) -> BatchItemData {
        BatchItemData::Query(self.graphql().0)
    }

    fn resolve(
        _: &BatchItemDataMeta,
        values: &mut VecDeque<serde_value::Value>,
    ) -> Self::ReturnValue {
        Q::convert(
            values
                .pop_front()
                .unwrap()
                .deserialize_into::<Q::RawType>()
                .unwrap(),
        )
    }
}

impl<'a, I: BatchItem<'a>> BatchItem<'a> for Vec<I> {
    type ReturnValue = Vec<I::ReturnValue>;

    fn data(self) -> BatchItemData {
        BatchItemData::Vec(self.into_iter().map(BatchItem::data).collect())
    }

    fn resolve(
        meta: &BatchItemDataMeta,
        values: &mut VecDeque<serde_value::Value>,
    ) -> Self::ReturnValue {
        match meta {
            BatchItemDataMeta::Vec(count, meta) => (0..*count)
                .map(|_| I::resolve(meta.as_ref(), values))
                .collect(),
            _ => unreachable!(),
        }
    }
}

/// A container that can hold queries to batch into a transaction
pub trait BatchContainer<'a, Marker> {
    type ReturnType;

    fn data(self) -> BatchData;

    fn resolve(meta: BatchDataMeta, values: VecDeque<serde_value::Value>) -> Self::ReturnType;
}

impl<'a, T: BatchItem<'a>, I: IntoIterator<Item = T>> BatchContainer<'a, ()> for I {
    type ReturnType = Vec<T::ReturnValue>;

    fn data(self) -> BatchData {
        BatchData::Iterator(self.into_iter().map(BatchItem::data).collect())
    }

    fn resolve(meta: BatchDataMeta, mut values: VecDeque<serde_value::Value>) -> Self::ReturnType {
        match meta {
            BatchDataMeta::Iterator(count, meta) => {
                (0..count).map(|_| T::resolve(&meta, &mut values)).collect()
            }
            _ => unreachable!(),
        }
    }
}

pub enum TupleMarker {}

macro_rules! impl_tuple {
    ($($generic:ident),+) => {
        #[allow(warnings)]
        impl<'a, $($generic: BatchItem<'a>),+> BatchContainer<'a, TupleMarker> for ($($generic),+) {
            type ReturnType = ($($generic::ReturnValue),+);

            fn data(self) -> BatchData {
                let ($($generic),+) = self;

                BatchData::Tuple(
                    vec![$(BatchItem::data($generic)),+]
                )
            }

            fn resolve(meta: BatchDataMeta, mut values: VecDeque<serde_value::Value>) -> Self::ReturnType {
                match meta {
                    BatchDataMeta::Tuple(metas) => {
                        let mut metas_iter = metas.iter();

                        (($(<$generic as BatchItem>::resolve(metas_iter.next().unwrap(), &mut values)),+))
                    },
                    _ => unreachable!()
                }
            }
        }

        #[allow(warnings)]
        impl<'a, $($generic: BatchItem<'a>),+> BatchItem<'a> for ($($generic),+) {
            type ReturnValue = ($($generic::ReturnValue),+);

            fn data(self) -> BatchItemData {
                let ($($generic),+) = self;

                BatchItemData::Tuple(
                    vec![$(BatchItem::data($generic)),+]
                )
            }

            fn resolve(
                meta: &BatchItemDataMeta,
                values: &mut VecDeque<serde_value::Value>,
            ) -> Self::ReturnValue {
                match meta {
                    BatchItemDataMeta::Tuple(meta) => {
                        let mut meta = meta.iter();

                        ($($generic::resolve(meta.next().unwrap(), values)),+)
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
}

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

/// TODO: remove this in 0.7.0
#[allow(warnings)]
impl<'a, Q: Query<'a>> BatchContainer<'a, TupleMarker> for Q {
    type ReturnType = Q::ReturnValue;

    fn data(self) -> BatchData {
        BatchData::Tuple(vec![BatchItem::data(self)])
    }

    fn resolve(meta: BatchDataMeta, mut values: VecDeque<serde_value::Value>) -> Self::ReturnType {
        match meta {
            BatchDataMeta::Tuple(metas) => {
                let mut metas_iter = metas.iter();

                <Q as BatchItem>::resolve(metas_iter.next().unwrap(), &mut values)
            }
            _ => unreachable!(),
        }
    }
}
