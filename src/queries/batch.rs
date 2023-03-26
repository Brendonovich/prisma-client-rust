use std::{collections::VecDeque, num::NonZeroUsize};

use query_core::Operation;

use crate::{PrismaClientInternals, Query, QueryConvert};

pub enum VecMeta {
    Empty,
    NotEmpty(NonZeroUsize, Box<BatchItemDataMeta>),
}

pub enum BatchItemDataMeta {
    Query,
    Vec(VecMeta),
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
            Self::Vec(v) => BatchItemDataMeta::Vec({
                NonZeroUsize::new(v.len())
                    .map(|size| VecMeta::NotEmpty(size, Box::new(v[0].meta())))
                    .unwrap_or(VecMeta::Empty)
            }),
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
    Iterator(VecMeta),
    Tuple(Vec<BatchItemDataMeta>),
}

pub enum BatchData {
    Iterator(Vec<BatchItemData>),
    Tuple(Vec<BatchItemData>),
}

impl BatchData {
    fn meta(&self) -> BatchDataMeta {
        match self {
            Self::Iterator(v) => BatchDataMeta::Iterator(
                NonZeroUsize::new(v.len())
                    .map(|size| VecMeta::NotEmpty(size, Box::new(v[0].meta())))
                    .unwrap_or(VecMeta::Empty),
            ),
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

pub async fn batch<'batch, 'b, T: BatchContainer<'batch, Marker>, Marker>(
    container: T,
    client: &'b PrismaClientInternals,
) -> super::Result<<T as BatchContainer<'batch, Marker>>::ReturnType> {
    let data = container.data();
    let meta = data.meta();

    let operations = data.operations();

    let values = client
        .engine
        .execute_all(operations)
        .await?
        .into_iter()
        .collect::<super::Result<VecDeque<_>>>()?;

    T::resolve(meta, values)
}

pub trait BatchItemParent {
    type ReturnValue;
}

pub trait BatchItem<'a>: BatchItemParent {
    fn data(self) -> BatchItemData;

    fn resolve(
        meta: &BatchItemDataMeta,
        values: &mut VecDeque<serde_value::Value>,
    ) -> super::Result<<Self as BatchItemParent>::ReturnValue>;
}

impl<Q: QueryConvert> BatchItemParent for Q {
    type ReturnValue = Q::ReturnValue;
}

impl<'a, 'b, Q: Query<'a>> BatchItem<'b> for Q {
    fn data(self) -> BatchItemData {
        BatchItemData::Query(self.graphql().0)
    }

    fn resolve(
        _: &BatchItemDataMeta,
        values: &mut VecDeque<serde_value::Value>,
    ) -> super::Result<<Self as BatchItemParent>::ReturnValue> {
        Q::convert(
            values
                .pop_front()
                .unwrap()
                .deserialize_into::<Q::RawType>()
                .unwrap(),
        )
    }
}

impl<'batch, I: BatchItemParent> BatchItemParent for Vec<I> {
    type ReturnValue = Vec<<I as BatchItemParent>::ReturnValue>;
}

impl<'batch, 'query, I: Query<'query>> BatchItem<'batch> for Vec<I> {
    fn data(self) -> BatchItemData {
        BatchItemData::Vec(self.into_iter().map(BatchItem::data).collect())
    }

    fn resolve(
        meta: &BatchItemDataMeta,
        values: &mut VecDeque<serde_value::Value>,
    ) -> super::Result<<Self as BatchItemParent>::ReturnValue> {
        Ok(match meta {
            BatchItemDataMeta::Vec(meta) => match meta {
                VecMeta::Empty => vec![],
                VecMeta::NotEmpty(size, meta) => (0..size.get())
                    .map(|_| <I as BatchItem>::resolve(meta.as_ref(), values))
                    .collect::<super::Result<Vec<_>>>()?,
            },
            _ => unreachable!(),
        })
    }
}

/// A container that can hold queries to batch into a transaction
pub trait BatchContainer<'batch, Marker> {
    type ReturnType;

    fn data(self) -> BatchData;

    fn resolve(
        meta: BatchDataMeta,
        values: VecDeque<serde_value::Value>,
    ) -> super::Result<Self::ReturnType>;
}

impl<'batch, 't: 'batch, T: BatchItem<'t>, I: IntoIterator<Item = T>> BatchContainer<'batch, ()>
    for I
{
    type ReturnType = Vec<<T as BatchItemParent>::ReturnValue>;

    fn data(self) -> BatchData {
        BatchData::Iterator(self.into_iter().map(BatchItem::data).collect())
    }

    fn resolve(
        meta: BatchDataMeta,
        mut values: VecDeque<serde_value::Value>,
    ) -> super::Result<Self::ReturnType> {
        Ok(match meta {
            BatchDataMeta::Iterator(meta) => match meta {
                VecMeta::Empty => vec![],
                VecMeta::NotEmpty(size, meta) => (0..size.get())
                    .map(|_| T::resolve(&meta, &mut values))
                    .collect::<super::Result<Vec<_>>>()?,
            },
            _ => unreachable!(),
        })
    }
}

pub enum TupleMarker {}

macro_rules! impl_tuple {
    ($generic_1: ident, $($generic:ident),+) => {
        impl_tuple!(impl $generic_1 $(,$generic)+);
        impl_tuple!($($generic),+);
    };
    ($generic:ident) => {};
    (impl $generic:ident) => {};
    (impl $($generic:ident),+) => {
        paste::paste! {
            #[allow(warnings)]
            impl<'batch, $( [< "'" $generic >]: 'batch),+, $($generic: BatchItem<[< "'" $generic >]>),+> BatchContainer<'batch, TupleMarker> for ($($generic),+) {
                type ReturnType = ($(<$generic as BatchItemParent>::ReturnValue),+);

                fn data(self) -> BatchData {
                    let ($($generic),+) = self;

                    BatchData::Tuple(
                        vec![$(BatchItem::data($generic)),+]
                    )
                }

                fn resolve(meta: BatchDataMeta, mut values: VecDeque<serde_value::Value>) -> $crate::Result<Self::ReturnType> {
                    Ok(match meta {
                        BatchDataMeta::Tuple(metas) => {
                            let mut metas_iter = metas.iter();

                            ($(<$generic as BatchItem>::resolve(metas_iter.next().unwrap(), &mut values)?),+)
                        },
                        _ => unreachable!()
                    })
                }
            }

            #[allow(warnings)]
            impl<$($generic: BatchItemParent),+> BatchItemParent for ($($generic),+) {
                type ReturnValue = ($(<$generic as BatchItemParent>::ReturnValue),+);
            }

            #[allow(warnings)]
            impl<'batch, $( [< "'" $generic >]: 'batch),+, $($generic: BatchItem<[< "'" $generic >]>),+> BatchItem<'batch> for ($($generic),+) {
                fn data(self) -> BatchItemData {
                    let ($($generic),+) = self;

                    BatchItemData::Tuple(
                        vec![$(BatchItem::data($generic)),+]
                    )
                }

                fn resolve(
                    meta: &BatchItemDataMeta,
                    values: &mut VecDeque<serde_value::Value>,
                ) -> $crate::Result<<Self as BatchItemParent>::ReturnValue> {
                    Ok(match meta {
                        BatchItemDataMeta::Tuple(meta) => {
                            let mut meta = meta.iter();

                            ($($generic::resolve(meta.next().unwrap(), values)?),+)
                        },
                        _ => unreachable!(),
                    })
                }
            }
        }
    };
}

impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);

/// TODO: remove this in 0.7.0
#[allow(warnings)]
impl<'batch, 'query, Q: Query<'query>> BatchContainer<'batch, TupleMarker> for Q {
    type ReturnType = Q::ReturnValue;

    fn data(self) -> BatchData {
        BatchData::Tuple(vec![BatchItem::data(self)])
    }

    fn resolve(
        meta: BatchDataMeta,
        mut values: VecDeque<serde_value::Value>,
    ) -> super::Result<Self::ReturnType> {
        Ok(match meta {
            BatchDataMeta::Tuple(metas) => {
                let mut metas_iter = metas.iter();

                <Q as BatchItem>::resolve(metas_iter.next().unwrap(), &mut values)?
            }
            _ => unreachable!(),
        })
    }
}
