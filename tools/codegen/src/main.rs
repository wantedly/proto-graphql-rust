use std::path::PathBuf;

use anyhow::{Context as _, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::visit_mut::{self, VisitMut};

mod gen {
    include!("../../../proto-graphql-build/src/gen.rs");
}

use gen::*;

const PROST_TAG: &str = "v0.7.0";

fn main() -> Result<()> {
    let mut root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root_dir.pop(); // codegen
    root_dir.pop(); // tools

    let out_dir = root_dir.join("proto-graphql/src/generated");

    let base_url = format!(
        "https://raw.githubusercontent.com/danburkert/prost/{}/prost-types/src/",
        PROST_TAG
    );
    for file in &["compiler.rs", "protobuf.rs"] {
        let url = format!("{}{}", base_url, file);
        let text = reqwest::blocking::get(&url)?.text()?;
        fs::write(out_dir.join(file), text)?;
    }

    for entry in fs::read_dir(&out_dir).unwrap() {
        let path = &entry.unwrap().path();
        if !path.file_name().unwrap().to_str().unwrap().ends_with(".rs") {
            continue;
        }

        let file = fs::read_to_string(path)?;
        let mut file = syn::parse_str(&file).with_context(|| {
            format!(
                "failed to parse {} as valid rust source file",
                path.display()
            )
        })?;

        let mut visitor = FileVisitor::default();
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        if file_stem != "protobuf" {
            visitor.module.push(format_ident!("{}", file_stem).into());
        }
        visitor.visit_file_mut(&mut file);

        fs::write(path, file.to_token_stream().to_string())?;
    }

    proto_graphql_build::fmt(out_dir.to_str().context("expected utf8 out_dir")?);

    Ok(())
}

#[derive(Default)]
struct FileVisitor {
    module: Vec<syn::PathSegment>,
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

            *item = syn::Item::Verbatim(TokenStream::new());
        }

        let path = &self.module;
        self_items.push(syn::parse_quote! {
            #[allow(unused_imports)]
            pub use ::prost_types::#(#path::)**;
        });
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
