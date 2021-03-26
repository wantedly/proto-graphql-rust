use std::{
    io::{self, Write},
    process::{self, Command},
};

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream};
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream};
use tonic_build::{Method, Service};

#[doc(no_inline)]
pub use prost_build::Config;

mod prost;
pub use prost::{compile_protos, configure, Builder};

mod client;

mod gen;
use gen::{will_rename, GraphqlAnnotations};

/// Format files under the out_dir with rustfmt
///
/// Unlike `tonic_build::fmt`, this pass `--config normalize_doc_attributes=true` to rustfmt
pub fn fmt(out_dir: &str) {
    let dir = fs::read_dir(out_dir).unwrap();

    for entry in dir {
        let file = entry.unwrap().file_name().into_string().unwrap();
        if !file.ends_with(".rs") {
            continue;
        }
        let result = Command::new("rustfmt")
            .arg("--emit")
            .arg("files")
            .arg("--edition")
            .arg("2018")
            .arg("--config")
            .arg("normalize_doc_attributes=true")
            .arg(format!("{}/{}", out_dir, file))
            .output();

        match result {
            Err(e) => {
                eprintln!("error running rustfmt: {:?}", e);
                process::exit(1)
            }
            Ok(output) => {
                if !output.status.success() {
                    io::stderr().write_all(&output.stderr).unwrap();
                    process::exit(output.status.code().unwrap_or(1))
                }
            }
        }
    }
}

// Generate a singular line of a doc comment
fn generate_doc_comment<S: AsRef<str>>(comment: S) -> TokenStream {
    let mut doc_stream = TokenStream::new();

    doc_stream.append(ident("doc"));
    doc_stream.append(Punct::new('=', Spacing::Alone));
    doc_stream.append(Literal::string(comment.as_ref()));

    let group = Group::new(Delimiter::Bracket, doc_stream);

    let mut stream = TokenStream::new();
    stream.append(Punct::new('#', Spacing::Alone));
    stream.append(group);
    stream
}

// Generate a larger doc comment composed of many lines of doc comments
fn generate_doc_comments<T: AsRef<str>>(comments: &[T]) -> TokenStream {
    let mut stream = TokenStream::new();

    for comment in comments {
        stream.extend(generate_doc_comment(comment));
    }

    stream
}

fn parse_attrs(tokens: TokenStream) -> Vec<syn::Attribute> {
    struct Attributes(Vec<syn::Attribute>);

    impl Parse for Attributes {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            syn::Attribute::parse_outer(input).map(Self)
        }
    }

    syn::parse2::<Attributes>(tokens).unwrap().0
}

fn ident(s: &str) -> Ident {
    Ident::new(s, proc_macro2::Span::call_site())
}
