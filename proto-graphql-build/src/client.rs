use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use super::{generate_doc_comments, parse_attrs, will_rename, GraphqlAnnotations, Method, Service};

/// Generate service for gRPC client and GraphQL server.
///
/// This takes some `Service` and will generate a `TokenStream` that contains
/// a public module with the generated client.
pub(crate) fn generate(
    service: &impl Service,
    emit_package: bool,
    proto_path: &str,
    enable_federation: bool,
    compile_well_known_types: bool,
) -> TokenStream {
    let grpc_client = tonic_build::client::generate(
        service,
        emit_package,
        proto_path,
        compile_well_known_types,
        &tonic_build::Attributes::default(),
    );

    let grpc_service_name = format_ident!("{}Client", service.name());
    let grpc_client_mod = format_ident!("{}_client", service.name().to_snake_case());
    let grpc_client_ty = quote! { super::#grpc_client_mod::#grpc_service_name<T> };

    let graphql_schema_name = format_ident!("{}Schema", service.name());

    let graphql_mod = format_ident!("{}_graphql", service.name().to_snake_case());
    let (mut query, mut subscription) = generate_methods(
        service,
        proto_path,
        compile_well_known_types,
        &grpc_client_ty,
    );

    let mut service_doc = parse_attrs(generate_doc_comments(service.comment()));
    let mut annotations = GraphqlAnnotations::default();
    let impl_annotations = annotations.visit(&mut service_doc, false);

    let where_clause = quote! {
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
            T::Future: Send,
            T::ResponseBody: Body + Send + Sync + 'static,
            T::Error: Into<StdError>,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    };

    let mut query_ty = quote! { ::proto_graphql::NoopQuery };
    let mut query_init = query_ty.clone();
    if !query.is_empty() {
        // The option to allow rename probably makes more sense.
        let graphql_query_name_str = if enable_federation {
            "Query".to_string()
        } else {
            format!("{}Query", service.name())
        };
        let graphql_query_name = format_ident!("{}Query", service.name());
        query_ty = quote! { #graphql_query_name<T> };
        query_init = quote! { <#query_ty>::default() };
        let query_struct_debug = format!("{} {{{{ ... }}}}", &graphql_query_name);
        query = quote! {
            pub struct #query_ty {
                _grpc_client: ::std::marker::PhantomData<#grpc_client_ty>,
            }
            #[::async_graphql::Object(name = #graphql_query_name_str, #(#impl_annotations),*)]
            impl<T> #query_ty
                #where_clause
            {
                #query
            }
            impl<T> Default for #query_ty {
                fn default() -> Self {
                    Self {
                        _grpc_client: ::std::marker::PhantomData,
                    }
                }
            }
            impl<T> Clone for #query_ty {
                fn clone(&self) -> Self {
                    Self {
                        _grpc_client: self._grpc_client,
                    }
                }
            }
            impl<T> ::std::fmt::Debug for #query_ty {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, #query_struct_debug)
                }
            }
        };
    }

    let mutation_ty = quote! { ::async_graphql::EmptyMutation };
    let mutation_init = mutation_ty.clone();

    let mut subscription_ty = quote! { ::async_graphql::EmptySubscription };
    let mut subscription_init = subscription_ty.clone();
    if !subscription.is_empty() {
        // The option to allow rename probably makes more sense.
        let graphql_subscription_name_str = if enable_federation {
            "Subscription".to_string()
        } else {
            format!("{}Subscription", service.name())
        };
        let graphql_subscription_name = format_ident!("{}Subscription", service.name());
        subscription_ty = quote! { #graphql_subscription_name<T> };
        subscription_init = quote! { <#subscription_ty>::default() };
        let subscription_struct_debug = format!("{} {{{{ ... }}}}", &graphql_subscription_name);
        subscription = quote! {
            pub struct #subscription_ty {
                _grpc_client: ::std::marker::PhantomData<#grpc_client_ty>,
            }
            #[::async_graphql::Subscription(name = #graphql_subscription_name_str, #(#impl_annotations),*)]
            impl<T> #subscription_ty
                #where_clause
            {
                #subscription
            }
            impl<T> Default for #subscription_ty {
                fn default() -> Self {
                    Self {
                        _grpc_client: ::std::marker::PhantomData,
                    }
                }
            }
            impl<T> Clone for #subscription_ty {
                fn clone(&self) -> Self {
                    Self {
                        _grpc_client: self._grpc_client,
                    }
                }
            }
            impl<T> ::std::fmt::Debug for #subscription_ty {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, #subscription_struct_debug)
                }
            }
        };
    }

    quote! {
        #grpc_client

        /// Generated gateway implementations.
        pub mod #graphql_mod {
            #![allow(unused_variables, dead_code, missing_docs)]
            use tonic::codegen::*;

            #(#service_doc)*
            pub type #graphql_schema_name<T> = ::async_graphql::Schema<
                #query_ty,
                #mutation_ty,
                #subscription_ty,
            >;

            /// Create a GraphQL schema builder.
            pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
                #query_ty,
                #mutation_ty,
                #subscription_ty,
            >
            #where_clause
            {
                ::async_graphql::Schema::build(
                    #query_init,
                    #mutation_init,
                    #subscription_init,
                )
            }

            #query
            #subscription
        }
    }
}

fn generate_methods<T: Service>(
    service: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    grpc_client_ty: &TokenStream,
) -> (TokenStream, TokenStream) {
    let mut query = TokenStream::new();
    let mut subscription = TokenStream::new();

    for method in service.methods() {
        let doc_comments = generate_doc_comments(method.comment());
        let mut annotations = GraphqlAnnotations::default();
        let method_annotations = annotations.visit(&mut parse_attrs(doc_comments.clone()), false);

        #[allow(clippy::single_match)]
        match (method.client_streaming(), method.server_streaming()) {
            (false, false) => {
                query.extend(doc_comments);
                if !method_annotations.is_empty() {
                    query.extend(quote! {
                        #[graphql(#(#method_annotations),*)]
                    });
                }
                query.extend(generate_unary(
                    method,
                    proto_path,
                    compile_well_known_types,
                    annotations,
                    grpc_client_ty,
                ));
            }
            (false, true) => {
                subscription.extend(doc_comments);
                if !method_annotations.is_empty() {
                    subscription.extend(quote! {
                        #[graphql(#(#method_annotations),*)]
                    });
                }
                subscription.extend(generate_server_streaming(
                    method,
                    proto_path,
                    compile_well_known_types,
                    annotations,
                    grpc_client_ty,
                ));
            }
            // TODO
            // (true, false) => generate_client_streaming(method, proto_path, annotations),
            // (true, true) => generate_streaming(method, proto_path, annotations),
            _ => panic!("only unary RPC and server streaming RPC are supported"),
        };
    }

    (query, subscription)
}

fn generate_request_arg(
    grpc_request: TokenStream,
    graphql_request: TokenStream,
) -> (TokenStream, TokenStream) {
    let arg = if grpc_request.to_string() == "()" {
        TokenStream::new()
    } else {
        quote! { request: #graphql_request }
    };

    let pass = if arg.is_empty() {
        quote! { () }
    } else {
        quote! { <#grpc_request>::from(request) }
    };

    (arg, pass)
}

fn generate_response_ret(grpc_response: TokenStream, graphql_response: TokenStream) -> TokenStream {
    if grpc_response.to_string() == "()" {
        quote! { ::proto_graphql::EmptyGraphQl }
    } else {
        quote! { #graphql_response }
    }
}

fn graphql_path(grpc_ty: &TokenStream, input: bool) -> TokenStream {
    if grpc_ty.to_string() == "()" {
        return grpc_ty.clone();
    }

    let mut path = syn::parse2::<syn::Path>(grpc_ty.clone()).unwrap();
    if will_rename(&path) {
        let ident = &mut path.segments.last_mut().unwrap().ident;
        if input {
            *ident = format_ident!("{}GraphQlInput", ident);
        } else {
            *ident = format_ident!("{}GraphQl", ident);
        }
        return path.to_token_stream();
    }
    grpc_ty.clone()
}

struct GraphqlRequestResponse {
    name: Ident,
    graphql_response_ty: TokenStream,
    request_arg: TokenStream,
    request_pass: TokenStream,
    output: TokenStream,
    prepare: TokenStream,
    ret: TokenStream,
}

fn graphql_request_response<T: Method>(
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    annotations: GraphqlAnnotations,
) -> GraphqlRequestResponse {
    let name = format_ident!("{}", method.name());
    let (grpc_request, grpc_response) =
        method.request_response_name(proto_path, compile_well_known_types);

    let graphql_request = graphql_path(&grpc_request, true);
    let graphql_response = graphql_path(&grpc_response, false);

    let (mut request_arg, request_pass) = generate_request_arg(grpc_request, graphql_request);
    let graphql_response_ty = generate_response_ret(grpc_response, graphql_response);
    let mut output = graphql_response_ty.clone();
    let mut prepare = quote! {};
    let mut ret = quote! { response };

    if let Some((input_ty, var)) = annotations.inputs {
        if let syn::FnArg::Typed(syn::PatType {
            pat,
            ty: request_ty,
            ..
        }) = syn::parse2(request_arg).unwrap()
        {
            prepare = quote! {
                let #pat = #request_ty { #(#var),* };
            };
            let inputs = input_ty.iter().zip(var).map(|(ty, var)| {
                quote! { #var: #ty, }
            });
            request_arg = quote! { #(#inputs)* };
        } else {
            unreachable!()
        }
    }
    if let Some((output_ty, var)) = annotations.output {
        output = quote! { #output_ty };
        ret = quote! { #ret.#var };
    }

    GraphqlRequestResponse {
        name,
        graphql_response_ty,
        request_arg,
        request_pass,
        output,
        prepare,
        ret,
    }
}

fn generate_unary<T: Method>(
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    annotations: GraphqlAnnotations,
    grpc_client_ty: &TokenStream,
) -> TokenStream {
    let GraphqlRequestResponse {
        name,
        graphql_response_ty,
        request_arg,
        request_pass,
        output,
        prepare,
        ret,
    } = graphql_request_response(method, proto_path, compile_well_known_types, annotations);

    quote! {
        pub async fn #name(
            &self,
            ctx: &::async_graphql::Context<'_>,
            #request_arg
        ) -> ::async_graphql::Result<#output> {
            #prepare
            let mut grpc_client = ctx.data::<#grpc_client_ty>()?.clone();
            let response = grpc_client
                .#name(#request_pass)
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <#graphql_response_ty>::from(response.into_inner());
            Ok(#ret)
        }
    }
}

fn generate_server_streaming<T: Method>(
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    annotations: GraphqlAnnotations,
    grpc_client_ty: &TokenStream,
) -> TokenStream {
    let GraphqlRequestResponse {
        name,
        graphql_response_ty,
        request_arg,
        request_pass,
        output,
        prepare,
        // TODO
        ret: _,
    } = graphql_request_response(method, proto_path, compile_well_known_types, annotations);

    quote! {
        pub async fn #name(
            &self,
            ctx: &::async_graphql::Context<'_>,
            #request_arg
        ) -> ::async_graphql::Result<impl ::proto_graphql::futures_util::stream::Stream<
            Item = ::async_graphql::Result<#output>,
        > + '_> {
            #prepare
            let mut grpc_client = ctx.data::<#grpc_client_ty>()?.clone();
            let response = grpc_client
                .#name(#request_pass)
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            Ok(::proto_graphql::futures_util::stream::StreamExt::map(
                response.into_inner(),
                |res| match res {
                    ::std::result::Result::Ok(ok) => {
                        ::std::result::Result::Ok(<#graphql_response_ty>::from(ok))
                    }
                    ::std::result::Result::Err(e) => {
                        ::std::result::Result::Err(::async_graphql::Error::new(e.to_string()))
                    }
                },
            ))
        }
    }
}

// TODO
// fn generate_client_streaming<T: Method>(
//     method: &T,
//     proto_path: &str,
//     path: String,
// ) -> TokenStream {
//     let codec_name = syn::parse_str::<syn::Path>(T::CODEC_PATH).unwrap();
//     let ident = format_ident!("{}", method.name());
//
//     let (request, response) = method.request_response_name(proto_path);
//     cx.request_types.push(request.clone());
//
//     quote! {
//         pub async fn #ident(
//             &mut self,
//             request: impl tonic::IntoStreamingRequest<Message = #request>
//         ) -> Result<tonic::Response<#response>, tonic::Status> {
//         }
//     }
// }

// TODO
// fn generate_streaming<T: Method>(
//     method: &T,
//     proto_path: &str,
//     path: String,
// ) -> TokenStream {
//     let codec_name = syn::parse_str::<syn::Path>(T::CODEC_PATH).unwrap();
//     let ident = format_ident!("{}", method.name());
//
//     let (request, response) = method.request_response_name(proto_path);
//     cx.request_types.push(request.clone());
//
//     quote! {
//         pub async fn #ident(
//             &mut self,
//             request: impl tonic::IntoStreamingRequest<Message = #request>
//         ) -> Result<tonic::Response<tonic::codec::Streaming<#response>>, tonic::Status> {
//         }
//     }
// }
