#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Review {
    #[prost(string, tag = "1")]
    pub body: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub author: ::core::option::Option<User>,
    #[prost(message, optional, tag = "3")]
    pub product: ::core::option::Option<Product>,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ReviewResponse {
    #[prost(message, repeated, tag = "1")]
    pub reviews: ::prost::alloc::vec::Vec<Review>,
}
/// graphql: extends, complex
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct User {
    /// graphql: external
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct UserRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// graphql: extends, complex
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Product {
    /// graphql: external
    #[prost(string, tag = "1")]
    pub upc: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ProductRequest {
    #[prost(string, tag = "1")]
    pub upc: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod reviews_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ReviewsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReviewsClient<tonic::transport::Channel> {
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
    impl<T> ReviewsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ReviewsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ReviewsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// graphql: output(repeated Review reviews)
        pub async fn reviews(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/federation.reviews.Reviews/Reviews");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// graphql: entity, inputs(string id)
        pub async fn find_user_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRequest>,
        ) -> Result<tonic::Response<super::User>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/federation.reviews.Reviews/FindUserById");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// graphql: entity, inputs(string upc)
        pub async fn find_product_by_upc(
            &mut self,
            request: impl tonic::IntoRequest<super::ProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/federation.reviews.Reviews/FindProductByUpc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// graphql: inputs(string id), output(repeated Review reviews)
        pub async fn reviews_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRequest>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/federation.reviews.Reviews/ReviewsById");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// graphql: inputs(string upc), output(repeated Review reviews)
        pub async fn reviews_by_upc(
            &mut self,
            request: impl tonic::IntoRequest<super::ProductRequest>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/federation.reviews.Reviews/ReviewsByUpc");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated gateway implementations.
pub mod reviews_graphql {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub type ReviewsSchema<T> = ::async_graphql::Schema<
        ReviewsQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >;
    /// Create a GraphQL schema builder.
    pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
        ReviewsQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        ::async_graphql::Schema::build(
            <ReviewsQuery<T>>::default(),
            ::async_graphql::EmptyMutation,
            ::async_graphql::EmptySubscription,
        )
    }
    pub struct ReviewsQuery<T> {
        _grpc_client: ::std::marker::PhantomData<super::reviews_client::ReviewsClient<T>>,
    }
    #[::async_graphql::Object(name = "Query")]
    impl<T> ReviewsQuery<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        /// graphql: output(repeated Review reviews)
        pub async fn reviews(
            &self,
            ctx: &::async_graphql::Context<'_>,
        ) -> ::async_graphql::Result<::prost::alloc::vec::Vec<super::ReviewGraphQl>> {
            let mut grpc_client = ctx
                .data::<super::reviews_client::ReviewsClient<T>>()?
                .clone();
            let response = grpc_client
                .reviews(())
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ReviewResponseGraphQl>::from(response.into_inner());
            Ok(response.reviews)
        }
        /// graphql: entity, inputs(string id)
        #[graphql(entity)]
        pub async fn find_user_by_id(
            &self,
            ctx: &::async_graphql::Context<'_>,
            id: ::prost::alloc::string::String,
        ) -> ::async_graphql::Result<super::UserGraphQl> {
            let request = super::UserRequestGraphQlInput { id };
            let mut grpc_client = ctx
                .data::<super::reviews_client::ReviewsClient<T>>()?
                .clone();
            let response = grpc_client
                .find_user_by_id(<super::UserRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::UserGraphQl>::from(response.into_inner());
            Ok(response)
        }
        /// graphql: entity, inputs(string upc)
        #[graphql(entity)]
        pub async fn find_product_by_upc(
            &self,
            ctx: &::async_graphql::Context<'_>,
            upc: ::prost::alloc::string::String,
        ) -> ::async_graphql::Result<super::ProductGraphQl> {
            let request = super::ProductRequestGraphQlInput { upc };
            let mut grpc_client = ctx
                .data::<super::reviews_client::ReviewsClient<T>>()?
                .clone();
            let response = grpc_client
                .find_product_by_upc(<super::ProductRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ProductGraphQl>::from(response.into_inner());
            Ok(response)
        }
        /// graphql: inputs(string id), output(repeated Review reviews)
        pub async fn reviews_by_id(
            &self,
            ctx: &::async_graphql::Context<'_>,
            id: ::prost::alloc::string::String,
        ) -> ::async_graphql::Result<::prost::alloc::vec::Vec<super::ReviewGraphQl>> {
            let request = super::UserRequestGraphQlInput { id };
            let mut grpc_client = ctx
                .data::<super::reviews_client::ReviewsClient<T>>()?
                .clone();
            let response = grpc_client
                .reviews_by_id(<super::UserRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ReviewResponseGraphQl>::from(response.into_inner());
            Ok(response.reviews)
        }
        /// graphql: inputs(string upc), output(repeated Review reviews)
        pub async fn reviews_by_upc(
            &self,
            ctx: &::async_graphql::Context<'_>,
            upc: ::prost::alloc::string::String,
        ) -> ::async_graphql::Result<::prost::alloc::vec::Vec<super::ReviewGraphQl>> {
            let request = super::ProductRequestGraphQlInput { upc };
            let mut grpc_client = ctx
                .data::<super::reviews_client::ReviewsClient<T>>()?
                .clone();
            let response = grpc_client
                .reviews_by_upc(<super::ProductRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ReviewResponseGraphQl>::from(response.into_inner());
            Ok(response.reviews)
        }
    }
    impl<T> Default for ReviewsQuery<T> {
        fn default() -> Self {
            Self {
                _grpc_client: ::std::marker::PhantomData,
            }
        }
    }
    impl<T> Clone for ReviewsQuery<T> {
        fn clone(&self) -> Self {
            Self {
                _grpc_client: self._grpc_client,
            }
        }
    }
    impl<T> ::std::fmt::Debug for ReviewsQuery<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "ReviewsQuery {{ ... }}")
        }
    }
}
/// Generated server implementations.
pub mod reviews_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ReviewsServer.
    #[async_trait]
    pub trait Reviews: Send + Sync + 'static {
        /// graphql: output(repeated Review reviews)
        async fn reviews(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status>;
        /// graphql: entity, inputs(string id)
        async fn find_user_by_id(
            &self,
            request: tonic::Request<super::UserRequest>,
        ) -> Result<tonic::Response<super::User>, tonic::Status>;
        /// graphql: entity, inputs(string upc)
        async fn find_product_by_upc(
            &self,
            request: tonic::Request<super::ProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status>;
        /// graphql: inputs(string id), output(repeated Review reviews)
        async fn reviews_by_id(
            &self,
            request: tonic::Request<super::UserRequest>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status>;
        /// graphql: inputs(string upc), output(repeated Review reviews)
        async fn reviews_by_upc(
            &self,
            request: tonic::Request<super::ProductRequest>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ReviewsServer<T: Reviews> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Reviews> ReviewsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReviewsServer<T>
    where
        T: Reviews,
        B: Body + Send + Sync + 'static,
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
                "/federation.reviews.Reviews/Reviews" => {
                    #[allow(non_camel_case_types)]
                    struct ReviewsSvc<T: Reviews>(pub Arc<T>);
                    impl<T: Reviews> tonic::server::UnaryService<()> for ReviewsSvc<T> {
                        type Response = super::ReviewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reviews(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReviewsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/federation.reviews.Reviews/FindUserById" => {
                    #[allow(non_camel_case_types)]
                    struct FindUserByIdSvc<T: Reviews>(pub Arc<T>);
                    impl<T: Reviews> tonic::server::UnaryService<super::UserRequest> for FindUserByIdSvc<T> {
                        type Response = super::User;
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
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FindUserByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/federation.reviews.Reviews/FindProductByUpc" => {
                    #[allow(non_camel_case_types)]
                    struct FindProductByUpcSvc<T: Reviews>(pub Arc<T>);
                    impl<T: Reviews> tonic::server::UnaryService<super::ProductRequest> for FindProductByUpcSvc<T> {
                        type Response = super::Product;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_product_by_upc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FindProductByUpcSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/federation.reviews.Reviews/ReviewsById" => {
                    #[allow(non_camel_case_types)]
                    struct ReviewsByIdSvc<T: Reviews>(pub Arc<T>);
                    impl<T: Reviews> tonic::server::UnaryService<super::UserRequest> for ReviewsByIdSvc<T> {
                        type Response = super::ReviewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reviews_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReviewsByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/federation.reviews.Reviews/ReviewsByUpc" => {
                    #[allow(non_camel_case_types)]
                    struct ReviewsByUpcSvc<T: Reviews>(pub Arc<T>);
                    impl<T: Reviews> tonic::server::UnaryService<super::ProductRequest> for ReviewsByUpcSvc<T> {
                        type Response = super::ReviewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reviews_by_upc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReviewsByUpcSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Reviews> Clone for ReviewsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Reviews> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Reviews> tonic::transport::NamedService for ReviewsServer<T> {
        const NAME: &'static str = "federation.reviews.Reviews";
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
#[graphql(name = "Review")]
pub struct ReviewGraphQl {
    pub body: ::prost::alloc::string::String,
    pub author: ::core::option::Option<UserGraphQl>,
    pub product: ::core::option::Option<ProductGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ReviewInput")]
pub struct ReviewGraphQlInput {
    pub body: ::prost::alloc::string::String,
    pub author: ::core::option::Option<UserGraphQlInput>,
    pub product: ::core::option::Option<ProductGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<Review> for ReviewGraphQl {
    fn from(other: Review) -> Self {
        let Review {
            body,
            author,
            product,
            ..
        } = other;
        Self {
            body: body.into(),
            author: author.map(Into::into),
            product: product.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ReviewGraphQl> for Review {
    fn from(other: ReviewGraphQl) -> Self {
        let ReviewGraphQl {
            body,
            author,
            product,
        } = other;
        Self {
            body: body.into(),
            author: author.map(Into::into),
            product: product.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Review> for ReviewGraphQlInput {
    fn from(other: Review) -> Self {
        let Review {
            body,
            author,
            product,
            ..
        } = other;
        Self {
            body: body.into(),
            author: author.map(Into::into),
            product: product.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ReviewGraphQlInput> for Review {
    fn from(other: ReviewGraphQlInput) -> Self {
        let ReviewGraphQlInput {
            body,
            author,
            product,
        } = other;
        Self {
            body: body.into(),
            author: author.map(Into::into),
            product: product.map(Into::into),
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
#[graphql(name = "ReviewResponse")]
pub struct ReviewResponseGraphQl {
    pub reviews: ::prost::alloc::vec::Vec<ReviewGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ReviewResponseInput")]
pub struct ReviewResponseGraphQlInput {
    pub reviews: ::prost::alloc::vec::Vec<ReviewGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ReviewResponse> for ReviewResponseGraphQl {
    fn from(other: ReviewResponse) -> Self {
        let ReviewResponse { reviews, .. } = other;
        Self {
            reviews: reviews.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ReviewResponseGraphQl> for ReviewResponse {
    fn from(other: ReviewResponseGraphQl) -> Self {
        let ReviewResponseGraphQl { reviews } = other;
        Self {
            reviews: reviews.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ReviewResponse> for ReviewResponseGraphQlInput {
    fn from(other: ReviewResponse) -> Self {
        let ReviewResponse { reviews, .. } = other;
        Self {
            reviews: reviews.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ReviewResponseGraphQlInput> for ReviewResponse {
    fn from(other: ReviewResponseGraphQlInput) -> Self {
        let ReviewResponseGraphQlInput { reviews } = other;
        Self {
            reviews: reviews.into_iter().map(Into::into).collect(),
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
#[graphql(name = "User", extends, complex)]
pub struct UserGraphQl {
    #[graphql(external)]
    pub id: ::prost::alloc::string::String,
}
/// graphql: extends, complex
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
    /// graphql: external
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<User> for UserGraphQl {
    fn from(other: User) -> Self {
        let User { id, .. } = other;
        Self { id: id.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserGraphQl> for User {
    fn from(other: UserGraphQl) -> Self {
        let UserGraphQl { id } = other;
        Self { id: id.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<User> for UserGraphQlInput {
    fn from(other: User) -> Self {
        let User { id, .. } = other;
        Self { id: id.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UserGraphQlInput> for User {
    fn from(other: UserGraphQlInput) -> Self {
        let UserGraphQlInput { id } = other;
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
#[graphql(name = "Product", extends, complex)]
pub struct ProductGraphQl {
    #[graphql(external)]
    pub upc: ::prost::alloc::string::String,
}
/// graphql: extends, complex
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductInput")]
pub struct ProductGraphQlInput {
    /// graphql: external
    pub upc: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<Product> for ProductGraphQl {
    fn from(other: Product) -> Self {
        let Product { upc, .. } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductGraphQl> for Product {
    fn from(other: ProductGraphQl) -> Self {
        let ProductGraphQl { upc } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Product> for ProductGraphQlInput {
    fn from(other: Product) -> Self {
        let Product { upc, .. } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductGraphQlInput> for Product {
    fn from(other: ProductGraphQlInput) -> Self {
        let ProductGraphQlInput { upc } = other;
        Self { upc: upc.into() }
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
#[graphql(name = "ProductRequest")]
pub struct ProductRequestGraphQl {
    pub upc: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductRequestInput")]
pub struct ProductRequestGraphQlInput {
    pub upc: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequest> for ProductRequestGraphQl {
    fn from(other: ProductRequest) -> Self {
        let ProductRequest { upc, .. } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequestGraphQl> for ProductRequest {
    fn from(other: ProductRequestGraphQl) -> Self {
        let ProductRequestGraphQl { upc } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequest> for ProductRequestGraphQlInput {
    fn from(other: ProductRequest) -> Self {
        let ProductRequest { upc, .. } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequestGraphQlInput> for ProductRequest {
    fn from(other: ProductRequestGraphQlInput) -> Self {
        let ProductRequestGraphQlInput { upc } = other;
        Self { upc: upc.into() }
    }
}
