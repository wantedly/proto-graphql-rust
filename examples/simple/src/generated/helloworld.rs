#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod greeter_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GreeterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GreeterClient<tonic::transport::Channel> {
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
    impl<T> GreeterClient<T>
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
        pub async fn say_hello(
            &mut self,
            request: impl tonic::IntoRequest<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/helloworld.Greeter/SayHello");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GreeterClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GreeterClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GreeterClient {{ ... }}")
        }
    }
}
/// Generated gateway implementations.
pub mod greeter_graphql {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub type GreeterSchema<T> = ::async_graphql::Schema<
        GreeterQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >;
    /// Create a GraphQL schema builder.
    pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
        GreeterQuery<T>,
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
            <GreeterQuery<T>>::default(),
            ::async_graphql::EmptyMutation,
            ::async_graphql::EmptySubscription,
        )
    }
    pub struct GreeterQuery<T> {
        _grpc_client: ::std::marker::PhantomData<super::greeter_client::GreeterClient<T>>,
    }
    #[::async_graphql::Object(name = "GreeterQuery")]
    impl<T> GreeterQuery<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub async fn say_hello(
            &self,
            ctx: &::async_graphql::Context<'_>,
            request: super::HelloRequestGraphQlInput,
        ) -> ::async_graphql::Result<super::HelloReplyGraphQl> {
            let mut grpc_client = ctx
                .data::<super::greeter_client::GreeterClient<T>>()?
                .clone();
            let response = grpc_client
                .say_hello(<super::HelloRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::HelloReplyGraphQl>::from(response.into_inner());
            Ok(response)
        }
    }
    impl<T> Default for GreeterQuery<T> {
        fn default() -> Self {
            Self {
                _grpc_client: ::std::marker::PhantomData,
            }
        }
    }
    impl<T> Clone for GreeterQuery<T> {
        fn clone(&self) -> Self {
            Self {
                _grpc_client: self._grpc_client.clone(),
            }
        }
    }
    impl<T> ::std::fmt::Debug for GreeterQuery<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "GreeterQuery {{ ... }}")
        }
    }
}
/// Generated server implementations.
pub mod greeter_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with GreeterServer.
    #[async_trait]
    pub trait Greeter: Send + Sync + 'static {
        async fn say_hello(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GreeterServer<T: Greeter> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Greeter> GreeterServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GreeterServer<T>
    where
        T: Greeter,
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
                "/helloworld.Greeter/SayHello" => {
                    #[allow(non_camel_case_types)]
                    struct SayHelloSvc<T: Greeter>(pub Arc<T>);
                    impl<T: Greeter> tonic::server::UnaryService<super::HelloRequest> for SayHelloSvc<T> {
                        type Response = super::HelloReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).say_hello(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SayHelloSvc(inner);
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
    impl<T: Greeter> Clone for GreeterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Greeter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Greeter> tonic::transport::NamedService for GreeterServer<T> {
        const NAME: &'static str = "helloworld.Greeter";
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
#[graphql(name = "HelloRequest")]
pub struct HelloRequestGraphQl {
    pub name: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "HelloRequestInput")]
pub struct HelloRequestGraphQlInput {
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequest> for HelloRequestGraphQl {
    fn from(other: HelloRequest) -> Self {
        let HelloRequest { name, .. } = other;
        Self { name: name.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequestGraphQl> for HelloRequest {
    fn from(other: HelloRequestGraphQl) -> Self {
        let HelloRequestGraphQl { name } = other;
        Self { name: name.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequest> for HelloRequestGraphQlInput {
    fn from(other: HelloRequest) -> Self {
        let HelloRequest { name, .. } = other;
        Self { name: name.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequestGraphQlInput> for HelloRequest {
    fn from(other: HelloRequestGraphQlInput) -> Self {
        let HelloRequestGraphQlInput { name } = other;
        Self { name: name.into() }
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
#[graphql(name = "HelloReply")]
pub struct HelloReplyGraphQl {
    pub message: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "HelloReplyInput")]
pub struct HelloReplyGraphQlInput {
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<HelloReply> for HelloReplyGraphQl {
    fn from(other: HelloReply) -> Self {
        let HelloReply { message, .. } = other;
        Self {
            message: message.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloReplyGraphQl> for HelloReply {
    fn from(other: HelloReplyGraphQl) -> Self {
        let HelloReplyGraphQl { message } = other;
        Self {
            message: message.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloReply> for HelloReplyGraphQlInput {
    fn from(other: HelloReply) -> Self {
        let HelloReply { message, .. } = other;
        Self {
            message: message.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloReplyGraphQlInput> for HelloReply {
    fn from(other: HelloReplyGraphQlInput) -> Self {
        let HelloReplyGraphQlInput { message } = other;
        Self {
            message: message.into(),
        }
    }
}
