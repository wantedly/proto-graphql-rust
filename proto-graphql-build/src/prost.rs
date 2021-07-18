use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

use proc_macro2::TokenStream;
use prost_build::Config;
use quote::ToTokens;
use syn::visit_mut::VisitMut;

use super::{client, gen::*};

/// Configure `proto-graphql` code generation.
///
/// Use [`compile_protos`] instead if you don't need to tweak anything.
pub fn configure() -> Builder {
    Builder {
        build_client: true,
        build_server: true,
        out_dir: None,
        extern_path: Vec::new(),
        field_attributes: Vec::new(),
        type_attributes: Vec::new(),
        proto_path: "super".to_string(),
        format: true,
        emit_package: true,
        compile_well_known_types: false,
        enable_federation: false,
    }
}

/// Simple `.proto` compiling. Use [`configure`] instead if you need more options.
///
/// The include directory will be the parent folder of the specified path.
/// The package name will be the filename without the extension.
pub fn compile_protos(proto: impl AsRef<Path>) -> Result<()> {
    let proto_path: &Path = proto.as_ref();

    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .context("proto file should reside in a directory")?;

    self::configure().compile(&[proto_path], &[proto_dir])?;

    Ok(())
}

struct ServiceGenerator {
    builder: Builder,
    clients: TokenStream,
    servers: TokenStream,
}

impl ServiceGenerator {
    fn new(builder: Builder) -> Self {
        ServiceGenerator {
            builder,
            clients: TokenStream::default(),
            servers: TokenStream::default(),
        }
    }
}

impl prost_build::ServiceGenerator for ServiceGenerator {
    fn generate(&mut self, service: prost_build::Service, _buf: &mut String) {
        if self.builder.build_server {
            let server = tonic_build::server::generate(
                &service,
                self.builder.emit_package,
                &self.builder.proto_path,
                self.builder.compile_well_known_types,
                &tonic_build::Attributes::default(),
            );
            self.servers.extend(server);
        }

        if self.builder.build_client {
            let client = client::generate(
                &service,
                self.builder.emit_package,
                &self.builder.proto_path,
                self.builder.enable_federation,
                self.builder.compile_well_known_types,
            );
            self.clients.extend(client);
        }
    }

    fn finalize(&mut self, buf: &mut String) {
        if self.builder.build_client && !self.clients.is_empty() {
            let clients = &self.clients;

            let client_service = quote::quote! {
                #clients
            };

            let code = format!("{}", client_service);
            buf.push_str(&code);

            self.clients = TokenStream::default();
        }

        if self.builder.build_server && !self.servers.is_empty() {
            let servers = &self.servers;

            let server_service = quote::quote! {
                #servers
            };

            let code = format!("{}", server_service);
            buf.push_str(&code);

            self.servers = TokenStream::default();
        }
    }
}

/// Service generator builder.
#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) build_client: bool,
    pub(crate) build_server: bool,
    pub(crate) extern_path: Vec<(String, String)>,
    pub(crate) field_attributes: Vec<(String, String)>,
    pub(crate) type_attributes: Vec<(String, String)>,
    pub(crate) proto_path: String,
    pub(crate) emit_package: bool,
    pub(crate) compile_well_known_types: bool,

    enable_federation: bool,

    out_dir: Option<PathBuf>,
    format: bool,
}

impl Builder {
    /// Enable or disable federation.
    pub fn federation(mut self, enable: bool) -> Self {
        self.enable_federation = enable;
        self
    }

    /// Enable or disable gRPC client code generation.
    pub fn build_client(mut self, enable: bool) -> Self {
        self.build_client = enable;
        self
    }

    /// Enable or disable gRPC server code generation.
    pub fn build_server(mut self, enable: bool) -> Self {
        self.build_server = enable;
        self
    }

    /// Enable the output to be formatted by rustfmt.
    pub fn format(mut self, run: bool) -> Self {
        self.format = run;
        self
    }

    /// Set the output directory to generate code to.
    ///
    /// Defaults to the `OUT_DIR` environment variable.
    pub fn out_dir(mut self, out_dir: impl AsRef<Path>) -> Self {
        self.out_dir = Some(out_dir.as_ref().to_path_buf());
        self
    }

    /// Declare externally provided Protobuf package or type.
    ///
    /// Passed directly to `prost_build::Config.extern_path`.
    /// Note that both the Protobuf path and the rust package paths should both be fully qualified.
    /// i.e. Protobuf paths should start with "." and rust paths should start with "::"
    pub fn extern_path(mut self, proto_path: impl AsRef<str>, rust_path: impl AsRef<str>) -> Self {
        self.extern_path.push((
            proto_path.as_ref().to_string(),
            rust_path.as_ref().to_string(),
        ));
        self
    }

    /// Add additional attribute to matched messages, enums, and one-offs.
    ///
    /// Passed directly to `prost_build::Config.field_attribute`.
    pub fn field_attribute<P: AsRef<str>, A: AsRef<str>>(mut self, path: P, attribute: A) -> Self {
        self.field_attributes
            .push((path.as_ref().to_string(), attribute.as_ref().to_string()));
        self
    }

    /// Add additional attribute to matched messages, enums, and one-offs.
    ///
    /// Passed directly to `prost_build::Config.type_attribute`.
    pub fn type_attribute<P: AsRef<str>, A: AsRef<str>>(mut self, path: P, attribute: A) -> Self {
        self.type_attributes
            .push((path.as_ref().to_string(), attribute.as_ref().to_string()));
        self
    }

    /// Set the path to where tonic will search for the Request/Response proto structs
    /// live relative to the module where you call `include_proto!`.
    ///
    /// This defaults to `super` since tonic will generate code in a module.
    pub fn proto_path(mut self, proto_path: impl AsRef<str>) -> Self {
        self.proto_path = proto_path.as_ref().to_string();
        self
    }

    /// Emits GRPC endpoints with no attached package. Effectively ignores protofile package declaration from grpc context.
    ///
    /// This effectively sets prost's exported package to an empty string.
    pub fn disable_package_emission(mut self) -> Self {
        self.emit_package = false;
        self
    }

    /// Enable or disable directing Prost to compile well-known protobuf types instead
    /// of using the already-compiled versions available in the `prost-types` crate.
    ///
    /// This defaults to `false`.
    pub fn compile_well_known_types(mut self, compile_well_known_types: bool) -> Self {
        self.compile_well_known_types = compile_well_known_types;
        self
    }

    /// Compile the .proto files and execute code generation.
    pub fn compile<P>(self, protos: &[P], includes: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.compile_with_config(Config::new(), protos, includes)
    }

    /// Compile the .proto files and execute code generation using a
    /// custom `prost_build::Config`.
    pub fn compile_with_config<P>(
        self,
        mut config: Config,
        protos: &[P],
        includes: &[P],
    ) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let out_dir = if let Some(out_dir) = self.out_dir.as_ref() {
            out_dir.clone()
        } else {
            PathBuf::from(std::env::var("OUT_DIR").unwrap())
        };

        let format = self.format;

        config.out_dir(out_dir.clone());
        for (proto_path, rust_path) in self.extern_path.iter() {
            config.extern_path(proto_path, rust_path);
        }
        for (prost_path, attr) in self.field_attributes.iter() {
            config.field_attribute(prost_path, attr);
        }
        for (prost_path, attr) in self.type_attributes.iter() {
            config.type_attribute(prost_path, attr);
        }
        config.service_generator(Box::new(ServiceGenerator::new(self)));

        config.compile_protos(protos, includes)?;

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
            visitor.visit_file_mut(&mut file);
            fs::write(
                path,
                file.to_token_stream()
                    .to_string()
                    .replace(":: prost_types ::", "::proto_graphql::prost_types::"),
            )?;
        }

        {
            if format {
                super::fmt(out_dir.to_str().context("expected utf8 out_dir")?);
            }
        }

        Ok(())
    }
}
