#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct UserRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct UserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
/// Generated client implementations.
pub mod accounts_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    /// graphql: extends
    pub struct AccountsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccountsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccountsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn me(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::User>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/federation.accounts.Accounts/Me");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// graphql: entity, inputs(string id), output(optional User user)
        pub async fn find_user_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRequest>,
        ) -> Result<tonic::Response<super::UserResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/federation.accounts.Accounts/FindUserById");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AccountsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AccountsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AccountsClient {{ ... }}")
        }
    }
}
/// Generated gateway implementations.
pub mod accounts_graphql {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub type AccountsSchema<T> = ::async_graphql::Schema<
        AccountsQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >;
    /// Create a GraphQL schema builder.
    pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
        AccountsQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        ::async_graphql::Schema::build(
            <AccountsQuery<T>>::default(),
            ::async_graphql::EmptyMutation,
            ::async_graphql::EmptySubscription,
        )
    }
    pub struct AccountsQuery<T> {
        _grpc_client: ::std::marker::PhantomData<super::accounts_client::AccountsClient<T>>,
    }
    #[::async_graphql::Object(name = "Query", extends)]
    impl<T> AccountsQuery<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub async fn me(
            &self,
            ctx: &::async_graphql::Context<'_>,
        ) -> ::async_graphql::Result<super::UserGraphQl> {
            let mut grpc_client = ctx
                .data::<super::accounts_client::AccountsClient<T>>()?
                .clone();
            let response = grpc_client
                .me(())
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::UserGraphQl>::from(response.into_inner());
            Ok(response)
        }
        /// graphql: entity, inputs(string id), output(optional User user)
        #[graphql(entity)]
        pub async fn find_user_by_id(
            &self,
            ctx: &::async_graphql::Context<'_>,
            id: ::prost::alloc::string::String,
        ) -> ::async_graphql::Result<::core::option::Option<super::UserGraphQl>> {
            let request = super::UserRequestGraphQlInput { id };
            let mut grpc_client = ctx
                .data::<super::accounts_client::AccountsClient<T>>()?
                .clone();
            let response = grpc_client
                .find_user_by_id(<super::UserRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::UserResponseGraphQl>::from(response.into_inner());
            Ok(response.user)
        }
    }
    impl<T> Default for AccountsQuery<T> {
        fn default() -> Self {
            Self {
                _grpc_client: ::std::marker::PhantomData,
            }
        }
    }
    impl<T> Clone for AccountsQuery<T> {
        fn clone(&self) -> Self {
            Self {
                _grpc_client: self._grpc_client.clone(),
            }
        }
    }
    impl<T> ::std::fmt::Debug for AccountsQuery<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "AccountsQuery {{ ... }}")
        }
    }
}
/// Generated server implementations.
pub mod accounts_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AccountsServer.
    #[async_trait]
    pub trait Accounts: Send + Sync + 'static {
        async fn me(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::User>, tonic::Status>;
        /// graphql: entity, inputs(string id), output(optional User user)
        async fn find_user_by_id(
            &self,
            request: tonic::Request<super::UserRequest>,
        ) -> Result<tonic::Response<super::UserResponse>, tonic::Status>;
    }
    /// graphql: extends
    #[derive(Debug)]
    pub struct AccountsServer<T: Accounts> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Accounts> AccountsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for AccountsServer<T>
    where
        T: Accounts,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/federation.accounts.Accounts/Me" => {
                    #[allow(non_camel_case_types)]
                    struct MeSvc<T: Accounts>(pub Arc<T>);
                    impl<T: Accounts> tonic::server::UnaryService<()> for MeSvc<T> {
                        type Response = super::User;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).me(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/federation.accounts.Accounts/FindUserById" => {
                    #[allow(non_camel_case_types)]
                    struct FindUserByIdSvc<T: Accounts>(pub Arc<T>);
                    impl<T: Accounts> tonic::server::UnaryService<super::UserRequest> for FindUserByIdSvc<T> {
                        type Response = super::UserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_user_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FindUserByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Accounts> Clone for AccountsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Accounts> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Accounts> tonic::transport::NamedService for AccountsServer<T> {
        const NAME: &'static str = "federation.accounts.Accounts";
    }
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "User")]
pub struct UserGraphQl {
    pub id: ::prost::alloc::string::String,
    pub username: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "UserInput")]
pub struct UserGraphQlInput {
    pub id: ::prost::alloc::string::String,
    pub username: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<User> for UserGraphQl {
    fn from(other: User) -> Self {
        let User { id, username, .. } = other;
        Self {
            id: id.into(),
            username: username.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserGraphQl> for User {
    fn from(other: UserGraphQl) -> Self {
        let UserGraphQl { id, username } = other;
        Self {
            id: id.into(),
            username: username.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<User> for UserGraphQlInput {
    fn from(other: User) -> Self {
        let User { id, username, .. } = other;
        Self {
            id: id.into(),
            username: username.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserGraphQlInput> for User {
    fn from(other: UserGraphQlInput) -> Self {
        let UserGraphQlInput { id, username } = other;
        Self {
            id: id.into(),
            username: username.into(),
        }
    }
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "UserRequest")]
pub struct UserRequestGraphQl {
    pub id: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "UserRequestInput")]
pub struct UserRequestGraphQlInput {
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<UserRequest> for UserRequestGraphQl {
    fn from(other: UserRequest) -> Self {
        let UserRequest { id, .. } = other;
        Self { id: id.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserRequestGraphQl> for UserRequest {
    fn from(other: UserRequestGraphQl) -> Self {
        let UserRequestGraphQl { id } = other;
        Self { id: id.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserRequest> for UserRequestGraphQlInput {
    fn from(other: UserRequest) -> Self {
        let UserRequest { id, .. } = other;
        Self { id: id.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserRequestGraphQlInput> for UserRequest {
    fn from(other: UserRequestGraphQlInput) -> Self {
        let UserRequestGraphQlInput { id } = other;
        Self { id: id.into() }
    }
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "UserResponse")]
pub struct UserResponseGraphQl {
    pub user: ::core::option::Option<UserGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "UserResponseInput")]
pub struct UserResponseGraphQlInput {
    pub user: ::core::option::Option<UserGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<UserResponse> for UserResponseGraphQl {
    fn from(other: UserResponse) -> Self {
        let UserResponse { user, .. } = other;
        Self {
            user: user.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserResponseGraphQl> for UserResponse {
    fn from(other: UserResponseGraphQl) -> Self {
        let UserResponseGraphQl { user } = other;
        Self {
            user: user.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserResponse> for UserResponseGraphQlInput {
    fn from(other: UserResponse) -> Self {
        let UserResponse { user, .. } = other;
        Self {
            user: user.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserResponseGraphQlInput> for UserResponse {
    fn from(other: UserResponseGraphQlInput) -> Self {
        let UserResponseGraphQlInput { user } = other;
        Self {
            user: user.map(Into::into),
        }
    }
}
