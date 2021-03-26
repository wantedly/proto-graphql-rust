extern crate alloc;
extern crate self as proto_graphql;

#[doc(hidden)]
pub use futures_util;
#[doc(hidden)]
pub use serde;

#[doc(hidden)]
pub mod prost_types;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoopQuery;

#[async_graphql::Object]
impl NoopQuery {
    #[doc(hidden)]
    #[graphql(name = "_noop", visible = false)]
    pub async fn _noop(&self) -> async_graphql::Result<EmptyGraphQl> {
        Ok(EmptyGraphQl::default())
    }
}

#[doc(hidden)]
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    async_graphql::SimpleObject,
    serde::Serialize,
    serde::Deserialize,
)]
#[graphql(name = "Empty")]
pub struct EmptyGraphQl {
    #[graphql(name = "_noop", visible = false)]
    _noop: Option<bool>,
}

#[doc(hidden)]
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    async_graphql::InputObject,
    serde::Serialize,
    serde::Deserialize,
)]
#[graphql(name = "EmptyInput")]
pub struct EmptyGraphQlInput {
    #[graphql(name = "_noop", visible = false, default)]
    _noop: Option<bool>,
}

impl From<()> for EmptyGraphQl {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<EmptyGraphQl> for () {
    fn from(_: EmptyGraphQl) -> Self {}
}

impl From<()> for EmptyGraphQlInput {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<EmptyGraphQlInput> for () {
    fn from(_: EmptyGraphQlInput) -> Self {}
}
