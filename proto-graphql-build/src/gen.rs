use std::collections::{BTreeMap, BTreeSet};

use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_quote,
    visit_mut::{self, VisitMut},
    PathSegment,
};

#[derive(Default)]
pub(crate) struct FileVisitor {
    pub(crate) module: Vec<syn::PathSegment>,
}

impl FileVisitor {
    fn visit_items(&mut self, items: &mut Vec<syn::Item>) {
        let mut self_items: Vec<syn::Item> = vec![];

        for item in &mut *items {
            match item {
                syn::Item::Struct(item) => {
                    let mut item = item.clone();
                    if is_proto(&mut item.attrs) {
                        generate_struct(&mut self_items, &self.module, item);
                    }
                }
                syn::Item::Enum(item) => {
                    let mut item = item.clone();
                    if is_proto(&mut item.attrs) {
                        let is_enum = item
                            .variants
                            .iter()
                            .all(|variant| variant.fields.is_empty());
                        if is_enum {
                            generate_enum(&mut self_items, &self.module, item);
                        } else {
                            generate_union(&mut self_items, &self.module, item);
                        }
                    }
                }
                syn::Item::Mod(item_mod) => {
                    self.module.push(item_mod.ident.clone().into());
                    visit_mut::visit_item_mut(self, item);
                    self.module.pop();
                    continue;
                }
                _ => {}
            }
            visit_mut::visit_item_mut(self, item);
        }

        items.append(&mut self_items);
    }
}

impl VisitMut for FileVisitor {
    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        if let Some((_, items)) = &mut item.content {
            self.visit_items(items);
        }
    }

    fn visit_file_mut(&mut self, item: &mut syn::File) {
        self.visit_items(&mut item.items);
    }
}

#[derive(Default)]
struct TypeVisitor {
    input: bool,
}

impl VisitMut for TypeVisitor {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if self.input {
            match ty {
                syn::Type::Tuple(t) if t.elems.is_empty() => {
                    *ty = syn::parse_quote!(::proto_graphql::EmptyGraphQlInput);
                    return;
                }
                _ => {}
            }
        }
        visit_mut::visit_type_mut(self, ty);
    }

    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        visit_mut::visit_path_mut(self, path);

        map_to_json(&mut *path);

        if will_rename(path) {
            let ident = &mut path.segments.last_mut().unwrap().ident;
            if self.input {
                *ident = format_ident!("{}GraphQlInput", ident);
            } else {
                *ident = format_ident!("{}GraphQl", ident);
            }
        }
    }
}

fn needs_wrap_in_json(path: &syn::Path) -> bool {
    path.segments[0].ident == "prost"
        && path.segments[1].ident == "alloc"
        && path.segments.last().unwrap().ident == "BTreeMap"
        || path.segments[0].ident == "std" && path.segments.last().unwrap().ident == "HashMap"
}

fn map_to_json(path: &mut syn::Path) {
    if needs_wrap_in_json(path) {
        *path = parse_quote!(::async_graphql::Json<#path>);
    }
}

// `graphql: ...` annotations
#[derive(Debug, Default)]
pub(crate) struct GraphqlAnnotations {
    // TODO: support compound primary keys.
    // https://www.apollographql.com/docs/federation/entities
    // (type, variable)
    pub(crate) inputs: Option<(Vec<TokenStream>, Vec<Ident>)>,
    // (type, variable)
    pub(crate) output: Option<(TokenStream, Ident)>,
    pub(crate) complex: bool,
    pub(crate) extends: bool,
    pub(crate) external: bool,
    pub(crate) entity: bool,
    pub(crate) skip: bool,
}

mod kw {
    syn::custom_keyword!(returns);
    syn::custom_keyword!(optional);
    syn::custom_keyword!(repeated);
}

#[derive(Debug)]
pub(crate) struct GraphQlTy {
    ty: TokenStream,
    variable: Option<Ident>,
}

fn parse_proto_ty(
    input: ParseStream<'_>,
    is_input: bool,
    path_prefix: &str,
) -> syn::Result<GraphQlTy> {
    let ty_suffix = if is_input { "GraphQlInput" } else { "GraphQl" };

    let ty = if input.peek(kw::optional) {
        input.parse::<kw::optional>()?;
        // TODO: support path, like `google.protobuf.Empty`
        let ty_str = &input.parse::<Ident>()?.to_string();
        if let Some(ty) = proto_primitive(ty_str) {
            quote! { ::core::option::Option<#ty> }
        } else {
            let ty: TokenStream =
                format!("{}{}{}", path_prefix, ty_str.to_camel_case(), ty_suffix).parse()?;
            quote! { ::core::option::Option<#ty> }
        }
    } else if input.peek(kw::repeated) {
        input.parse::<kw::repeated>()?;
        // TODO: support path, like `google.protobuf.Empty`
        let ty_str = &input.parse::<Ident>()?.to_string();
        if let Some(ty) = proto_primitive(ty_str) {
            quote! { ::prost::alloc::vec::Vec<#ty> }
        } else {
            let ty: TokenStream =
                format!("{}{}{}", path_prefix, ty_str.to_camel_case(), ty_suffix).parse()?;
            quote! { ::prost::alloc::vec::Vec<#ty> }
        }
    } else {
        // TODO: support path, like `google.protobuf.Empty`
        let ty_str = &input.parse::<Ident>()?.to_string();
        if let Some(ty) = proto_primitive(ty_str) {
            ty
        } else {
            format!("{}{}{}", path_prefix, ty_str.to_camel_case(), ty_suffix)
                .parse::<TokenStream>()?
        }
    };

    let variable = input.parse()?;

    Ok(GraphQlTy { ty, variable })
}

fn parse_pair(s: &str, is_input: bool) -> (TokenStream, Ident) {
    struct GraphQlRpcOutput(GraphQlTy);
    impl Parse for GraphQlRpcOutput {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            parse_proto_ty(input, false, "super::").map(Self)
        }
    }
    struct GraphQlRpcInput(GraphQlTy);
    impl Parse for GraphQlRpcInput {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            parse_proto_ty(input, true, "super::").map(Self)
        }
    }

    if is_input {
        let GraphQlRpcInput(GraphQlTy { ty, variable }) = syn::parse_str(s).unwrap();
        (ty, variable.expect("needs variable name"))
    } else {
        let GraphQlRpcOutput(GraphQlTy { ty, variable }) = syn::parse_str(s).unwrap();
        (ty, variable.expect("needs variable name"))
    }
}

impl GraphqlAnnotations {
    pub(crate) fn visit(
        &mut self,
        attrs: &mut Vec<syn::Attribute>,
        is_input: bool,
    ) -> Vec<TokenStream> {
        let mut res = vec![];
        let mut graphql_doc_indices = BTreeSet::new();
        'outer: for (i, attr) in attrs.iter().enumerate() {
            if attr.path.is_ident("doc") {
                if let Ok(syn::Meta::NameValue(m)) = attr.parse_meta() {
                    if let syn::Lit::Str(s) = m.lit {
                        for s in s.value().split('\n').map(str::trim) {
                            if let Some(s) = s.strip_prefix("graphql:") {
                                graphql_doc_indices.insert(i);
                                for s in s.split(',').map(str::trim) {
                                    if let Some(s) = s.strip_prefix("inputs") {
                                        if (s.starts_with('(') || s.starts_with(" ("))
                                            && s.ends_with(')')
                                        {
                                            let s = &s[..s.len() - 1];
                                            let s = s
                                                .strip_prefix('(')
                                                .unwrap_or_else(|| s.strip_prefix(" (").unwrap());
                                            let mut tys = vec![];
                                            let mut vars = vec![];
                                            for s in s.split(',') {
                                                let (ty, var) = parse_pair(s, true);
                                                tys.push(ty);
                                                vars.push(var);
                                            }
                                            self.inputs = Some((tys, vars));
                                            continue;
                                        }
                                        panic!("invalid inputs {:?}", s);
                                    }
                                    if let Some(s) = s.strip_prefix("output") {
                                        if (s.starts_with('(') || s.starts_with(" ("))
                                            && s.ends_with(')')
                                        {
                                            let s = &s[..s.len() - 1];
                                            let s = s
                                                .strip_prefix('(')
                                                .unwrap_or_else(|| s.strip_prefix(" (").unwrap());
                                            let (ty, var) = parse_pair(s, false);
                                            self.output = Some((ty, var));
                                            continue;
                                        }
                                        panic!("invalid output {:?}", s);
                                    }
                                    match s {
                                        "complex" => {
                                            if is_input {
                                                continue;
                                            }
                                            self.complex = true;
                                        }
                                        "extends" => {
                                            if is_input {
                                                continue;
                                            }
                                            self.extends = true;
                                        }
                                        "external" => {
                                            if is_input {
                                                continue;
                                            }
                                            self.external = true;
                                        }
                                        "entity" => self.entity = true,
                                        "skip" => self.skip = true,
                                        _ => {
                                            panic!("unknown key {:?}", s)
                                        }
                                    }
                                    res.push(s.parse().unwrap());
                                }
                            }
                        }
                    }
                }
            } else if attr.path.is_ident("deprecated") {
                if is_input {
                    continue;
                }
                match attr.parse_meta() {
                    Ok(syn::Meta::List(m)) => {
                        for m in m.nested {
                            match m {
                                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                                    if m.path.is_ident("note") =>
                                {
                                    let lit = m.lit;
                                    res.push(quote! { deprecation = #lit });
                                    continue 'outer;
                                }
                                _ => {}
                            }
                        }
                        res.push(quote! { deprecation });
                    }
                    Ok(syn::Meta::Path(_)) => {
                        res.push(quote! { deprecation });
                    }
                    res => unreachable!("{:?}", res),
                }
            }
        }

        // TODO: remove all docs if it is extend type.
        // https://github.com/graphql/graphql-js/issues/2385
        for i in graphql_doc_indices.into_iter().rev() {
            attrs.remove(i);
        }
        res
    }
}

pub(crate) fn generate_struct(
    self_items: &mut Vec<syn::Item>,
    module: &[PathSegment],
    mut item: syn::ItemStruct,
) {
    assert!(matches!(item.fields, syn::Fields::Named(..)));
    let mut annotations = GraphqlAnnotations::default();

    let mut input_item = item.clone();
    let proto_name = item.ident.clone();
    item.ident = format_ident!("{}GraphQl", proto_name);
    input_item.ident = format_ident!("{}GraphQlInput", proto_name);
    let graphql_name = &item.ident;
    let graphql_input_name = &input_item.ident;

    let name = format!("{}{}", path_to_name(module), proto_name);
    let input_name = format!("{}{}Input", path_to_name(module), proto_name);

    let item_annotations = annotations.visit(&mut item.attrs, false);

    let derive_struct: syn::Attribute = parse_quote! {
        #[derive(::async_graphql::SimpleObject, ::proto_graphql::serde::Serialize,
            ::proto_graphql::serde::Deserialize)]
    };
    let derive_input_struct: syn::Attribute = parse_quote! {
        #[derive(::async_graphql::InputObject, ::proto_graphql::serde::Serialize,
            ::proto_graphql::serde::Deserialize)]
    };
    item.attrs.push(derive_struct);
    input_item.attrs.push(derive_input_struct);
    item.attrs.push(parse_quote! {
        #[serde(crate = "::proto_graphql::serde")]
    });
    input_item.attrs.push(parse_quote! {
        #[serde(crate = "::proto_graphql::serde")]
    });

    if item.fields.is_empty() {
        item.attrs
            .push(parse_quote!(#[graphql(name = #name, #(#item_annotations),*)]));
        input_item
            .attrs
            .push(parse_quote!(#[graphql(name = #input_name)]));

        // GraphQL doesn't allow empty type.
        // It seems using a dummy boolean field is a common workaround.
        // https://github.com/graphql/graphql-js/issues/937
        // https://github.com/graphql/graphql-spec/issues/568
        match (&mut item.fields, &mut input_item.fields) {
            (syn::Fields::Named(fields), syn::Fields::Named(input_fields)) => {
                fields.named.push(syn::Field {
                    attrs: vec![parse_quote! {
                        // Note that `#[skip(graphql)]` cannot be used here
                        // because GraphQL Object must have one or more fields.
                        #[graphql(name = "_noop", visible = false)]
                    }],
                    vis: syn::Visibility::Inherited,
                    ident: Some(format_ident!("_noop")),
                    ty: parse_quote!(::core::option::Option<bool>),
                    colon_token: Some(Default::default()),
                });
                input_fields.named.push(syn::Field {
                    attrs: vec![parse_quote! {
                        #[graphql(name = "_noop", visible = false, default)]
                    }],
                    vis: syn::Visibility::Inherited,
                    ident: Some(format_ident!("_noop")),
                    ty: parse_quote!(::core::option::Option<bool>),
                    colon_token: Some(Default::default()),
                });

                self_items.push(syn::Item::Verbatim(quote! {
                    #item
                    #input_item
                    #[allow(clippy::useless_conversion)]
                    impl From<#proto_name> for #graphql_name {
                        fn from(other: #proto_name) -> Self {
                            let #proto_name { } = other;
                            Self { _noop: None }
                        }
                    }
                    #[allow(clippy::useless_conversion)]
                    impl From<#graphql_name> for #proto_name {
                        fn from(other: #graphql_name) -> Self {
                            let #graphql_name { _noop } = other;
                            Self {}
                        }
                    }
                    #[allow(clippy::useless_conversion)]
                    impl From<#proto_name> for #graphql_input_name {
                        fn from(other: #proto_name) -> Self {
                            let #proto_name { } = other;
                            Self { _noop: None }
                        }
                    }
                    #[allow(clippy::useless_conversion)]
                    impl From<#graphql_input_name> for #proto_name {
                        fn from(other: #graphql_input_name) -> Self {
                            let #graphql_input_name { _noop } = other;
                            Self {}
                        }
                    }
                }));
            }
            _ => unreachable!(),
        }

        return;
    }

    item.attrs
        .push(parse_quote!(#[graphql(name = #name, #(#item_annotations),*)]));

    input_item
        .attrs
        .push(parse_quote!(#[graphql(name = #input_name)]));

    let mut prost_attrs = vec![];
    let mut pre_convert_proto = vec![];
    let mut convert_proto = vec![];
    let mut convert_graphql = vec![];
    let mut proto_bindings = vec![];
    let mut graphql_bindings = vec![];
    for (i, field) in item.fields.iter_mut().enumerate() {
        let mut enumeration = false;
        if let Some(prost) = find_remove(&mut field.attrs, "prost") {
            let prost = prost_attr(prost);
            if let Some(s) = prost.get("enumeration") {
                enumeration = true;
                if let syn::Lit::Str(s) = s {
                    replace_i32(&mut field.ty, syn::parse_str(&s.value()).unwrap());
                } else {
                    unreachable!()
                }
            }
            prost_attrs.push(prost);
        } else {
            prost_attrs.push(Default::default());
        }

        TypeVisitor::default().visit_type_mut(&mut field.ty);

        let convert = convert_field(&mut graphql_bindings, i, field, enumeration);
        convert_proto.push(convert.0);
        convert_graphql.push(convert.1);
        if enumeration {
            pre_convert_proto.push(convert.2);
        } else {
            proto_bindings.push(graphql_bindings.last().unwrap().clone())
        }

        let field_annotations = annotations.visit(&mut field.attrs, false);

        if !field_annotations.is_empty() {
            field
                .attrs
                .push(parse_quote!(#[graphql(#(#field_annotations),*)]));
        }
    }

    for (i, field) in input_item.fields.iter_mut().enumerate() {
        find_remove(&mut field.attrs, "prost");
        if let Some(syn::Lit::Str(s)) = prost_attrs[i].get("enumeration") {
            replace_i32(&mut field.ty, syn::parse_str(&s.value()).unwrap());
        }
        TypeVisitor { input: true }.visit_type_mut(&mut field.ty);
    }

    self_items.push(syn::Item::Verbatim(quote! {
        #item
        #input_item
        #[allow(clippy::useless_conversion)]
        impl From<#proto_name> for #graphql_name {
            fn from(other: #proto_name) -> Self {
                #(#pre_convert_proto)*
                let #proto_name { #(#proto_bindings)* .. } = other;
                Self { #(#convert_proto)* }
            }
        }
        #[allow(clippy::useless_conversion)]
        impl From<#graphql_name> for #proto_name {
            fn from(other: #graphql_name) -> Self {
                let #graphql_name { #(#graphql_bindings)* } = other;
                Self { #(#convert_graphql)* }
            }
        }
        #[allow(clippy::useless_conversion)]
        impl From<#proto_name> for #graphql_input_name {
            fn from(other: #proto_name) -> Self {
                #(#pre_convert_proto)*
                let #proto_name { #(#proto_bindings)* .. } = other;
                Self { #(#convert_proto)* }
            }
        }
        #[allow(clippy::useless_conversion)]
        impl From<#graphql_input_name> for #proto_name {
            fn from(other: #graphql_input_name) -> Self {
                let #graphql_input_name { #(#graphql_bindings)* } =
                    other;
                Self { #(#convert_graphql)* }
            }
        }
    }));
}

pub(crate) fn generate_union(
    self_items: &mut Vec<syn::Item>,
    module: &[PathSegment],
    mut item: syn::ItemEnum,
) {
    // GraphQL unions is similar to Rust enums with fields.
    let derive_union: syn::Attribute = parse_quote! {
        #[derive(::async_graphql::Union, ::proto_graphql::serde::Serialize,
            ::proto_graphql::serde::Deserialize)]
    };
    // TODO: https://github.com/async-graphql/async-graphql/issues/373#issuecomment-753761917
    let derive_input_struct: syn::Attribute = parse_quote! {
        #[derive(::async_graphql::InputObject, ::proto_graphql::serde::Serialize,
            ::proto_graphql::serde::Deserialize)]
    };

    let mut input_item = item.clone();
    let proto_name = item.ident.clone();
    item.ident = format_ident!("{}GraphQl", proto_name);
    input_item.ident = format_ident!("{}GraphQlInput", proto_name);
    let graphql_name = item.ident.clone();
    let graphql_input_name = input_item.ident.clone();

    item.attrs.push(derive_union);
    input_item.attrs.push(derive_input_struct);
    item.attrs.push(parse_quote! {
        #[serde(crate = "::proto_graphql::serde")]
    });
    input_item.attrs.push(parse_quote! {
        #[serde(crate = "::proto_graphql::serde")]
    });
    let name = format!("{}{}", path_to_name(module), proto_name);
    let input_name = format!("{}{}Input", path_to_name(module), proto_name);
    item.attrs.push(parse_quote!(#[graphql(name = #name)]));
    input_item
        .attrs
        .push(parse_quote!(#[graphql(name = #input_name)]));

    let mut fields = vec![];
    for variant in &mut item.variants {
        find_remove(&mut variant.attrs, "prost");
        for field in variant.fields.iter_mut() {
            find_remove(&mut field.attrs, "prost");
            TypeVisitor { input: false }.visit_type_mut(&mut field.ty);
            match &field.ty {
                syn::Type::Path(ty) => {
                    if !will_rename(&ty.path) {
                        let field_name =
                            format_ident!("{}", variant.ident.to_string().to_snake_case());
                        let name =
                            format!("{}{}{}", path_to_name(module), proto_name, variant.ident);
                        let attr: syn::Attribute = parse_quote!(#[graphql(name = #name)]);
                        let newtype_ident = format_ident!("{}{}", item.ident, variant.ident);
                        let ty = field.ty.clone();
                        field.ty = parse_quote!(#newtype_ident);
                        self_items.push(syn::Item::Verbatim(quote! {
                            #[derive(
                                Clone, PartialEq,
                                ::async_graphql::SimpleObject,
                                ::proto_graphql::serde::Serialize,
                                ::proto_graphql::serde::Deserialize
                            )]
                            #attr
                            pub struct #newtype_ident {
                                // TODO: what is a better field name?
                                #field_name: #ty,
                            }
                            #[allow(clippy::useless_conversion)]
                            impl From<#newtype_ident> for #ty {
                                fn from(other: #newtype_ident) -> Self {
                                    other.#field_name
                                }
                            }
                            #[allow(clippy::useless_conversion)]
                            impl From<#ty> for #newtype_ident {
                                fn from(other: #ty) -> Self {
                                    Self{ #field_name: other }
                                }
                            }
                        }));
                        continue;
                    }
                }
                syn::Type::Tuple(ty) if ty.elems.is_empty() => {
                    let variant_name = &variant.ident;
                    let name = format!("{}{}{}", path_to_name(module), proto_name, variant_name);
                    let attr: syn::Attribute = parse_quote!(#[graphql(name = #name)]);
                    let newtype_ident = format_ident!("{}{}", item.ident, variant_name);
                    let ty = field.ty.clone();
                    field.ty = parse_quote!(#newtype_ident);
                    self_items.push(syn::Item::Verbatim(quote! {
                        #[derive(
                            Clone, PartialEq,
                            ::async_graphql::SimpleObject,
                            ::proto_graphql::serde::Serialize,
                            ::proto_graphql::serde::Deserialize
                        )]
                        #attr
                        pub struct #newtype_ident {
                            #[graphql(name = "_noop", visible = false)]
                            _noop: ::core::option::Option<bool>,
                        }
                        #[allow(clippy::useless_conversion)]
                        impl From<#newtype_ident> for #ty {
                            fn from(_: #newtype_ident) -> Self {
                            }
                        }
                        #[allow(clippy::useless_conversion)]
                        impl From<#ty> for #newtype_ident {
                            fn from((): #ty) -> Self {
                                Self{ _noop: None }
                            }
                        }
                    }));
                    continue;
                }
                _ => {}
            }
            fields.push(field.ty.clone());
        }
    }

    {
        let mut fields_def = vec![];
        let mut field_names = vec![];
        for variant in input_item.variants.iter_mut() {
            let mut fields = vec![];
            find_remove(&mut variant.attrs, "prost");
            for field in variant.fields.iter_mut() {
                find_remove(&mut field.attrs, "prost");
                TypeVisitor { input: true }.visit_type_mut(&mut field.ty);
                fields.push(field.ty.clone());
            }
            let field_name = format_ident!("{}", variant.ident.to_string().to_snake_case());
            field_names.push(field_name.clone());
            fields_def.push(quote! {
                #field_name: ::core::option::Option<#(#fields)*>,
            })
        }
        let mut from_proto = vec![];
        let mut from_graphql = vec![];
        item.variants.iter().enumerate().for_each(|(i, variant)| {
            let mut field_names = field_names.clone();
            let field_name = field_names.remove(i);
            let ident = &variant.ident;
            match variant.fields {
                syn::Fields::Unnamed(_) => {
                    from_proto.push(quote! {
                        #proto_name::#ident(#field_name) => Self {
                            #field_name: ::core::option::Option::Some(#field_name.into()),
                            #(#field_names: ::core::option::Option::None),*
                        },
                    });
                    from_graphql.push(quote! {
                        #graphql_input_name {
                            #field_name: ::core::option::Option::Some(#field_name),
                            #(#field_names: ::core::option::Option::None),*
                        } => {
                            Self::#ident(#field_name.into())
                        }
                    });
                }
                _ => unreachable!(),
            }
        });
        let attrs = &input_item.attrs;
        self_items.push(syn::Item::Verbatim(quote! {
            #(#attrs)*
            pub struct #graphql_input_name {
                #(#fields_def)*
            }
            #[allow(clippy::useless_conversion)]
            impl From<#proto_name> for #graphql_input_name {
                fn from(other: #proto_name) -> Self {
                    match other {
                        #(#from_proto)*
                    }
                }
            }
            #[allow(clippy::useless_conversion)]
            impl From<#graphql_input_name> for #proto_name {
                fn from(other: #graphql_input_name) -> Self {
                    match other {
                        #(#from_graphql)*
                        // TODO: better error handling
                        _ => panic!("invalid input"),
                    }
                }
            }
        }));
    }

    let mut from_proto = vec![];
    let mut from_graphql = vec![];
    item.variants.iter().for_each(|variant| {
        let mut bindings = vec![];
        let mut convert_proto = vec![];
        let mut convert_graphql = vec![];
        variant.fields.iter().enumerate().for_each(|(j, field)| {
            let convert = convert_field(&mut bindings, j, field, false);
            convert_proto.push(convert.0);
            convert_graphql.push(convert.1);
        });

        let ident = &variant.ident;
        match variant.fields {
            syn::Fields::Named(_) => {
                from_proto.push(quote! {
                    #proto_name::#ident {
                        #(#bindings),*
                    } => Self::#ident {
                        #(#convert_proto)*
                    },
                });
                from_graphql.push(quote! {
                    #graphql_name::#ident {
                        #(#bindings),*
                    } => Self::#ident {
                        #(#convert_graphql)*
                    },
                });
            }
            syn::Fields::Unnamed(_) => {
                from_proto.push(quote! {
                    #proto_name::#ident(
                        #(#bindings),*
                    ) => Self::#ident(#(#convert_proto)*),
                });
                from_graphql.push(quote! {
                    #graphql_name::#ident(
                        #(#bindings),*
                    ) => Self::#ident(#(#convert_graphql)*),
                });
            }
            syn::Fields::Unit => {
                from_proto.push(quote! {
                    #proto_name::#ident => Self::#ident,
                });
                from_graphql.push(quote! {
                    #graphql_name::#ident => Self::#ident,
                });
            }
        }
    });

    self_items.push(syn::Item::Verbatim(quote! {
        #[allow(clippy::useless_conversion)]
        impl From<#proto_name> for #graphql_name {
            fn from(other: #proto_name) -> Self {
                match other {
                    #(#from_proto)*
                }
            }
        }
        #[allow(clippy::useless_conversion)]
        impl From<#graphql_name> for #proto_name {
            fn from(other: #graphql_name) -> Self {
                match other {
                    #(#from_graphql)*
                }
            }
        }
    }));

    self_items.push(item.into());
}

pub(crate) fn generate_enum(
    self_items: &mut Vec<syn::Item>,
    module: &[PathSegment],
    mut item: syn::ItemEnum,
) {
    // GraphQL enums is similar to Rust enums with no fields.
    let derive_enum: syn::Attribute = parse_quote! {
        #[derive(::async_graphql::Enum, ::proto_graphql::serde::Serialize,
            ::proto_graphql::serde::Deserialize)]
    };

    let mut input_item = item.clone();
    let proto_name = item.ident.clone();
    item.ident = format_ident!("{}GraphQl", proto_name);
    input_item.ident = format_ident!("{}GraphQlInput", proto_name);
    let graphql_name = item.ident.clone();
    let graphql_input_name = input_item.ident.clone();

    item.attrs.push(derive_enum.clone());
    input_item.attrs.push(derive_enum);
    item.attrs.push(parse_quote! {
        #[serde(crate = "::proto_graphql::serde")]
    });
    input_item.attrs.push(parse_quote! {
        #[serde(crate = "::proto_graphql::serde")]
    });
    let name = format!("{}{}", path_to_name(module), proto_name);
    let input_name = format!("{}{}Input", path_to_name(module), proto_name);
    item.attrs.push(parse_quote!(#[graphql(name = #name)]));
    input_item
        .attrs
        .push(parse_quote!(#[graphql(name = #input_name)]));

    for variant in &mut item.variants {
        find_remove(&mut variant.attrs, "prost");
        for field in variant.fields.iter_mut() {
            find_remove(&mut field.attrs, "prost");
            TypeVisitor { input: false }.visit_type_mut(&mut field.ty);
        }
    }

    let mut from_proto = vec![];
    let mut from_graphql = vec![];
    item.variants.iter().for_each(|variant| {
        let mut bindings = vec![];
        let mut convert_proto = vec![];
        let mut convert_graphql = vec![];
        variant.fields.iter().enumerate().for_each(|(j, field)| {
            let convert = convert_field(&mut bindings, j, field, false);
            convert_proto.push(convert.0);
            convert_graphql.push(convert.1);
        });

        let ident = &variant.ident;
        match variant.fields {
            syn::Fields::Named(_) => {
                from_proto.push(quote! {
                    #proto_name::#ident {
                        #(#bindings),*
                    } => Self::#ident {
                        #(#convert_proto)*
                    },
                });
                from_graphql.push(quote! {
                    #graphql_name::#ident {
                        #(#bindings),*
                    } => Self::#ident {
                        #(#convert_graphql)*
                    },
                });
            }
            syn::Fields::Unnamed(_) => {
                from_proto.push(quote! {
                    #proto_name::#ident(
                        #(#bindings),*
                    ) => Self::#ident(#(#convert_proto)*),
                });
                from_graphql.push(quote! {
                    #graphql_name::#ident(
                        #(#bindings),*
                    ) => Self::#ident(#(#convert_graphql)*),
                });
            }
            syn::Fields::Unit => {
                from_proto.push(quote! {
                    #proto_name::#ident => Self::#ident,
                });
                from_graphql.push(quote! {
                    #graphql_name::#ident => Self::#ident,
                });
            }
        }
    });

    self_items.push(syn::Item::Verbatim(quote! {
        #[allow(clippy::useless_conversion)]
        impl From<#proto_name> for #graphql_name {
            fn from(other: #proto_name) -> Self {
                match other {
                    #(#from_proto)*
                }
            }
        }
        #[allow(clippy::useless_conversion)]
        impl From<#graphql_name> for #proto_name {
            fn from(other: #graphql_name) -> Self {
                match other {
                    #(#from_graphql)*
                }
            }
        }
    }));

    self_items.push(item.into());
    self_items.push(syn::Item::Verbatim(quote! {
        pub use self::#graphql_name as #graphql_input_name;
    }));
}

pub(crate) fn convert_field(
    bindings: &mut Vec<TokenStream>,
    index: usize,
    syn::Field {
        ident,
        colon_token,
        ty,
        ..
    }: &syn::Field,
    enumeration: bool,
) -> (TokenStream, TokenStream, TokenStream) {
    let binding = ident
        .clone()
        .unwrap_or_else(|| format_ident!("_binding{}", index));
    bindings.push(quote!(#binding,));
    if let syn::Type::Path(ty) = ty {
        if ty.path.segments[0].ident == "core" && ty.path.segments.last().unwrap().ident == "Option"
        {
            if enumeration {
                return (
                    quote! {
                        #ident #colon_token #binding.map(Into::into),
                    },
                    quote! {
                        #ident #colon_token #binding.map(|b| b as i32),
                    },
                    quote! {
                        let #binding = if other.#binding.is_some() { Some(other.#binding()) } else { None };
                    },
                );
            }

            let convert = quote! {
                #ident #colon_token #binding.map(Into::into),
            };
            return (convert.clone(), convert, quote!());
        }
        if ty.path.segments[0].ident == "prost"
            && ty.path.segments[1].ident == "alloc"
            && ty.path.segments.last().unwrap().ident == "Vec"
        {
            if enumeration {
                return (
                    quote! {
                        #binding,
                    },
                    quote! {
                        #ident #colon_token #binding.into_iter().map(|b| b as i32).collect(),
                    },
                    quote! {
                        let #binding = other.#binding().map(Into::into).collect();
                    },
                );
            }

            let convert = quote! {
                #ident #colon_token #binding
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            };
            return (convert.clone(), convert, quote!());
        }
        if ty.path.segments[0].ident == "async_graphql"
            && ty.path.segments.last().unwrap().ident == "Json"
        {
            return (
                quote! {
                    #ident #colon_token ::async_graphql::Json(
                        #binding
                            .into_iter()
                            .map(|(k, v)| (k, v.into()))
                            .collect()
                    ),
                },
                quote! {
                    #ident #colon_token #binding.0
                        .into_iter()
                        .map(|(k, v)| (k, v.into()))
                        .collect(),
                },
                quote!(),
            );
        }
        if enumeration {
            return (
                quote! {
                    #ident #colon_token #binding.into(),
                },
                quote! {
                    #ident #colon_token #binding as _,
                },
                quote! {
                    let #binding = other.#binding();
                },
            );
        }
    }

    let convert = quote! {
        #ident #colon_token #binding.into(),
    };
    (convert.clone(), convert, quote!())
}

pub(crate) fn is_proto(attrs: &mut Vec<syn::Attribute>) -> bool {
    for (i, meta) in attrs
        .iter_mut()
        .enumerate()
        .filter_map(|(i, attr)| attr.parse_meta().ok().map(|meta| (i, meta)))
    {
        if let syn::Meta::List(list) = meta {
            if list.path.is_ident("derive") {
                for (j, meta) in list.nested.iter().enumerate() {
                    match meta {
                        syn::NestedMeta::Meta(syn::Meta::Path(path))
                        | syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList { path, .. }))
                            if path.segments[0].ident == "prost" =>
                        {
                            let mut tmp: Vec<_> = list.nested.iter().cloned().collect();
                            tmp.remove(j);
                            attrs[i] = parse_quote!(#[derive(#(#tmp,)*)]);
                            return true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    false
}

pub(crate) fn prost_attr(attr: syn::Attribute) -> BTreeMap<String, syn::Lit> {
    let mut map = BTreeMap::new();
    let meta = attr.parse_meta().unwrap();
    let list = if let syn::Meta::List(list) = meta {
        list
    } else {
        unreachable!()
    };
    assert!(list.path.is_ident("prost"));
    for meta in list.nested {
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) = meta {
            map.insert(meta.path.get_ident().unwrap().to_string(), meta.lit);
        }
    }
    map
}

pub(crate) fn find_remove(attrs: &mut Vec<syn::Attribute>, ident: &str) -> Option<syn::Attribute> {
    attrs
        .iter()
        .position(|attr| attr.path.is_ident(ident))
        .map(|i| attrs.remove(i))
}

pub(crate) fn is_rust_primitive(s: &str) -> bool {
    matches!(
        s,
        "isize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "usize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "f32"
            | "f64"
            | "bool"
    )
}

pub(crate) fn proto_primitive(s: &str) -> Option<TokenStream> {
    match s {
        "double" => Some(quote! { f64 }),
        "float" => Some(quote! { f32 }),
        "int32" => Some(quote! { i32 }),
        "int64" => Some(quote! { i64 }),
        "uint32" => Some(quote! { u32 }),
        "uint64" => Some(quote! { u64 }),
        "sint32" => Some(quote! { i32 }),
        "sint64" => Some(quote! { i64 }),
        "fixed32" => Some(quote! { u32 }),
        "fixed64" => Some(quote! { u64 }),
        "sfixed32" => Some(quote! { i32 }),
        "sfixed64" => Some(quote! { i64 }),
        "bool" => Some(quote! { bool }),
        "string" => Some(quote! { ::prost::alloc::string::String }),
        "bytes" => Some(quote! { ::prost::alloc::vec::Vec<u8> }),
        _ => None,
    }
}

pub(crate) fn will_rename(path: &syn::Path) -> bool {
    match path.get_ident().map(ToString::to_string).as_deref() {
        Some(s) if is_rust_primitive(s) => {
            return false;
        }
        _ => {}
    }
    path.segments[0].ident != "prost"
        && path.segments[0].ident != "async_graphql"
        && path.segments[0].ident != "core"
        && path.segments[0].ident != "std"
}

pub(crate) fn path_to_name<'a>(segments: impl IntoIterator<Item = &'a syn::PathSegment>) -> String {
    let mut name = String::new();
    for seg in segments {
        name.push_str(&seg.ident.unraw().to_string().to_camel_case());
    }
    name
}

pub(crate) fn replace_i32(ty: &mut syn::Type, with: syn::TypePath) {
    struct Visitor(Option<syn::TypePath>);

    impl VisitMut for Visitor {
        fn visit_type_path_mut(&mut self, ty: &mut syn::TypePath) {
            if ty.path.is_ident("i32") {
                *ty = self.0.take().unwrap();
                return;
            }
            visit_mut::visit_type_path_mut(self, ty);
        }
    }

    Visitor(Some(with)).visit_type_mut(ty);
}
