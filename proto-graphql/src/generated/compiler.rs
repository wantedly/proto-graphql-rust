/// Nested message and enum types in `CodeGeneratorResponse`.
pub mod code_generator_response {
    /// Represents a single generated file.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "CompilerCodeGeneratorResponseFile")]
    pub struct FileGraphQl {
        /// The file name, relative to the output directory.  The name must not
        /// contain "." or ".." components and must be relative, not be absolute (so,
        /// the file cannot lie outside the output directory).  "/" must be used as
        /// the path separator, not "\".
        ///
        /// If the name is omitted, the content will be appended to the previous
        /// file.  This allows the generator to break large files into small chunks,
        /// and allows the generated text to be streamed back to protoc so that large
        /// files need not reside completely in memory at one time.  Note that as of
        /// this writing protoc does not optimize for this -- it will read the entire
        /// CodeGeneratorResponse before writing files to disk.
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        /// If non-empty, indicates that the named file should already exist, and the
        /// content here is to be inserted into that file at a defined insertion
        /// point.  This feature allows a code generator to extend the output
        /// produced by another code generator.  The original generator may provide
        /// insertion points by placing special annotations in the file that look
        /// like:
        ///   @@protoc_insertion_point(NAME)
        /// The annotation can have arbitrary text before and after it on the line,
        /// which allows it to be placed in a comment.  NAME should be replaced with
        /// an identifier naming the point -- this is what other generators will use
        /// as the insertion_point.  Code inserted at this point will be placed
        /// immediately above the line containing the insertion point (thus multiple
        /// insertions to the same point will come out in the order they were added).
        /// The double-@ is intended to make it unlikely that the generated code
        /// could contain things that look like insertion points by accident.
        ///
        /// For example, the C++ code generator places the following line in the
        /// .pb.h files that it generates:
        ///   // @@protoc_insertion_point(namespace_scope)
        /// This line appears within the scope of the file's package namespace, but
        /// outside of any particular class.  Another plugin can then specify the
        /// insertion_point "namespace_scope" to generate additional classes or
        /// other declarations that should be placed in this scope.
        ///
        /// Note that if the line containing the insertion point begins with
        /// whitespace, the same whitespace will be added to every line of the
        /// inserted text.  This is useful for languages like Python, where
        /// indentation matters.  In these languages, the insertion point comment
        /// should be indented the same amount as any inserted code will need to be
        /// in order to work correctly in that context.
        ///
        /// The code generator that generates the initial file and the one which
        /// inserts into it must both run as part of a single invocation of protoc.
        /// Code generators are executed in the order in which they appear on the
        /// command line.
        ///
        /// If |insertion_point| is present, |name| must also be present.
        pub insertion_point: ::core::option::Option<::prost::alloc::string::String>,
        /// The file contents.
        pub content: ::core::option::Option<::prost::alloc::string::String>,
        /// Information describing the file content being inserted. If an insertion
        /// point is used, this information will be appropriately offset and inserted
        /// into the code generation metadata for the generated files.
        pub generated_code_info: ::core::option::Option<super::super::GeneratedCodeInfoGraphQl>,
    }
    /// Represents a single generated file.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "CompilerCodeGeneratorResponseFileInput")]
    pub struct FileGraphQlInput {
        /// The file name, relative to the output directory.  The name must not
        /// contain "." or ".." components and must be relative, not be absolute (so,
        /// the file cannot lie outside the output directory).  "/" must be used as
        /// the path separator, not "\".
        ///
        /// If the name is omitted, the content will be appended to the previous
        /// file.  This allows the generator to break large files into small chunks,
        /// and allows the generated text to be streamed back to protoc so that large
        /// files need not reside completely in memory at one time.  Note that as of
        /// this writing protoc does not optimize for this -- it will read the entire
        /// CodeGeneratorResponse before writing files to disk.
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        /// If non-empty, indicates that the named file should already exist, and the
        /// content here is to be inserted into that file at a defined insertion
        /// point.  This feature allows a code generator to extend the output
        /// produced by another code generator.  The original generator may provide
        /// insertion points by placing special annotations in the file that look
        /// like:
        ///   @@protoc_insertion_point(NAME)
        /// The annotation can have arbitrary text before and after it on the line,
        /// which allows it to be placed in a comment.  NAME should be replaced with
        /// an identifier naming the point -- this is what other generators will use
        /// as the insertion_point.  Code inserted at this point will be placed
        /// immediately above the line containing the insertion point (thus multiple
        /// insertions to the same point will come out in the order they were added).
        /// The double-@ is intended to make it unlikely that the generated code
        /// could contain things that look like insertion points by accident.
        ///
        /// For example, the C++ code generator places the following line in the
        /// .pb.h files that it generates:
        ///   // @@protoc_insertion_point(namespace_scope)
        /// This line appears within the scope of the file's package namespace, but
        /// outside of any particular class.  Another plugin can then specify the
        /// insertion_point "namespace_scope" to generate additional classes or
        /// other declarations that should be placed in this scope.
        ///
        /// Note that if the line containing the insertion point begins with
        /// whitespace, the same whitespace will be added to every line of the
        /// inserted text.  This is useful for languages like Python, where
        /// indentation matters.  In these languages, the insertion point comment
        /// should be indented the same amount as any inserted code will need to be
        /// in order to work correctly in that context.
        ///
        /// The code generator that generates the initial file and the one which
        /// inserts into it must both run as part of a single invocation of protoc.
        /// Code generators are executed in the order in which they appear on the
        /// command line.
        ///
        /// If |insertion_point| is present, |name| must also be present.
        pub insertion_point: ::core::option::Option<::prost::alloc::string::String>,
        /// The file contents.
        pub content: ::core::option::Option<::prost::alloc::string::String>,
        /// Information describing the file content being inserted. If an insertion
        /// point is used, this information will be appropriately offset and inserted
        /// into the code generation metadata for the generated files.
        pub generated_code_info:
            ::core::option::Option<super::super::GeneratedCodeInfoGraphQlInput>,
    }
    #[allow(clippy::useless_conversion)]
    impl From<File> for FileGraphQl {
        fn from(other: File) -> Self {
            let File {
                name,
                insertion_point,
                content,
                generated_code_info,
                ..
            } = other;
            Self {
                name: name.map(Into::into),
                insertion_point: insertion_point.map(Into::into),
                content: content.map(Into::into),
                generated_code_info: generated_code_info.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<FileGraphQl> for File {
        fn from(other: FileGraphQl) -> Self {
            let FileGraphQl {
                name,
                insertion_point,
                content,
                generated_code_info,
            } = other;
            Self {
                name: name.map(Into::into),
                insertion_point: insertion_point.map(Into::into),
                content: content.map(Into::into),
                generated_code_info: generated_code_info.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<File> for FileGraphQlInput {
        fn from(other: File) -> Self {
            let File {
                name,
                insertion_point,
                content,
                generated_code_info,
                ..
            } = other;
            Self {
                name: name.map(Into::into),
                insertion_point: insertion_point.map(Into::into),
                content: content.map(Into::into),
                generated_code_info: generated_code_info.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<FileGraphQlInput> for File {
        fn from(other: FileGraphQlInput) -> Self {
            let FileGraphQlInput {
                name,
                insertion_point,
                content,
                generated_code_info,
            } = other;
            Self {
                name: name.map(Into::into),
                insertion_point: insertion_point.map(Into::into),
                content: content.map(Into::into),
                generated_code_info: generated_code_info.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<Feature> for FeatureGraphQl {
        fn from(other: Feature) -> Self {
            match other {
                Feature::None => Self::None,
                Feature::Proto3Optional => Self::Proto3Optional,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<FeatureGraphQl> for Feature {
        fn from(other: FeatureGraphQl) -> Self {
            match other {
                FeatureGraphQl::None => Self::None,
                FeatureGraphQl::Proto3Optional => Self::Proto3Optional,
            }
        }
    }
    /// Sync with code_generator.h.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "CompilerCodeGeneratorResponseFeature")]
    pub enum FeatureGraphQl {
        None = 0,
        Proto3Optional = 1,
    }
    pub use self::FeatureGraphQl as FeatureGraphQlInput;
    #[allow(unused_imports)]
    pub use ::prost_types::compiler::code_generator_response::*;
}
/// The version number of protocol compiler.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "CompilerVersion")]
pub struct VersionGraphQl {
    pub major: ::core::option::Option<i32>,
    pub minor: ::core::option::Option<i32>,
    pub patch: ::core::option::Option<i32>,
    /// A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
    /// be empty for mainline stable releases.
    pub suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// The version number of protocol compiler.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "CompilerVersionInput")]
pub struct VersionGraphQlInput {
    pub major: ::core::option::Option<i32>,
    pub minor: ::core::option::Option<i32>,
    pub patch: ::core::option::Option<i32>,
    /// A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
    /// be empty for mainline stable releases.
    pub suffix: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::useless_conversion)]
impl From<Version> for VersionGraphQl {
    fn from(other: Version) -> Self {
        let Version {
            major,
            minor,
            patch,
            suffix,
            ..
        } = other;
        Self {
            major: major.map(Into::into),
            minor: minor.map(Into::into),
            patch: patch.map(Into::into),
            suffix: suffix.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<VersionGraphQl> for Version {
    fn from(other: VersionGraphQl) -> Self {
        let VersionGraphQl {
            major,
            minor,
            patch,
            suffix,
        } = other;
        Self {
            major: major.map(Into::into),
            minor: minor.map(Into::into),
            patch: patch.map(Into::into),
            suffix: suffix.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Version> for VersionGraphQlInput {
    fn from(other: Version) -> Self {
        let Version {
            major,
            minor,
            patch,
            suffix,
            ..
        } = other;
        Self {
            major: major.map(Into::into),
            minor: minor.map(Into::into),
            patch: patch.map(Into::into),
            suffix: suffix.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<VersionGraphQlInput> for Version {
    fn from(other: VersionGraphQlInput) -> Self {
        let VersionGraphQlInput {
            major,
            minor,
            patch,
            suffix,
        } = other;
        Self {
            major: major.map(Into::into),
            minor: minor.map(Into::into),
            patch: patch.map(Into::into),
            suffix: suffix.map(Into::into),
        }
    }
}
/// An encoded CodeGeneratorRequest is written to the plugin's stdin.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "CompilerCodeGeneratorRequest")]
pub struct CodeGeneratorRequestGraphQl {
    /// The .proto files that were explicitly listed on the command-line.  The
    /// code generator should generate code only for these files.  Each file's
    /// descriptor will be included in proto_file, below.
    pub file_to_generate: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The generator parameter passed on the command-line.
    pub parameter: ::core::option::Option<::prost::alloc::string::String>,
    /// FileDescriptorProtos for all files in files_to_generate and everything
    /// they import.  The files will appear in topological order, so each file
    /// appears before any file that imports it.
    ///
    /// protoc guarantees that all proto_files will be written after
    /// the fields above, even though this is not technically guaranteed by the
    /// protobuf wire format.  This theoretically could allow a plugin to stream
    /// in the FileDescriptorProtos and handle them one by one rather than read
    /// the entire set into memory at once.  However, as of this writing, this
    /// is not similarly optimized on protoc's end -- it will store all fields in
    /// memory at once before sending them to the plugin.
    ///
    /// Type names of fields and extensions in the FileDescriptorProto are always
    /// fully qualified.
    pub proto_file: ::prost::alloc::vec::Vec<super::FileDescriptorProtoGraphQl>,
    /// The version number of protocol compiler.
    pub compiler_version: ::core::option::Option<VersionGraphQl>,
}
/// An encoded CodeGeneratorRequest is written to the plugin's stdin.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "CompilerCodeGeneratorRequestInput")]
pub struct CodeGeneratorRequestGraphQlInput {
    /// The .proto files that were explicitly listed on the command-line.  The
    /// code generator should generate code only for these files.  Each file's
    /// descriptor will be included in proto_file, below.
    pub file_to_generate: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The generator parameter passed on the command-line.
    pub parameter: ::core::option::Option<::prost::alloc::string::String>,
    /// FileDescriptorProtos for all files in files_to_generate and everything
    /// they import.  The files will appear in topological order, so each file
    /// appears before any file that imports it.
    ///
    /// protoc guarantees that all proto_files will be written after
    /// the fields above, even though this is not technically guaranteed by the
    /// protobuf wire format.  This theoretically could allow a plugin to stream
    /// in the FileDescriptorProtos and handle them one by one rather than read
    /// the entire set into memory at once.  However, as of this writing, this
    /// is not similarly optimized on protoc's end -- it will store all fields in
    /// memory at once before sending them to the plugin.
    ///
    /// Type names of fields and extensions in the FileDescriptorProto are always
    /// fully qualified.
    pub proto_file: ::prost::alloc::vec::Vec<super::FileDescriptorProtoGraphQlInput>,
    /// The version number of protocol compiler.
    pub compiler_version: ::core::option::Option<VersionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorRequest> for CodeGeneratorRequestGraphQl {
    fn from(other: CodeGeneratorRequest) -> Self {
        let CodeGeneratorRequest {
            file_to_generate,
            parameter,
            proto_file,
            compiler_version,
            ..
        } = other;
        Self {
            file_to_generate: file_to_generate.into_iter().map(Into::into).collect(),
            parameter: parameter.map(Into::into),
            proto_file: proto_file.into_iter().map(Into::into).collect(),
            compiler_version: compiler_version.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorRequestGraphQl> for CodeGeneratorRequest {
    fn from(other: CodeGeneratorRequestGraphQl) -> Self {
        let CodeGeneratorRequestGraphQl {
            file_to_generate,
            parameter,
            proto_file,
            compiler_version,
        } = other;
        Self {
            file_to_generate: file_to_generate.into_iter().map(Into::into).collect(),
            parameter: parameter.map(Into::into),
            proto_file: proto_file.into_iter().map(Into::into).collect(),
            compiler_version: compiler_version.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorRequest> for CodeGeneratorRequestGraphQlInput {
    fn from(other: CodeGeneratorRequest) -> Self {
        let CodeGeneratorRequest {
            file_to_generate,
            parameter,
            proto_file,
            compiler_version,
            ..
        } = other;
        Self {
            file_to_generate: file_to_generate.into_iter().map(Into::into).collect(),
            parameter: parameter.map(Into::into),
            proto_file: proto_file.into_iter().map(Into::into).collect(),
            compiler_version: compiler_version.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorRequestGraphQlInput> for CodeGeneratorRequest {
    fn from(other: CodeGeneratorRequestGraphQlInput) -> Self {
        let CodeGeneratorRequestGraphQlInput {
            file_to_generate,
            parameter,
            proto_file,
            compiler_version,
        } = other;
        Self {
            file_to_generate: file_to_generate.into_iter().map(Into::into).collect(),
            parameter: parameter.map(Into::into),
            proto_file: proto_file.into_iter().map(Into::into).collect(),
            compiler_version: compiler_version.map(Into::into),
        }
    }
}
/// The plugin writes an encoded CodeGeneratorResponse to stdout.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "CompilerCodeGeneratorResponse")]
pub struct CodeGeneratorResponseGraphQl {
    /// Error message.  If non-empty, code generation failed.  The plugin process
    /// should exit with status code zero even if it reports an error in this way.
    ///
    /// This should be used to indicate errors in .proto files which prevent the
    /// code generator from generating correct code.  Errors which indicate a
    /// problem in protoc itself -- such as the input CodeGeneratorRequest being
    /// unparseable -- should be reported by writing a message to stderr and
    /// exiting with a non-zero status code.
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    /// A bitmask of supported features that the code generator supports.
    /// This is a bitwise "or" of values from the Feature enum.
    pub supported_features: ::core::option::Option<u64>,
    pub file: ::prost::alloc::vec::Vec<code_generator_response::FileGraphQl>,
}
/// The plugin writes an encoded CodeGeneratorResponse to stdout.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "CompilerCodeGeneratorResponseInput")]
pub struct CodeGeneratorResponseGraphQlInput {
    /// Error message.  If non-empty, code generation failed.  The plugin process
    /// should exit with status code zero even if it reports an error in this way.
    ///
    /// This should be used to indicate errors in .proto files which prevent the
    /// code generator from generating correct code.  Errors which indicate a
    /// problem in protoc itself -- such as the input CodeGeneratorRequest being
    /// unparseable -- should be reported by writing a message to stderr and
    /// exiting with a non-zero status code.
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    /// A bitmask of supported features that the code generator supports.
    /// This is a bitwise "or" of values from the Feature enum.
    pub supported_features: ::core::option::Option<u64>,
    pub file: ::prost::alloc::vec::Vec<code_generator_response::FileGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorResponse> for CodeGeneratorResponseGraphQl {
    fn from(other: CodeGeneratorResponse) -> Self {
        let CodeGeneratorResponse {
            error,
            supported_features,
            file,
            ..
        } = other;
        Self {
            error: error.map(Into::into),
            supported_features: supported_features.map(Into::into),
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorResponseGraphQl> for CodeGeneratorResponse {
    fn from(other: CodeGeneratorResponseGraphQl) -> Self {
        let CodeGeneratorResponseGraphQl {
            error,
            supported_features,
            file,
        } = other;
        Self {
            error: error.map(Into::into),
            supported_features: supported_features.map(Into::into),
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorResponse> for CodeGeneratorResponseGraphQlInput {
    fn from(other: CodeGeneratorResponse) -> Self {
        let CodeGeneratorResponse {
            error,
            supported_features,
            file,
            ..
        } = other;
        Self {
            error: error.map(Into::into),
            supported_features: supported_features.map(Into::into),
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<CodeGeneratorResponseGraphQlInput> for CodeGeneratorResponse {
    fn from(other: CodeGeneratorResponseGraphQlInput) -> Self {
        let CodeGeneratorResponseGraphQlInput {
            error,
            supported_features,
            file,
        } = other;
        Self {
            error: error.map(Into::into),
            supported_features: supported_features.map(Into::into),
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(unused_imports)]
pub use ::prost_types::compiler::*;
