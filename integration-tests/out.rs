pub mod post {
    use super::*;
    pub mod id {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: String) -> T {
            UniqueWhereParam::IdEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Id(direction)
        }
        pub fn cursor(cursor: String) -> Cursor {
            Cursor::Id(cursor)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::IdInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::IdNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::IdLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::IdLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::IdGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::IdGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::IdContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::IdStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::IdEndsWith(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::IdNot(value)
        }
        pub struct Set(String);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetId(value.0)
            }
        }
    }
    pub mod created_at {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: chrono::DateTime<chrono::FixedOffset>) -> T {
            Set(value).into()
        }
        pub fn equals(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::CreatedAt(direction)
        }
        pub fn in_vec(value: Vec<chrono::DateTime<chrono::FixedOffset>>) -> WhereParam {
            WhereParam::CreatedAtInVec(value)
        }
        pub fn not_in_vec(value: Vec<chrono::DateTime<chrono::FixedOffset>>) -> WhereParam {
            WhereParam::CreatedAtNotInVec(value)
        }
        pub fn lt(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtLt(value)
        }
        pub fn lte(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtLte(value)
        }
        pub fn gt(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtGt(value)
        }
        pub fn gte(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtGte(value)
        }
        pub fn not(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtNot(value)
        }
        pub struct Set(chrono::DateTime<chrono::FixedOffset>);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetCreatedAt(value.0)
            }
        }
    }
    pub mod updated_at {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: chrono::DateTime<chrono::FixedOffset>) -> T {
            Set(value).into()
        }
        pub fn equals(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::UpdatedAtEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::UpdatedAt(direction)
        }
        pub fn in_vec(value: Vec<chrono::DateTime<chrono::FixedOffset>>) -> WhereParam {
            WhereParam::UpdatedAtInVec(value)
        }
        pub fn not_in_vec(value: Vec<chrono::DateTime<chrono::FixedOffset>>) -> WhereParam {
            WhereParam::UpdatedAtNotInVec(value)
        }
        pub fn lt(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::UpdatedAtLt(value)
        }
        pub fn lte(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::UpdatedAtLte(value)
        }
        pub fn gt(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::UpdatedAtGt(value)
        }
        pub fn gte(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::UpdatedAtGte(value)
        }
        pub fn not(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::UpdatedAtNot(value)
        }
        pub struct Set(chrono::DateTime<chrono::FixedOffset>);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetUpdatedAt(value.0)
            }
        }
    }
    pub mod title {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn equals(value: String) -> WhereParam {
            WhereParam::TitleEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Title(direction)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::TitleInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::TitleNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::TitleLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::TitleLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::TitleGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::TitleGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::TitleContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::TitleStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::TitleEndsWith(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::TitleNot(value)
        }
        pub struct Set(String);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetTitle(value.0)
            }
        }
    }
    pub mod published {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: bool) -> T {
            Set(value).into()
        }
        pub fn equals(value: bool) -> WhereParam {
            WhereParam::PublishedEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Published(direction)
        }
        pub struct Set(bool);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetPublished(value.0)
            }
        }
    }
    pub mod views {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: i32) -> T {
            Set(value).into()
        }
        pub fn equals(value: i32) -> WhereParam {
            WhereParam::ViewsEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Views(direction)
        }
        pub fn increment(value: i32) -> SetParam {
            SetParam::IncrementViews(value)
        }
        pub fn decrement(value: i32) -> SetParam {
            SetParam::DecrementViews(value)
        }
        pub fn multiply(value: i32) -> SetParam {
            SetParam::MultiplyViews(value)
        }
        pub fn divide(value: i32) -> SetParam {
            SetParam::DivideViews(value)
        }
        pub fn in_vec(value: Vec<i32>) -> WhereParam {
            WhereParam::ViewsInVec(value)
        }
        pub fn not_in_vec(value: Vec<i32>) -> WhereParam {
            WhereParam::ViewsNotInVec(value)
        }
        pub fn lt(value: i32) -> WhereParam {
            WhereParam::ViewsLt(value)
        }
        pub fn lte(value: i32) -> WhereParam {
            WhereParam::ViewsLte(value)
        }
        pub fn gt(value: i32) -> WhereParam {
            WhereParam::ViewsGt(value)
        }
        pub fn gte(value: i32) -> WhereParam {
            WhereParam::ViewsGte(value)
        }
        pub fn not(value: i32) -> WhereParam {
            WhereParam::ViewsNot(value)
        }
        pub struct Set(i32);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetViews(value.0)
            }
        }
    }
    pub mod desc {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: Option<String>) -> T {
            Set(value).into()
        }
        pub fn equals(value: Option<String>) -> WhereParam {
            WhereParam::DescEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Desc(direction)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::DescInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::DescNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::DescLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::DescLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::DescGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::DescGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::DescContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::DescStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::DescEndsWith(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::DescNot(value)
        }
        pub struct Set(Option<String>);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetDesc(value.0)
            }
        }
    }
    pub mod author {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn is(value: Vec<user::WhereParam>) -> WhereParam {
            WhereParam::AuthorIs(value)
        }
        pub fn is_not(value: Vec<user::WhereParam>) -> WhereParam {
            WhereParam::AuthorIsNot(value)
        }
        pub struct Fetch {
            args: user::Args,
        }
        impl Fetch {
            pub fn with(mut self, params: impl Into<user::WithParam>) -> Self {
                self.args = self.args.with(params.into());
                self
            }
        }
        impl From<Fetch> for WithParam {
            fn from(fetch: Fetch) -> Self {
                WithParam::Author(fetch.args)
            }
        }
        pub fn fetch() -> Fetch {
            Fetch {
                args: user::Args::new(),
            }
        }
        pub fn link<T: From<Link>>(value: user::UniqueWhereParam) -> T {
            Link(value).into()
        }
        pub fn unlink() -> SetParam {
            SetParam::UnlinkAuthor
        }
        pub struct Link(user::UniqueWhereParam);
        impl From<Link> for SetParam {
            fn from(value: Link) -> Self {
                Self::LinkAuthor(value.0)
            }
        }
    }
    pub mod author_id {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: Option<String>) -> T {
            Set(value).into()
        }
        pub fn equals(value: Option<String>) -> WhereParam {
            WhereParam::AuthorIdEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::AuthorId(direction)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::AuthorIdInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::AuthorIdNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::AuthorIdLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::AuthorIdLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::AuthorIdGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::AuthorIdGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::AuthorIdContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::AuthorIdStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::AuthorIdEndsWith(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::AuthorIdNot(value)
        }
        pub struct Set(Option<String>);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetAuthorId(value.0)
            }
        }
    }
    pub mod categories {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn some(value: Vec<category::WhereParam>) -> WhereParam {
            WhereParam::CategoriesSome(value)
        }
        pub fn every(value: Vec<category::WhereParam>) -> WhereParam {
            WhereParam::CategoriesEvery(value)
        }
        pub fn none(value: Vec<category::WhereParam>) -> WhereParam {
            WhereParam::CategoriesNone(value)
        }
        pub struct Fetch {
            args: category::FindManyArgs,
        }
        impl Fetch {
            pub fn with(mut self, params: impl Into<category::WithParam>) -> Self {
                self.args = self.args.with(params.into());
                self
            }
            pub fn order_by(mut self, param: category::OrderByParam) -> Self {
                self.args = self.args.order_by(param);
                self
            }
            pub fn skip(mut self, value: i64) -> Self {
                self.args = self.args.skip(value);
                self
            }
            pub fn take(mut self, value: i64) -> Self {
                self.args = self.args.take(value);
                self
            }
            pub fn cursor(mut self, value: impl Into<category::Cursor>) -> Self {
                self.args = self.args.cursor(value.into());
                self
            }
        }
        impl From<Fetch> for WithParam {
            fn from(fetch: Fetch) -> Self {
                WithParam::Categories(fetch.args)
            }
        }
        pub fn fetch(params: Vec<category::WhereParam>) -> Fetch {
            Fetch {
                args: category::FindManyArgs::new(params),
            }
        }
        pub fn link<T: From<Link>>(params: Vec<category::UniqueWhereParam>) -> T {
            Link(params).into()
        }
        pub fn unlink(params: Vec<category::UniqueWhereParam>) -> SetParam {
            SetParam::UnlinkCategories(params)
        }
        pub struct Link(Vec<category::UniqueWhereParam>);
        impl From<Link> for SetParam {
            fn from(value: Link) -> Self {
                Self::LinkCategories(value.0)
            }
        }
    }
    pub mod favouriters {
        use super::super::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn some(value: Vec<user::WhereParam>) -> WhereParam {
            WhereParam::FavouritersSome(value)
        }
        pub fn every(value: Vec<user::WhereParam>) -> WhereParam {
            WhereParam::FavouritersEvery(value)
        }
        pub fn none(value: Vec<user::WhereParam>) -> WhereParam {
            WhereParam::FavouritersNone(value)
        }
        pub struct Fetch {
            args: user::FindManyArgs,
        }
        impl Fetch {
            pub fn with(mut self, params: impl Into<user::WithParam>) -> Self {
                self.args = self.args.with(params.into());
                self
            }
            pub fn order_by(mut self, param: user::OrderByParam) -> Self {
                self.args = self.args.order_by(param);
                self
            }
            pub fn skip(mut self, value: i64) -> Self {
                self.args = self.args.skip(value);
                self
            }
            pub fn take(mut self, value: i64) -> Self {
                self.args = self.args.take(value);
                self
            }
            pub fn cursor(mut self, value: impl Into<user::Cursor>) -> Self {
                self.args = self.args.cursor(value.into());
                self
            }
        }
        impl From<Fetch> for WithParam {
            fn from(fetch: Fetch) -> Self {
                WithParam::Favouriters(fetch.args)
            }
        }
        pub fn fetch(params: Vec<user::WhereParam>) -> Fetch {
            Fetch {
                args: user::FindManyArgs::new(params),
            }
        }
        pub fn link<T: From<Link>>(params: Vec<user::UniqueWhereParam>) -> T {
            Link(params).into()
        }
        pub fn unlink(params: Vec<user::UniqueWhereParam>) -> SetParam {
            SetParam::UnlinkFavouriters(params)
        }
        pub struct Link(Vec<user::UniqueWhereParam>);
        impl From<Link> for SetParam {
            fn from(value: Link) -> Self {
                Self::LinkFavouriters(value.0)
            }
        }
    }
    pub fn title_author_id<T: From<UniqueWhereParam>>(title: String, author_id: String) -> T {
        UniqueWhereParam::TitleAuthorIdEquals(title, author_id).into()
    }
    pub fn _outputs() -> Vec<Selection> {
        [
            "id",
            "created_at",
            "updated_at",
            "title",
            "published",
            "views",
            "desc",
            "author_id",
        ]
        .into_iter()
        .map(|o| {
            let builder = Selection::builder(o);
            builder.build()
        })
        .collect()
    }
    pub struct Data {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "created_at")]
        pub created_at: chrono::DateTime<chrono::FixedOffset>,
        #[serde(rename = "updated_at")]
        pub updated_at: chrono::DateTime<chrono::FixedOffset>,
        #[serde(rename = "title")]
        pub title: String,
        #[serde(rename = "published")]
        pub published: bool,
        #[serde(rename = "views")]
        pub views: i32,
        #[serde(rename = "desc")]
        pub desc: Option<String>,
        #[serde(
            rename = "author",
            skip_serializing_if = "Result::is_err",
            with = "prisma_client_rust::serde::optional_single_relation"
        )]
        pub author: Result<Option<Box<super::user::Data>>, String>,
        #[serde(rename = "author_id")]
        pub author_id: Option<String>,
        #[serde(
            rename = "categories",
            skip_serializing_if = "Result::is_err",
            with = "prisma_client_rust::serde::required_relation"
        )]
        pub categories: Result<Vec<super::category::Data>, String>,
        #[serde(
            rename = "favouriters",
            skip_serializing_if = "Result::is_err",
            with = "prisma_client_rust::serde::required_relation"
        )]
        pub favouriters: Result<Vec<super::user::Data>, String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Data {
                    id: ref __self_0_0,
                    created_at: ref __self_0_1,
                    updated_at: ref __self_0_2,
                    title: ref __self_0_3,
                    published: ref __self_0_4,
                    views: ref __self_0_5,
                    desc: ref __self_0_6,
                    author: ref __self_0_7,
                    author_id: ref __self_0_8,
                    categories: ref __self_0_9,
                    favouriters: ref __self_0_10,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Data");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "created_at",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "updated_at",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "title",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "published",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "views",
                        &&(*__self_0_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "desc",
                        &&(*__self_0_6),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "author",
                        &&(*__self_0_7),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "author_id",
                        &&(*__self_0_8),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "categories",
                        &&(*__self_0_9),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "favouriters",
                        &&(*__self_0_10),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Data {
        #[inline]
        fn clone(&self) -> Data {
            match *self {
                Data {
                    id: ref __self_0_0,
                    created_at: ref __self_0_1,
                    updated_at: ref __self_0_2,
                    title: ref __self_0_3,
                    published: ref __self_0_4,
                    views: ref __self_0_5,
                    desc: ref __self_0_6,
                    author: ref __self_0_7,
                    author_id: ref __self_0_8,
                    categories: ref __self_0_9,
                    favouriters: ref __self_0_10,
                } => Data {
                    id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    created_at: ::core::clone::Clone::clone(&(*__self_0_1)),
                    updated_at: ::core::clone::Clone::clone(&(*__self_0_2)),
                    title: ::core::clone::Clone::clone(&(*__self_0_3)),
                    published: ::core::clone::Clone::clone(&(*__self_0_4)),
                    views: ::core::clone::Clone::clone(&(*__self_0_5)),
                    desc: ::core::clone::Clone::clone(&(*__self_0_6)),
                    author: ::core::clone::Clone::clone(&(*__self_0_7)),
                    author_id: ::core::clone::Clone::clone(&(*__self_0_8)),
                    categories: ::core::clone::Clone::clone(&(*__self_0_9)),
                    favouriters: ::core::clone::Clone::clone(&(*__self_0_10)),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Data {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Data",
                    false as usize
                        + 1
                        + 1
                        + 1
                        + 1
                        + 1
                        + 1
                        + 1
                        + if Result::is_err(&self.author) { 0 } else { 1 }
                        + 1
                        + if Result::is_err(&self.categories) {
                            0
                        } else {
                            1
                        }
                        + if Result::is_err(&self.favouriters) {
                            0
                        } else {
                            1
                        },
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "created_at",
                    &self.created_at,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updated_at",
                    &self.updated_at,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "published",
                    &self.published,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "views",
                    &self.views,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "desc",
                    &self.desc,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                if !Result::is_err(&self.author) {
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "author",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a Result<Option<Box<super::user::Data>>, String>,),
                                phantom: _serde::__private::PhantomData<Data>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    prisma_client_rust::serde::optional_single_relation::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.author,),
                                phantom: _serde::__private::PhantomData::<Data>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                } else {
                    match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "author") {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                }
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "author_id",
                    &self.author_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                if !Result::is_err(&self.categories) {
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "categories",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a Result<Vec<super::category::Data>, String>,),
                                phantom: _serde::__private::PhantomData<Data>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    prisma_client_rust::serde::required_relation::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.categories,),
                                phantom: _serde::__private::PhantomData::<Data>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                } else {
                    match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "categories")
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                }
                if !Result::is_err(&self.favouriters) {
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "favouriters",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a Result<Vec<super::user::Data>, String>,),
                                phantom: _serde::__private::PhantomData<Data>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    prisma_client_rust::serde::required_relation::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.favouriters,),
                                phantom: _serde::__private::PhantomData::<Data>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                } else {
                    match _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "favouriters",
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                }
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Data {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "created_at" => _serde::__private::Ok(__Field::__field1),
                            "updated_at" => _serde::__private::Ok(__Field::__field2),
                            "title" => _serde::__private::Ok(__Field::__field3),
                            "published" => _serde::__private::Ok(__Field::__field4),
                            "views" => _serde::__private::Ok(__Field::__field5),
                            "desc" => _serde::__private::Ok(__Field::__field6),
                            "author" => _serde::__private::Ok(__Field::__field7),
                            "author_id" => _serde::__private::Ok(__Field::__field8),
                            "categories" => _serde::__private::Ok(__Field::__field9),
                            "favouriters" => _serde::__private::Ok(__Field::__field10),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"created_at" => _serde::__private::Ok(__Field::__field1),
                            b"updated_at" => _serde::__private::Ok(__Field::__field2),
                            b"title" => _serde::__private::Ok(__Field::__field3),
                            b"published" => _serde::__private::Ok(__Field::__field4),
                            b"views" => _serde::__private::Ok(__Field::__field5),
                            b"desc" => _serde::__private::Ok(__Field::__field6),
                            b"author" => _serde::__private::Ok(__Field::__field7),
                            b"author_id" => _serde::__private::Ok(__Field::__field8),
                            b"categories" => _serde::__private::Ok(__Field::__field9),
                            b"favouriters" => _serde::__private::Ok(__Field::__field10),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Data>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Data;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Data")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Data with 11 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            chrono::DateTime<chrono::FixedOffset>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Data with 11 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            chrono::DateTime<chrono::FixedOffset>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Data with 11 elements",
                                ));
                            }
                        };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Data with 11 elements",
                                        ),
                                    );
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Data with 11 elements",
                                        ),
                                    );
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Data with 11 elements",
                                        ),
                                    );
                                }
                            };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct Data with 11 elements",
                                ));
                            }
                        };
                        let __field7 = match {
                            struct __DeserializeWith<'de> {
                                value: Result<Option<Box<super::user::Data>>, String>,
                                phantom: _serde::__private::PhantomData<Data>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde :: __private :: Ok (__DeserializeWith { value : match prisma_client_rust :: serde :: optional_single_relation :: deserialize (__deserializer) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } , phantom : _serde :: __private :: PhantomData , lifetime : _serde :: __private :: PhantomData , })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct Data with 11 elements",
                                ));
                            }
                        };
                        let __field8 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    8usize,
                                    &"struct Data with 11 elements",
                                ));
                            }
                        };
                        let __field9 = match {
                            struct __DeserializeWith<'de> {
                                value: Result<Vec<super::category::Data>, String>,
                                phantom: _serde::__private::PhantomData<Data>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde :: __private :: Ok (__DeserializeWith { value : match prisma_client_rust :: serde :: required_relation :: deserialize (__deserializer) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } , phantom : _serde :: __private :: PhantomData , lifetime : _serde :: __private :: PhantomData , })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    9usize,
                                    &"struct Data with 11 elements",
                                ));
                            }
                        };
                        let __field10 = match {
                            struct __DeserializeWith<'de> {
                                value: Result<Vec<super::user::Data>, String>,
                                phantom: _serde::__private::PhantomData<Data>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde :: __private :: Ok (__DeserializeWith { value : match prisma_client_rust :: serde :: required_relation :: deserialize (__deserializer) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } , phantom : _serde :: __private :: PhantomData , lifetime : _serde :: __private :: PhantomData , })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    10usize,
                                    &"struct Data with 11 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(Data {
                            id: __field0,
                            created_at: __field1,
                            updated_at: __field2,
                            title: __field3,
                            published: __field4,
                            views: __field5,
                            desc: __field6,
                            author: __field7,
                            author_id: __field8,
                            categories: __field9,
                            favouriters: __field10,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            chrono::DateTime<chrono::FixedOffset>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            chrono::DateTime<chrono::FixedOffset>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field7: _serde::__private::Option<
                            Result<Option<Box<super::user::Data>>, String>,
                        > = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field9: _serde::__private::Option<
                            Result<Vec<super::category::Data>, String>,
                        > = _serde::__private::None;
                        let mut __field10: _serde::__private::Option<
                            Result<Vec<super::user::Data>, String>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "created_at",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            chrono::DateTime<chrono::FixedOffset>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updated_at",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            chrono::DateTime<chrono::FixedOffset>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "title",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "published",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "views",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "desc",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "author",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Result<Option<Box<super::user::Data>>, String>,
                                            phantom: _serde::__private::PhantomData<Data>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: __private :: Ok (__DeserializeWith { value : match prisma_client_rust :: serde :: optional_single_relation :: deserialize (__deserializer) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } , phantom : _serde :: __private :: PhantomData , lifetime : _serde :: __private :: PhantomData , })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "author_id",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "categories",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Result<Vec<super::category::Data>, String>,
                                            phantom: _serde::__private::PhantomData<Data>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: __private :: Ok (__DeserializeWith { value : match prisma_client_rust :: serde :: required_relation :: deserialize (__deserializer) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } , phantom : _serde :: __private :: PhantomData , lifetime : _serde :: __private :: PhantomData , })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field10 => {
                                    if _serde::__private::Option::is_some(&__field10) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "favouriters",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Result<Vec<super::user::Data>, String>,
                                            phantom: _serde::__private::PhantomData<Data>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: __private :: Ok (__DeserializeWith { value : match prisma_client_rust :: serde :: required_relation :: deserialize (__deserializer) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } , phantom : _serde :: __private :: PhantomData , lifetime : _serde :: __private :: PhantomData , })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("created_at") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("updated_at") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("title") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("published") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("views") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("desc") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("author"),
                                )
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("author_id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("categories"),
                                )
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::__private::Some(__field10) => __field10,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("favouriters"),
                                )
                            }
                        };
                        _serde::__private::Ok(Data {
                            id: __field0,
                            created_at: __field1,
                            updated_at: __field2,
                            title: __field3,
                            published: __field4,
                            views: __field5,
                            desc: __field6,
                            author: __field7,
                            author_id: __field8,
                            categories: __field9,
                            favouriters: __field10,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "created_at",
                    "updated_at",
                    "title",
                    "published",
                    "views",
                    "desc",
                    "author",
                    "author_id",
                    "categories",
                    "favouriters",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Data",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Data>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub enum WithParam {
        Author(super::user::Args),
        Categories(super::category::FindManyArgs),
        Favouriters(super::user::FindManyArgs),
    }
    impl Into<Selection> for WithParam {
        fn into(self) -> Selection {
            match self {
                Self::Author(args) => {
                    let mut selections = super::user::_outputs();
                    selections.extend(args.with_params.into_iter().map(Into::<Selection>::into));
                    let mut builder = Selection::builder("author");
                    builder.nested_selections(selections);
                    builder.build()
                }
                Self::Categories(args) => {
                    let FindManySelectionArgs {
                        mut nested_selections,
                        arguments,
                    } = args.into();
                    nested_selections.extend(super::category::_outputs());
                    let mut builder = Selection::builder("categories");
                    builder
                        .nested_selections(nested_selections)
                        .set_arguments(arguments);
                    builder.build()
                }
                Self::Favouriters(args) => {
                    let FindManySelectionArgs {
                        mut nested_selections,
                        arguments,
                    } = args.into();
                    nested_selections.extend(super::user::_outputs());
                    let mut builder = Selection::builder("favouriters");
                    builder
                        .nested_selections(nested_selections)
                        .set_arguments(arguments);
                    builder.build()
                }
            }
        }
    }
    pub enum SetParam {
        SetId(String),
        SetCreatedAt(chrono::DateTime<chrono::FixedOffset>),
        SetUpdatedAt(chrono::DateTime<chrono::FixedOffset>),
        SetTitle(String),
        SetPublished(bool),
        SetViews(i32),
        IncrementViews(i32),
        DecrementViews(i32),
        MultiplyViews(i32),
        DivideViews(i32),
        SetDesc(Option<String>),
        LinkAuthor(super::user::UniqueWhereParam),
        UnlinkAuthor,
        SetAuthorId(Option<String>),
        LinkCategories(Vec<super::category::UniqueWhereParam>),
        UnlinkCategories(Vec<super::category::UniqueWhereParam>),
        LinkFavouriters(Vec<super::user::UniqueWhereParam>),
        UnlinkFavouriters(Vec<super::user::UniqueWhereParam>),
    }
    impl Into<(String, QueryValue)> for SetParam {
        fn into(self) -> (String, QueryValue) {
            match self {
                SetParam::SetId(value) => ("id".to_string(), PrismaValue::String(value).into()),
                SetParam::SetCreatedAt(value) => (
                    "created_at".to_string(),
                    PrismaValue::DateTime(value).into(),
                ),
                SetParam::SetUpdatedAt(value) => (
                    "updated_at".to_string(),
                    PrismaValue::DateTime(value).into(),
                ),
                SetParam::SetTitle(value) => {
                    ("title".to_string(), PrismaValue::String(value).into())
                }
                SetParam::SetPublished(value) => {
                    ("published".to_string(), PrismaValue::Boolean(value).into())
                }
                SetParam::SetViews(value) => {
                    ("views".to_string(), PrismaValue::Int(value as i64).into())
                }
                SetParam::IncrementViews(value) => (
                    "views".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "increment".to_string(),
                            PrismaValue::Int(value as i64).into(),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::DecrementViews(value) => (
                    "views".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "decrement".to_string(),
                            PrismaValue::Int(value as i64).into(),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::MultiplyViews(value) => (
                    "views".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "multiply".to_string(),
                            PrismaValue::Int(value as i64).into(),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::DivideViews(value) => (
                    "views".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "divide".to_string(),
                            PrismaValue::Int(value as i64).into(),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::SetDesc(value) => (
                    "desc".to_string(),
                    value
                        .map(|value| PrismaValue::String(value).into())
                        .unwrap_or(QueryValue::Null),
                ),
                SetParam::LinkAuthor(where_param) => (
                    "author".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "connect".to_string(),
                            QueryValue::Object(
                                transform_equals(
                                    <[_]>::into_vec(box [where_param])
                                        .into_iter()
                                        .map(Into::<super::user::WhereParam>::into)
                                        .map(Into::into),
                                )
                                .into_iter()
                                .collect(),
                            ),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::UnlinkAuthor => (
                    "author".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "disconnect".to_string(),
                            QueryValue::Boolean(true),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::SetAuthorId(value) => (
                    "author_id".to_string(),
                    value
                        .map(|value| PrismaValue::String(value).into())
                        .unwrap_or(QueryValue::Null),
                ),
                SetParam::LinkCategories(where_params) => (
                    "categories".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "connect".to_string(),
                            QueryValue::Object(
                                transform_equals(
                                    where_params
                                        .into_iter()
                                        .map(Into::<super::category::WhereParam>::into)
                                        .map(Into::into),
                                )
                                .into_iter()
                                .collect(),
                            ),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::UnlinkCategories(where_params) => (
                    "categories".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "disconnect".to_string(),
                            QueryValue::Object(
                                transform_equals(
                                    where_params
                                        .into_iter()
                                        .map(Into::<super::category::WhereParam>::into)
                                        .map(Into::into),
                                )
                                .into_iter()
                                .collect(),
                            ),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::LinkFavouriters(where_params) => (
                    "favouriters".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "connect".to_string(),
                            QueryValue::Object(
                                transform_equals(
                                    where_params
                                        .into_iter()
                                        .map(Into::<super::user::WhereParam>::into)
                                        .map(Into::into),
                                )
                                .into_iter()
                                .collect(),
                            ),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
                SetParam::UnlinkFavouriters(where_params) => (
                    "favouriters".to_string(),
                    QueryValue::Object(
                        <[_]>::into_vec(box [(
                            "disconnect".to_string(),
                            QueryValue::Object(
                                transform_equals(
                                    where_params
                                        .into_iter()
                                        .map(Into::<super::user::WhereParam>::into)
                                        .map(Into::into),
                                )
                                .into_iter()
                                .collect(),
                            ),
                        )])
                        .into_iter()
                        .collect(),
                    ),
                ),
            }
        }
    }
    pub enum OrderByParam {
        Id(Direction),
        CreatedAt(Direction),
        UpdatedAt(Direction),
        Title(Direction),
        Published(Direction),
        Views(Direction),
        Desc(Direction),
        AuthorId(Direction),
    }
    impl Into<(String, QueryValue)> for OrderByParam {
        fn into(self) -> (String, QueryValue) {
            match self {
                Self::Id(direction) => {
                    ("id".to_string(), QueryValue::String(direction.to_string()))
                }
                Self::CreatedAt(direction) => (
                    "created_at".to_string(),
                    QueryValue::String(direction.to_string()),
                ),
                Self::UpdatedAt(direction) => (
                    "updated_at".to_string(),
                    QueryValue::String(direction.to_string()),
                ),
                Self::Title(direction) => (
                    "title".to_string(),
                    QueryValue::String(direction.to_string()),
                ),
                Self::Published(direction) => (
                    "published".to_string(),
                    QueryValue::String(direction.to_string()),
                ),
                Self::Views(direction) => (
                    "views".to_string(),
                    QueryValue::String(direction.to_string()),
                ),
                Self::Desc(direction) => (
                    "desc".to_string(),
                    QueryValue::String(direction.to_string()),
                ),
                Self::AuthorId(direction) => (
                    "author_id".to_string(),
                    QueryValue::String(direction.to_string()),
                ),
            }
        }
    }
    pub enum Cursor {
        Id(String),
        CreatedAt(chrono::DateTime<chrono::FixedOffset>),
        UpdatedAt(chrono::DateTime<chrono::FixedOffset>),
        Title(String),
        Published(bool),
        Views(i32),
        Desc(String),
        AuthorId(String),
    }
    impl Into<(String, QueryValue)> for Cursor {
        fn into(self) -> (String, QueryValue) {
            match self {
                Self::Id(cursor) => ("id".to_string(), PrismaValue::String(cursor).into()),
                Self::CreatedAt(cursor) => (
                    "created_at".to_string(),
                    PrismaValue::DateTime(cursor).into(),
                ),
                Self::UpdatedAt(cursor) => (
                    "updated_at".to_string(),
                    PrismaValue::DateTime(cursor).into(),
                ),
                Self::Title(cursor) => ("title".to_string(), PrismaValue::String(cursor).into()),
                Self::Published(cursor) => {
                    ("published".to_string(), PrismaValue::Boolean(cursor).into())
                }
                Self::Views(cursor) => {
                    ("views".to_string(), PrismaValue::Int(cursor as i64).into())
                }
                Self::Desc(cursor) => ("desc".to_string(), PrismaValue::String(cursor).into()),
                Self::AuthorId(cursor) => {
                    ("author_id".to_string(), PrismaValue::String(cursor).into())
                }
            }
        }
    }
    pub enum WhereParam {
        Not(Vec<WhereParam>),
        Or(Vec<WhereParam>),
        And(Vec<WhereParam>),
        TitleAuthorIdEquals(String, String),
        IdEquals(String),
        IdInVec(Vec<String>),
        IdNotInVec(Vec<String>),
        IdLt(String),
        IdLte(String),
        IdGt(String),
        IdGte(String),
        IdContains(String),
        IdStartsWith(String),
        IdEndsWith(String),
        IdNot(String),
        CreatedAtEquals(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtInVec(Vec<chrono::DateTime<chrono::FixedOffset>>),
        CreatedAtNotInVec(Vec<chrono::DateTime<chrono::FixedOffset>>),
        CreatedAtLt(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtLte(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtGt(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtGte(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtNot(chrono::DateTime<chrono::FixedOffset>),
        UpdatedAtEquals(chrono::DateTime<chrono::FixedOffset>),
        UpdatedAtInVec(Vec<chrono::DateTime<chrono::FixedOffset>>),
        UpdatedAtNotInVec(Vec<chrono::DateTime<chrono::FixedOffset>>),
        UpdatedAtLt(chrono::DateTime<chrono::FixedOffset>),
        UpdatedAtLte(chrono::DateTime<chrono::FixedOffset>),
        UpdatedAtGt(chrono::DateTime<chrono::FixedOffset>),
        UpdatedAtGte(chrono::DateTime<chrono::FixedOffset>),
        UpdatedAtNot(chrono::DateTime<chrono::FixedOffset>),
        TitleEquals(String),
        TitleInVec(Vec<String>),
        TitleNotInVec(Vec<String>),
        TitleLt(String),
        TitleLte(String),
        TitleGt(String),
        TitleGte(String),
        TitleContains(String),
        TitleStartsWith(String),
        TitleEndsWith(String),
        TitleNot(String),
        PublishedEquals(bool),
        ViewsEquals(i32),
        ViewsInVec(Vec<i32>),
        ViewsNotInVec(Vec<i32>),
        ViewsLt(i32),
        ViewsLte(i32),
        ViewsGt(i32),
        ViewsGte(i32),
        ViewsNot(i32),
        DescEquals(Option<String>),
        DescInVec(Vec<String>),
        DescNotInVec(Vec<String>),
        DescLt(String),
        DescLte(String),
        DescGt(String),
        DescGte(String),
        DescContains(String),
        DescStartsWith(String),
        DescEndsWith(String),
        DescNot(String),
        AuthorIs(Vec<super::user::WhereParam>),
        AuthorIsNot(Vec<super::user::WhereParam>),
        AuthorIdEquals(Option<String>),
        AuthorIdInVec(Vec<String>),
        AuthorIdNotInVec(Vec<String>),
        AuthorIdLt(String),
        AuthorIdLte(String),
        AuthorIdGt(String),
        AuthorIdGte(String),
        AuthorIdContains(String),
        AuthorIdStartsWith(String),
        AuthorIdEndsWith(String),
        AuthorIdNot(String),
        CategoriesSome(Vec<super::category::WhereParam>),
        CategoriesEvery(Vec<super::category::WhereParam>),
        CategoriesNone(Vec<super::category::WhereParam>),
        FavouritersSome(Vec<super::user::WhereParam>),
        FavouritersEvery(Vec<super::user::WhereParam>),
        FavouritersNone(Vec<super::user::WhereParam>),
    }
    impl Into<SerializedWhere> for WhereParam {
        fn into(self) -> SerializedWhere {
            match self {
                Self::Not(value) => (
                    "NOT".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| {
                                QueryValue::Object(
                                    transform_equals(
                                        <[_]>::into_vec(box [Into::<SerializedWhere>::into(v)])
                                            .into_iter(),
                                    )
                                    .into_iter()
                                    .collect(),
                                )
                            })
                            .collect(),
                    ),
                ),
                Self::Or(value) => (
                    "OR".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| {
                                QueryValue::Object(
                                    transform_equals(
                                        <[_]>::into_vec(box [Into::<SerializedWhere>::into(v)])
                                            .into_iter(),
                                    )
                                    .into_iter()
                                    .collect(),
                                )
                            })
                            .collect(),
                    ),
                ),
                Self::And(value) => (
                    "AND".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| {
                                QueryValue::Object(
                                    transform_equals(
                                        <[_]>::into_vec(box [Into::<SerializedWhere>::into(v)])
                                            .into_iter(),
                                    )
                                    .into_iter()
                                    .collect(),
                                )
                            })
                            .collect(),
                    ),
                ),
                Self::TitleAuthorIdEquals(title, author_id) => (
                    "title_author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [
                        ("title".to_string(), PrismaValue::String(title).into()),
                        (
                            "author_id".to_string(),
                            PrismaValue::String(author_id).into(),
                        ),
                    ])),
                ),
                Self::IdEquals(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdInVec(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "in".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::IdNotInVec(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "notIn".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::IdLt(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdLte(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdGt(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdGte(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdContains(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "contains".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdStartsWith(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "startsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdEndsWith(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "endsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::IdNot(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "not".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::CreatedAtEquals(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::CreatedAtInVec(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "in".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::DateTime(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::CreatedAtNotInVec(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "notIn".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::DateTime(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::CreatedAtLt(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lt".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::CreatedAtLte(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lte".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::CreatedAtGt(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gt".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::CreatedAtGte(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gte".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::CreatedAtNot(value) => (
                    "created_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "not".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::UpdatedAtEquals(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::UpdatedAtInVec(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "in".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::DateTime(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::UpdatedAtNotInVec(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "notIn".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::DateTime(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::UpdatedAtLt(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lt".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::UpdatedAtLte(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lte".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::UpdatedAtGt(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gt".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::UpdatedAtGte(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gte".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::UpdatedAtNot(value) => (
                    "updated_at".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "not".to_string(),
                        PrismaValue::DateTime(value).into(),
                    )])),
                ),
                Self::TitleEquals(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleInVec(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "in".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::TitleNotInVec(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "notIn".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::TitleLt(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleLte(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleGt(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleGte(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleContains(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "contains".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleStartsWith(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "startsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleEndsWith(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "endsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::TitleNot(value) => (
                    "title".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "not".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::PublishedEquals(value) => (
                    "published".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        PrismaValue::Boolean(value).into(),
                    )])),
                ),
                Self::ViewsEquals(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        PrismaValue::Int(value as i64).into(),
                    )])),
                ),
                Self::ViewsInVec(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "in".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::Int(v as i64).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::ViewsNotInVec(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "notIn".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::Int(v as i64).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::ViewsLt(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lt".to_string(),
                        PrismaValue::Int(value as i64).into(),
                    )])),
                ),
                Self::ViewsLte(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lte".to_string(),
                        PrismaValue::Int(value as i64).into(),
                    )])),
                ),
                Self::ViewsGt(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gt".to_string(),
                        PrismaValue::Int(value as i64).into(),
                    )])),
                ),
                Self::ViewsGte(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gte".to_string(),
                        PrismaValue::Int(value as i64).into(),
                    )])),
                ),
                Self::ViewsNot(value) => (
                    "views".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "not".to_string(),
                        PrismaValue::Int(value as i64).into(),
                    )])),
                ),
                Self::DescEquals(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        value
                            .map(|value| PrismaValue::String(value).into())
                            .unwrap_or(QueryValue::Null),
                    )])),
                ),
                Self::DescInVec(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "in".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::DescNotInVec(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "notIn".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::DescLt(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::DescLte(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::DescGt(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::DescGte(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::DescContains(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "contains".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::DescStartsWith(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "startsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::DescEndsWith(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "endsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::DescNot(value) => (
                    "desc".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "not".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIs(value) => (
                    "author".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "is".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
                Self::AuthorIsNot(value) => (
                    "author".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "isNot".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
                Self::AuthorIdEquals(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "equals".to_string(),
                        value
                            .map(|value| PrismaValue::String(value).into())
                            .unwrap_or(QueryValue::Null),
                    )])),
                ),
                Self::AuthorIdInVec(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "in".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::AuthorIdNotInVec(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "notIn".to_string(),
                        QueryValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::String(v).into())
                                .collect(),
                        ),
                    )])),
                ),
                Self::AuthorIdLt(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIdLte(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "lte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIdGt(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gt".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIdGte(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "gte".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIdContains(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "contains".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIdStartsWith(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "startsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIdEndsWith(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "endsWith".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::AuthorIdNot(value) => (
                    "author_id".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "not".to_string(),
                        PrismaValue::String(value).into(),
                    )])),
                ),
                Self::CategoriesSome(value) => (
                    "categories".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "some".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
                Self::CategoriesEvery(value) => (
                    "categories".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "every".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
                Self::CategoriesNone(value) => (
                    "categories".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "none".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
                Self::FavouritersSome(value) => (
                    "favouriters".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "some".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
                Self::FavouritersEvery(value) => (
                    "favouriters".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "every".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
                Self::FavouritersNone(value) => (
                    "favouriters".to_string(),
                    SerializedWhereValue::Object(<[_]>::into_vec(box [(
                        "none".to_string(),
                        QueryValue::Object(
                            transform_equals(value.into_iter().map(Into::<SerializedWhere>::into))
                                .into_iter()
                                .collect(),
                        ),
                    )])),
                ),
            }
        }
    }
    pub enum UniqueWhereParam {
        TitleAuthorIdEquals(String, String),
        IdEquals(String),
    }
    impl From<UniqueWhereParam> for WhereParam {
        fn from(value: UniqueWhereParam) -> Self {
            match value {
                UniqueWhereParam::TitleAuthorIdEquals(title, author_id) => {
                    Self::TitleAuthorIdEquals(title, author_id)
                }
                UniqueWhereParam::IdEquals(value) => Self::IdEquals(value),
            }
        }
    }
    impl From<Operator<Self>> for WhereParam {
        fn from(op: Operator<Self>) -> Self {
            match op {
                Operator::Not(value) => Self::Not(value),
                Operator::And(value) => Self::And(value),
                Operator::Or(value) => Self::Or(value),
            }
        }
    }
    pub type FindManyArgs =
        prisma_client_rust::FindManyArgs<WhereParam, WithParam, OrderByParam, Cursor>;
    pub struct FindMany<'a> {
        ctx: QueryContext<'a>,
        args: FindManyArgs,
    }
    impl<'a> FindMany<'a> {
        pub async fn exec(self) -> QueryResult<Vec<Data>> {
            let Self { ctx, args } = self;
            ctx.execute(args.to_operation("Post", _outputs())).await
        }
        pub fn delete(self) -> DeleteMany<'a> {
            let Self { ctx, args } = self;
            DeleteMany {
                ctx,
                args: DeleteManyArgs::new(args.where_params),
            }
        }
        pub fn update(mut self, params: Vec<SetParam>) -> UpdateMany<'a> {
            let Self { ctx, args } = self;
            UpdateMany {
                ctx,
                args: UpdateManyArgs::new(args.where_params, params),
            }
        }
        pub fn with(mut self, params: impl Into<post::WithParam>) -> Self {
            self.args = self.args.with(params.into());
            self
        }
        pub fn order_by(mut self, param: post::OrderByParam) -> Self {
            self.args = self.args.order_by(param);
            self
        }
        pub fn skip(mut self, value: i64) -> Self {
            self.args = self.args.skip(value);
            self
        }
        pub fn take(mut self, value: i64) -> Self {
            self.args = self.args.take(value);
            self
        }
        pub fn cursor(mut self, value: impl Into<post::Cursor>) -> Self {
            self.args = self.args.cursor(value.into());
            self
        }
    }
    pub type FindFirstArgs =
        prisma_client_rust::FindFirstArgs<WhereParam, WithParam, OrderByParam, Cursor>;
    pub struct FindFirst<'a> {
        ctx: QueryContext<'a>,
        args: FindFirstArgs,
    }
    impl<'a> FindFirst<'a> {
        pub async fn exec(self) -> QueryResult<Option<Data>> {
            let Self { ctx, args } = self;
            ctx.execute(args.to_operation("Post", _outputs())).await
        }
        pub fn with(mut self, params: impl Into<post::WithParam>) -> Self {
            self.args = self.args.with(params.into());
            self
        }
        pub fn order_by(mut self, param: post::OrderByParam) -> Self {
            self.args = self.args.order_by(param);
            self
        }
        pub fn skip(mut self, value: i64) -> Self {
            self.args = self.args.skip(value);
            self
        }
        pub fn take(mut self, value: i64) -> Self {
            self.args = self.args.take(value);
            self
        }
        pub fn cursor(mut self, value: impl Into<post::Cursor>) -> Self {
            self.args = self.args.cursor(value.into());
            self
        }
    }
    pub type Args = prisma_client_rust::Args<WithParam>;
    pub type FindUniqueArgs = prisma_client_rust::FindUniqueArgs<WhereParam, WithParam>;
    pub struct FindUnique<'a> {
        ctx: QueryContext<'a>,
        args: FindUniqueArgs,
    }
    impl<'a> FindUnique<'a> {
        pub async fn exec(self) -> QueryResult<Option<Data>> {
            let Self { ctx, args } = self;
            ctx.execute(args.to_operation("Post", _outputs())).await
        }
        pub fn delete(self) -> Delete<'a> {
            let Self { ctx, args } = self;
            let FindUniqueArgs {
                where_param,
                with_params,
            } = args;
            Delete {
                ctx,
                args: DeleteArgs::new(where_param, with_params),
            }
        }
        pub fn with(mut self, params: impl Into<post::WithParam>) -> Self {
            self.args = self.args.with(params.into());
            self
        }
        pub fn update(mut self, params: Vec<SetParam>) -> Update<'a> {
            let Self { ctx, args } = self;
            let FindUniqueArgs {
                where_param,
                with_params,
            } = args;
            Update {
                ctx,
                args: UpdateArgs::new(where_param, params, with_params),
            }
        }
    }
    pub type CreateArgs = prisma_client_rust::CreateArgs<SetParam, WithParam>;
    pub struct Create<'a> {
        ctx: QueryContext<'a>,
        args: CreateArgs,
    }
    impl<'a> Create<'a> {
        pub async fn exec(self) -> QueryResult<Data> {
            let Self { ctx, args } = self;
            ctx.execute(args.to_operation("Post", _outputs())).await
        }
        pub fn with(mut self, params: impl Into<post::WithParam>) -> Self {
            self.args = self.args.with(params.into());
            self
        }
    }
    pub type UpdateArgs = prisma_client_rust::UpdateArgs<WhereParam, SetParam, WithParam>;
    pub struct Update<'a> {
        ctx: QueryContext<'a>,
        args: UpdateArgs,
    }
    impl<'a> Update<'a> {
        pub async fn exec(self) -> QueryResult<Option<Data>> {
            let Self { ctx, args } = self;
            let result = ctx.execute(args.to_operation("Post", _outputs())).await;
            match result {
                Err(QueryError::Execute(CoreError::InterpreterError(
                    InterpreterError::InterpretationError(msg, Some(interpreter_error)),
                ))) => match *interpreter_error {
                    InterpreterError::QueryGraphBuilderError(
                        QueryGraphBuilderError::RecordNotFound(_),
                    ) => Ok(None),
                    res => Err(QueryError::Execute(CoreError::InterpreterError(
                        InterpreterError::InterpretationError(msg, Some(Box::new(res))),
                    ))),
                },
                res => res,
            }
        }
        pub fn with(mut self, params: impl Into<post::WithParam>) -> Self {
            self.args = self.args.with(params.into());
            self
        }
    }
    pub type UpdateManyArgs = prisma_client_rust::UpdateManyArgs<WhereParam, SetParam>;
    pub struct UpdateMany<'a> {
        ctx: QueryContext<'a>,
        args: UpdateManyArgs,
    }
    impl<'a> UpdateMany<'a> {
        pub async fn exec(self) -> QueryResult<i64> {
            let Self { ctx, args } = self;
            ctx.execute(args.to_operation("Post"))
                .await
                .map(|res: BatchResult| res.count)
        }
    }
    pub type UpsertArgs = prisma_client_rust::UpsertArgs<WhereParam, SetParam, WithParam>;
    pub struct Upsert<'a> {
        ctx: QueryContext<'a>,
        args: UpsertArgs,
    }
    impl<'a> Upsert<'a> {
        pub async fn exec(self) -> QueryResult<Data> {
            let Self { ctx, args } = self;
            ctx.execute(args.to_operation("Post", _outputs())).await
        }
        pub fn create(
            mut self,
            title: title::Set,
            published: published::Set,
            mut params: Vec<SetParam>,
        ) -> Self {
            params.push(title.into());
            params.push(published.into());
            self.args = self.args.create(params);
            self
        }
        pub fn update(mut self, params: Vec<SetParam>) -> Self {
            self.args = self.args.update(params);
            self
        }
    }
    pub type DeleteArgs = prisma_client_rust::DeleteArgs<WhereParam, WithParam>;
    pub struct Delete<'a> {
        ctx: QueryContext<'a>,
        args: DeleteArgs,
    }
    impl<'a> Delete<'a> {
        pub async fn exec(self) -> QueryResult<Option<Data>> {
            let Self { ctx, args } = self;
            let result = ctx.execute(args.to_operation("Post", _outputs())).await;
            match result {
                Err(QueryError::Execute(CoreError::InterpreterError(
                    InterpreterError::InterpretationError(msg, Some(interpreter_error)),
                ))) => match *interpreter_error {
                    InterpreterError::QueryGraphBuilderError(
                        QueryGraphBuilderError::RecordNotFound(_),
                    ) => Ok(None),
                    res => Err(QueryError::Execute(CoreError::InterpreterError(
                        InterpreterError::InterpretationError(msg, Some(Box::new(res))),
                    ))),
                },
                res => res,
            }
        }
        pub fn with(mut self, params: impl Into<post::WithParam>) -> Self {
            self.args = self.args.with(params.into());
            self
        }
    }
    pub type DeleteManyArgs = prisma_client_rust::DeleteManyArgs<WhereParam>;
    pub struct DeleteMany<'a> {
        ctx: QueryContext<'a>,
        args: DeleteManyArgs,
    }
    impl<'a> DeleteMany<'a> {
        pub async fn exec(self) -> QueryResult<i64> {
            let Self { ctx, args } = self;
            ctx.execute(args.to_operation("Post"))
                .await
                .map(|res: BatchResult| res.count)
        }
    }
    pub struct Actions<'a> {
        pub client: &'a PrismaClient,
    }
    impl<'a> Actions<'a> {
        pub fn create(
            &self,
            title: title::Set,
            published: published::Set,
            mut params: Vec<SetParam>,
        ) -> Create {
            params.push(title.into());
            params.push(published.into());
            Create {
                ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                args: CreateArgs::new(params),
            }
        }
        pub fn find_unique(&self, param: UniqueWhereParam) -> FindUnique {
            FindUnique {
                ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                args: FindUniqueArgs::new(param.into()),
            }
        }
        pub fn find_first(&self, params: Vec<WhereParam>) -> FindFirst {
            FindFirst {
                ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                args: FindFirstArgs::new(params),
            }
        }
        pub fn find_many(&self, params: Vec<WhereParam>) -> FindMany {
            FindMany {
                ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                args: FindManyArgs::new(params),
            }
        }
        pub fn upsert(&self, param: UniqueWhereParam) -> Upsert {
            Upsert {
                ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                args: UpsertArgs::new(param.into()),
            }
        }
    }
}
