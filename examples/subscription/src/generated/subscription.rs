#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct HelloRequest {
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "2")]
    pub interval: ::core::option::Option<f32>,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod subscription_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct SubscriptionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SubscriptionClient<tonic::transport::Channel> {
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
    impl<T> SubscriptionClient<T>
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
        pub async fn server_streaming(
            &mut self,
            request: impl tonic::IntoRequest<super::HelloRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::HelloReply>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/subscription.Subscription/ServerStreaming");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for SubscriptionClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SubscriptionClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SubscriptionClient {{ ... }}")
        }
    }
}
/// Generated gateway implementations.
pub mod subscription_graphql {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub type SubscriptionSchema<T> = ::async_graphql::Schema<
        ::proto_graphql::NoopQuery,
        ::async_graphql::EmptyMutation,
        SubscriptionSubscription<T>,
    >;
    /// Create a GraphQL schema builder.
    pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
        ::proto_graphql::NoopQuery,
        ::async_graphql::EmptyMutation,
        SubscriptionSubscription<T>,
    >
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        ::async_graphql::Schema::build(
            ::proto_graphql::NoopQuery,
            ::async_graphql::EmptyMutation,
            <SubscriptionSubscription<T>>::default(),
        )
    }
    pub struct SubscriptionSubscription<T> {
        _grpc_client: ::std::marker::PhantomData<super::subscription_client::SubscriptionClient<T>>,
    }
    #[::async_graphql::Subscription(name = "SubscriptionSubscription")]
    impl<T> SubscriptionSubscription<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub async fn server_streaming(
            &self,
            ctx: &::async_graphql::Context<'_>,
            request: super::HelloRequestGraphQlInput,
        ) -> ::async_graphql::Result<
            impl ::proto_graphql::futures_util::stream::Stream<
                    Item = ::async_graphql::Result<super::HelloReplyGraphQl>,
                > + '_,
        > {
            let mut grpc_client = ctx
                .data::<super::subscription_client::SubscriptionClient<T>>()?
                .clone();
            let response = grpc_client
                .server_streaming(<super::HelloRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            Ok(::proto_graphql::futures_util::stream::StreamExt::map(
                response.into_inner(),
                |res| match res {
                    ::std::result::Result::Ok(ok) => {
                        ::std::result::Result::Ok(<super::HelloReplyGraphQl>::from(ok))
                    }
                    ::std::result::Result::Err(e) => {
                        ::std::result::Result::Err(::async_graphql::Error::new(e.to_string()))
                    }
                },
            ))
        }
    }
    impl<T> Default for SubscriptionSubscription<T> {
        fn default() -> Self {
            Self {
                _grpc_client: ::std::marker::PhantomData,
            }
        }
    }
    impl<T> Clone for SubscriptionSubscription<T> {
        fn clone(&self) -> Self {
            Self {
                _grpc_client: self._grpc_client,
            }
        }
    }
    impl<T> ::std::fmt::Debug for SubscriptionSubscription<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "SubscriptionSubscription {{ ... }}")
        }
    }
}
/// Generated server implementations.
pub mod subscription_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SubscriptionServer.
    #[async_trait]
    pub trait Subscription: Send + Sync + 'static {
        ///Server streaming response type for the ServerStreaming method.
        type ServerStreamingStream: futures_core::Stream<Item = Result<super::HelloReply, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn server_streaming(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> Result<tonic::Response<Self::ServerStreamingStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SubscriptionServer<T: Subscription> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Subscription> SubscriptionServer<T> {
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
    impl<T, B> Service<http::Request<B>> for SubscriptionServer<T>
    where
        T: Subscription,
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
                "/subscription.Subscription/ServerStreaming" => {
                    #[allow(non_camel_case_types)]
                    struct ServerStreamingSvc<T: Subscription>(pub Arc<T>);
                    impl<T: Subscription> tonic::server::ServerStreamingService<super::HelloRequest>
                        for ServerStreamingSvc<T>
                    {
                        type Response = super::HelloReply;
                        type ResponseStream = T::ServerStreamingStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).server_streaming(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ServerStreamingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: Subscription> Clone for SubscriptionServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Subscription> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Subscription> tonic::transport::NamedService for SubscriptionServer<T> {
        const NAME: &'static str = "subscription.Subscription";
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
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    pub interval: ::core::option::Option<f32>,
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
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    pub interval: ::core::option::Option<f32>,
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequest> for HelloRequestGraphQl {
    fn from(other: HelloRequest) -> Self {
        let HelloRequest {
            names, interval, ..
        } = other;
        Self {
            names: names.into_iter().map(Into::into).collect(),
            interval: interval.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequestGraphQl> for HelloRequest {
    fn from(other: HelloRequestGraphQl) -> Self {
        let HelloRequestGraphQl { names, interval } = other;
        Self {
            names: names.into_iter().map(Into::into).collect(),
            interval: interval.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequest> for HelloRequestGraphQlInput {
    fn from(other: HelloRequest) -> Self {
        let HelloRequest {
            names, interval, ..
        } = other;
        Self {
            names: names.into_iter().map(Into::into).collect(),
            interval: interval.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequestGraphQlInput> for HelloRequest {
    fn from(other: HelloRequestGraphQlInput) -> Self {
        let HelloRequestGraphQlInput { names, interval } = other;
        Self {
            names: names.into_iter().map(Into::into).collect(),
            interval: interval.map(Into::into),
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
