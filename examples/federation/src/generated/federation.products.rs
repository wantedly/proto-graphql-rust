#[derive(
    Clone,
    PartialEq,
    :: prost :: Message,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Product")]
pub struct Product {
    #[prost(string, tag = "1")]
    pub upc: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub price: i32,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductInput")]
pub struct ProductInput {
    pub upc: ::prost::alloc::string::String,
    pub name: ::prost::alloc::string::String,
    pub price: i32,
}
#[allow(clippy::useless_conversion)]
impl From<Product> for ProductInput {
    fn from(other: Product) -> Self {
        let Product {
            upc, name, price, ..
        } = other;
        Self {
            upc: upc.into(),
            name: name.into(),
            price: price.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductInput> for Product {
    fn from(other: ProductInput) -> Self {
        let ProductInput { upc, name, price } = other;
        Self {
            upc: upc.into(),
            name: name.into(),
            price: price.into(),
        }
    }
}
#[derive(
    Clone,
    PartialEq,
    :: prost :: Message,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductRequest")]
pub struct ProductRequest {
    #[prost(string, tag = "1")]
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
pub struct ProductRequestInput {
    pub upc: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequest> for ProductRequestInput {
    fn from(other: ProductRequest) -> Self {
        let ProductRequest { upc, .. } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequestInput> for ProductRequest {
    fn from(other: ProductRequestInput) -> Self {
        let ProductRequestInput { upc } = other;
        Self { upc: upc.into() }
    }
}
#[derive(
    Clone,
    PartialEq,
    :: prost :: Message,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductsResponse")]
pub struct ProductsResponse {
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductsResponseInput")]
pub struct ProductsResponseInput {
    pub products: ::prost::alloc::vec::Vec<ProductInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ProductsResponse> for ProductsResponseInput {
    fn from(other: ProductsResponse) -> Self {
        let ProductsResponse { products, .. } = other;
        Self {
            products: products.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductsResponseInput> for ProductsResponse {
    fn from(other: ProductsResponseInput) -> Self {
        let ProductsResponseInput { products } = other;
        Self {
            products: products.into_iter().map(Into::into).collect(),
        }
    }
}
#[derive(
    Clone,
    PartialEq,
    :: prost :: Message,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductResponse")]
pub struct ProductResponse {
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<Product>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ProductResponseInput")]
pub struct ProductResponseInput {
    pub product: ::core::option::Option<ProductInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ProductResponse> for ProductResponseInput {
    fn from(other: ProductResponse) -> Self {
        let ProductResponse { product, .. } = other;
        Self {
            product: product.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductResponseInput> for ProductResponse {
    fn from(other: ProductResponseInput) -> Self {
        let ProductResponseInput { product } = other;
        Self {
            product: product.map(Into::into),
        }
    }
}
/// Generated client implementations.
pub mod products_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    /// graphql: extends
    pub struct ProductsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProductsClient<tonic::transport::Channel> {
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
    impl<T> ProductsClient<T>
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
        /// graphql: output(repeated Product products)
        pub async fn top_products(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::ProductsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/federation.products.Products/TopProducts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// graphql: entity, inputs(string upc), output(optional Product product)
        pub async fn find_product_by_upc(
            &mut self,
            request: impl tonic::IntoRequest<super::ProductRequest>,
        ) -> Result<tonic::Response<super::ProductResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/federation.products.Products/FindProductByUpc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProductsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProductsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProductsClient {{ ... }}")
        }
    }
}
/// Generated gateway implementations.
pub mod products_graphql {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub type ProductsSchema<T> = ::async_graphql::Schema<
        ProductsQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >;
    /// Create a GraphQL schema builder.
    pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
        ProductsQuery<T>,
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
            <ProductsQuery<T>>::default(),
            ::async_graphql::EmptyMutation,
            ::async_graphql::EmptySubscription,
        )
    }
    pub struct ProductsQuery<T> {
        _grpc_client: ::std::marker::PhantomData<super::products_client::ProductsClient<T>>,
    }
    #[::async_graphql::Object(name = "Query", extends)]
    impl<T> ProductsQuery<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        /// graphql: output(repeated Product products)
        pub async fn top_products(
            &self,
            ctx: &::async_graphql::Context<'_>,
        ) -> ::async_graphql::Result<::prost::alloc::vec::Vec<super::Product>> {
            let mut grpc_client = ctx
                .data::<super::products_client::ProductsClient<T>>()?
                .clone();
            let response = grpc_client
                .top_products(())
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ProductsResponse>::from(response.into_inner());
            Ok(response.products)
        }
        /// graphql: entity, inputs(string upc), output(optional Product product)
        #[graphql(entity)]
        pub async fn find_product_by_upc(
            &self,
            ctx: &::async_graphql::Context<'_>,
            upc: ::prost::alloc::string::String,
        ) -> ::async_graphql::Result<::core::option::Option<super::Product>> {
            let request = super::ProductRequestInput { upc };
            let mut grpc_client = ctx
                .data::<super::products_client::ProductsClient<T>>()?
                .clone();
            let response = grpc_client
                .find_product_by_upc(<super::ProductRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ProductResponse>::from(response.into_inner());
            Ok(response.product)
        }
    }
    impl<T> Default for ProductsQuery<T> {
        fn default() -> Self {
            Self {
                _grpc_client: ::std::marker::PhantomData,
            }
        }
    }
    impl<T> Clone for ProductsQuery<T> {
        fn clone(&self) -> Self {
            Self {
                _grpc_client: self._grpc_client.clone(),
            }
        }
    }
    impl<T> ::std::fmt::Debug for ProductsQuery<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "ProductsQuery {{ ... }}")
        }
    }
}
/// Generated server implementations.
pub mod products_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ProductsServer.
    #[async_trait]
    pub trait Products: Send + Sync + 'static {
        /// graphql: output(repeated Product products)
        async fn top_products(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::ProductsResponse>, tonic::Status>;
        /// graphql: entity, inputs(string upc), output(optional Product product)
        async fn find_product_by_upc(
            &self,
            request: tonic::Request<super::ProductRequest>,
        ) -> Result<tonic::Response<super::ProductResponse>, tonic::Status>;
    }
    /// graphql: extends
    #[derive(Debug)]
    pub struct ProductsServer<T: Products> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Products> ProductsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ProductsServer<T>
    where
        T: Products,
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
                "/federation.products.Products/TopProducts" => {
                    #[allow(non_camel_case_types)]
                    struct TopProductsSvc<T: Products>(pub Arc<T>);
                    impl<T: Products> tonic::server::UnaryService<()> for TopProductsSvc<T> {
                        type Response = super::ProductsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).top_products(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TopProductsSvc(inner);
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
                "/federation.products.Products/FindProductByUpc" => {
                    #[allow(non_camel_case_types)]
                    struct FindProductByUpcSvc<T: Products>(pub Arc<T>);
                    impl<T: Products> tonic::server::UnaryService<super::ProductRequest> for FindProductByUpcSvc<T> {
                        type Response = super::ProductResponse;
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FindProductByUpcSvc(inner);
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
    impl<T: Products> Clone for ProductsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Products> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Products> tonic::transport::NamedService for ProductsServer<T> {
        const NAME: &'static str = "federation.products.Products";
    }
}
