/// Nested message and enum types in `DescriptorProto`.
pub mod descriptor_proto {
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "DescriptorProtoExtensionRange")]
    pub struct ExtensionRangeGraphQl {
        /// Inclusive.
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        pub end: ::core::option::Option<i32>,
        pub options: ::core::option::Option<super::ExtensionRangeOptionsGraphQl>,
    }
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "DescriptorProtoExtensionRangeInput")]
    pub struct ExtensionRangeGraphQlInput {
        /// Inclusive.
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        pub end: ::core::option::Option<i32>,
        pub options: ::core::option::Option<super::ExtensionRangeOptionsGraphQlInput>,
    }
    #[allow(clippy::useless_conversion)]
    impl From<ExtensionRange> for ExtensionRangeGraphQl {
        fn from(other: ExtensionRange) -> Self {
            let ExtensionRange {
                start,
                end,
                options,
                ..
            } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
                options: options.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<ExtensionRangeGraphQl> for ExtensionRange {
        fn from(other: ExtensionRangeGraphQl) -> Self {
            let ExtensionRangeGraphQl {
                start,
                end,
                options,
            } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
                options: options.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<ExtensionRange> for ExtensionRangeGraphQlInput {
        fn from(other: ExtensionRange) -> Self {
            let ExtensionRange {
                start,
                end,
                options,
                ..
            } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
                options: options.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<ExtensionRangeGraphQlInput> for ExtensionRange {
        fn from(other: ExtensionRangeGraphQlInput) -> Self {
            let ExtensionRangeGraphQlInput {
                start,
                end,
                options,
            } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
                options: options.map(Into::into),
            }
        }
    }
    /// Range of reserved tag numbers. Reserved tag numbers may not be used by
    /// fields or extension ranges in the same message. Reserved ranges may
    /// not overlap.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "DescriptorProtoReservedRange")]
    pub struct ReservedRangeGraphQl {
        /// Inclusive.
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        pub end: ::core::option::Option<i32>,
    }
    /// Range of reserved tag numbers. Reserved tag numbers may not be used by
    /// fields or extension ranges in the same message. Reserved ranges may
    /// not overlap.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "DescriptorProtoReservedRangeInput")]
    pub struct ReservedRangeGraphQlInput {
        /// Inclusive.
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        pub end: ::core::option::Option<i32>,
    }
    #[allow(clippy::useless_conversion)]
    impl From<ReservedRange> for ReservedRangeGraphQl {
        fn from(other: ReservedRange) -> Self {
            let ReservedRange { start, end, .. } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<ReservedRangeGraphQl> for ReservedRange {
        fn from(other: ReservedRangeGraphQl) -> Self {
            let ReservedRangeGraphQl { start, end } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<ReservedRange> for ReservedRangeGraphQlInput {
        fn from(other: ReservedRange) -> Self {
            let ReservedRange { start, end, .. } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<ReservedRangeGraphQlInput> for ReservedRange {
        fn from(other: ReservedRangeGraphQlInput) -> Self {
            let ReservedRangeGraphQlInput { start, end } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(unused_imports)]
    pub use ::prost_types::descriptor_proto::*;
}
/// Nested message and enum types in `FieldDescriptorProto`.
pub mod field_descriptor_proto {
    #[allow(clippy::useless_conversion)]
    impl From<Type> for TypeGraphQl {
        fn from(other: Type) -> Self {
            match other {
                Type::Double => Self::Double,
                Type::Float => Self::Float,
                Type::Int64 => Self::Int64,
                Type::Uint64 => Self::Uint64,
                Type::Int32 => Self::Int32,
                Type::Fixed64 => Self::Fixed64,
                Type::Fixed32 => Self::Fixed32,
                Type::Bool => Self::Bool,
                Type::String => Self::String,
                Type::Group => Self::Group,
                Type::Message => Self::Message,
                Type::Bytes => Self::Bytes,
                Type::Uint32 => Self::Uint32,
                Type::Enum => Self::Enum,
                Type::Sfixed32 => Self::Sfixed32,
                Type::Sfixed64 => Self::Sfixed64,
                Type::Sint32 => Self::Sint32,
                Type::Sint64 => Self::Sint64,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<TypeGraphQl> for Type {
        fn from(other: TypeGraphQl) -> Self {
            match other {
                TypeGraphQl::Double => Self::Double,
                TypeGraphQl::Float => Self::Float,
                TypeGraphQl::Int64 => Self::Int64,
                TypeGraphQl::Uint64 => Self::Uint64,
                TypeGraphQl::Int32 => Self::Int32,
                TypeGraphQl::Fixed64 => Self::Fixed64,
                TypeGraphQl::Fixed32 => Self::Fixed32,
                TypeGraphQl::Bool => Self::Bool,
                TypeGraphQl::String => Self::String,
                TypeGraphQl::Group => Self::Group,
                TypeGraphQl::Message => Self::Message,
                TypeGraphQl::Bytes => Self::Bytes,
                TypeGraphQl::Uint32 => Self::Uint32,
                TypeGraphQl::Enum => Self::Enum,
                TypeGraphQl::Sfixed32 => Self::Sfixed32,
                TypeGraphQl::Sfixed64 => Self::Sfixed64,
                TypeGraphQl::Sint32 => Self::Sint32,
                TypeGraphQl::Sint64 => Self::Sint64,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "FieldDescriptorProtoType")]
    pub enum TypeGraphQl {
        /// 0 is reserved for errors.
        /// Order is weird for historical reasons.
        Double = 1,
        Float = 2,
        /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
        /// negative values are likely.
        Int64 = 3,
        Uint64 = 4,
        /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
        /// negative values are likely.
        Int32 = 5,
        Fixed64 = 6,
        Fixed32 = 7,
        Bool = 8,
        String = 9,
        /// Tag-delimited aggregate.
        /// Group type is deprecated and not supported in proto3. However, Proto3
        /// implementations should still be able to parse the group wire format and
        /// treat group fields as unknown fields.
        Group = 10,
        /// Length-delimited aggregate.
        Message = 11,
        /// New in version 2.
        Bytes = 12,
        Uint32 = 13,
        Enum = 14,
        Sfixed32 = 15,
        Sfixed64 = 16,
        /// Uses ZigZag encoding.
        Sint32 = 17,
        /// Uses ZigZag encoding.
        Sint64 = 18,
    }
    pub use self::TypeGraphQl as TypeGraphQlInput;
    #[allow(clippy::useless_conversion)]
    impl From<Label> for LabelGraphQl {
        fn from(other: Label) -> Self {
            match other {
                Label::Optional => Self::Optional,
                Label::Required => Self::Required,
                Label::Repeated => Self::Repeated,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<LabelGraphQl> for Label {
        fn from(other: LabelGraphQl) -> Self {
            match other {
                LabelGraphQl::Optional => Self::Optional,
                LabelGraphQl::Required => Self::Required,
                LabelGraphQl::Repeated => Self::Repeated,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "FieldDescriptorProtoLabel")]
    pub enum LabelGraphQl {
        /// 0 is reserved for errors
        Optional = 1,
        Required = 2,
        Repeated = 3,
    }
    pub use self::LabelGraphQl as LabelGraphQlInput;
    #[allow(unused_imports)]
    pub use ::prost_types::field_descriptor_proto::*;
}
/// Nested message and enum types in `EnumDescriptorProto`.
pub mod enum_descriptor_proto {
    /// Range of reserved numeric values. Reserved values may not be used by
    /// entries in the same enum. Reserved ranges may not overlap.
    ///
    /// Note that this is distinct from DescriptorProto.ReservedRange in that it
    /// is inclusive such that it can appropriately represent the entire int32
    /// domain.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "EnumDescriptorProtoEnumReservedRange")]
    pub struct EnumReservedRangeGraphQl {
        /// Inclusive.
        pub start: ::core::option::Option<i32>,
        /// Inclusive.
        pub end: ::core::option::Option<i32>,
    }
    /// Range of reserved numeric values. Reserved values may not be used by
    /// entries in the same enum. Reserved ranges may not overlap.
    ///
    /// Note that this is distinct from DescriptorProto.ReservedRange in that it
    /// is inclusive such that it can appropriately represent the entire int32
    /// domain.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "EnumDescriptorProtoEnumReservedRangeInput")]
    pub struct EnumReservedRangeGraphQlInput {
        /// Inclusive.
        pub start: ::core::option::Option<i32>,
        /// Inclusive.
        pub end: ::core::option::Option<i32>,
    }
    #[allow(clippy::useless_conversion)]
    impl From<EnumReservedRange> for EnumReservedRangeGraphQl {
        fn from(other: EnumReservedRange) -> Self {
            let EnumReservedRange { start, end, .. } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<EnumReservedRangeGraphQl> for EnumReservedRange {
        fn from(other: EnumReservedRangeGraphQl) -> Self {
            let EnumReservedRangeGraphQl { start, end } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<EnumReservedRange> for EnumReservedRangeGraphQlInput {
        fn from(other: EnumReservedRange) -> Self {
            let EnumReservedRange { start, end, .. } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<EnumReservedRangeGraphQlInput> for EnumReservedRange {
        fn from(other: EnumReservedRangeGraphQlInput) -> Self {
            let EnumReservedRangeGraphQlInput { start, end } = other;
            Self {
                start: start.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(unused_imports)]
    pub use ::prost_types::enum_descriptor_proto::*;
}
/// Nested message and enum types in `FileOptions`.
pub mod file_options {
    #[allow(clippy::useless_conversion)]
    impl From<OptimizeMode> for OptimizeModeGraphQl {
        fn from(other: OptimizeMode) -> Self {
            match other {
                OptimizeMode::Speed => Self::Speed,
                OptimizeMode::CodeSize => Self::CodeSize,
                OptimizeMode::LiteRuntime => Self::LiteRuntime,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<OptimizeModeGraphQl> for OptimizeMode {
        fn from(other: OptimizeModeGraphQl) -> Self {
            match other {
                OptimizeModeGraphQl::Speed => Self::Speed,
                OptimizeModeGraphQl::CodeSize => Self::CodeSize,
                OptimizeModeGraphQl::LiteRuntime => Self::LiteRuntime,
            }
        }
    }
    /// Generated classes can be optimized for speed or code size.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "FileOptionsOptimizeMode")]
    pub enum OptimizeModeGraphQl {
        /// Generate complete code for parsing, serialization,
        Speed = 1,
        /// etc.
        ///
        /// Use ReflectionOps to implement these methods.
        CodeSize = 2,
        /// Generate code using MessageLite and the lite runtime.
        LiteRuntime = 3,
    }
    pub use self::OptimizeModeGraphQl as OptimizeModeGraphQlInput;
    #[allow(unused_imports)]
    pub use ::prost_types::file_options::*;
}
/// Nested message and enum types in `FieldOptions`.
pub mod field_options {
    #[allow(clippy::useless_conversion)]
    impl From<CType> for CTypeGraphQl {
        fn from(other: CType) -> Self {
            match other {
                CType::String => Self::String,
                CType::Cord => Self::Cord,
                CType::StringPiece => Self::StringPiece,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<CTypeGraphQl> for CType {
        fn from(other: CTypeGraphQl) -> Self {
            match other {
                CTypeGraphQl::String => Self::String,
                CTypeGraphQl::Cord => Self::Cord,
                CTypeGraphQl::StringPiece => Self::StringPiece,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "FieldOptionsCType")]
    pub enum CTypeGraphQl {
        /// Default mode.
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
    pub use self::CTypeGraphQl as CTypeGraphQlInput;
    #[allow(clippy::useless_conversion)]
    impl From<JsType> for JsTypeGraphQl {
        fn from(other: JsType) -> Self {
            match other {
                JsType::JsNormal => Self::JsNormal,
                JsType::JsString => Self::JsString,
                JsType::JsNumber => Self::JsNumber,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<JsTypeGraphQl> for JsType {
        fn from(other: JsTypeGraphQl) -> Self {
            match other {
                JsTypeGraphQl::JsNormal => Self::JsNormal,
                JsTypeGraphQl::JsString => Self::JsString,
                JsTypeGraphQl::JsNumber => Self::JsNumber,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "FieldOptionsJsType")]
    pub enum JsTypeGraphQl {
        /// Use the default type.
        JsNormal = 0,
        /// Use JavaScript strings.
        JsString = 1,
        /// Use JavaScript numbers.
        JsNumber = 2,
    }
    pub use self::JsTypeGraphQl as JsTypeGraphQlInput;
    #[allow(unused_imports)]
    pub use ::prost_types::field_options::*;
}
/// Nested message and enum types in `MethodOptions`.
pub mod method_options {
    #[allow(clippy::useless_conversion)]
    impl From<IdempotencyLevel> for IdempotencyLevelGraphQl {
        fn from(other: IdempotencyLevel) -> Self {
            match other {
                IdempotencyLevel::IdempotencyUnknown => Self::IdempotencyUnknown,
                IdempotencyLevel::NoSideEffects => Self::NoSideEffects,
                IdempotencyLevel::Idempotent => Self::Idempotent,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<IdempotencyLevelGraphQl> for IdempotencyLevel {
        fn from(other: IdempotencyLevelGraphQl) -> Self {
            match other {
                IdempotencyLevelGraphQl::IdempotencyUnknown => Self::IdempotencyUnknown,
                IdempotencyLevelGraphQl::NoSideEffects => Self::NoSideEffects,
                IdempotencyLevelGraphQl::Idempotent => Self::Idempotent,
            }
        }
    }
    /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
    /// or neither? HTTP based RPC implementation may choose GET verb for safe
    /// methods, and PUT verb for idempotent methods instead of the default POST.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "MethodOptionsIdempotencyLevel")]
    pub enum IdempotencyLevelGraphQl {
        IdempotencyUnknown = 0,
        /// implies idempotent
        NoSideEffects = 1,
        /// idempotent, but may have side effects
        Idempotent = 2,
    }
    pub use self::IdempotencyLevelGraphQl as IdempotencyLevelGraphQlInput;
    #[allow(unused_imports)]
    pub use ::prost_types::method_options::*;
}
/// Nested message and enum types in `UninterpretedOption`.
pub mod uninterpreted_option {
    /// The name of the uninterpreted option.  Each string represents a segment in
    /// a dot-separated name.  is_extension is true iff a segment represents an
    /// extension (denoted with parentheses in options specs in .proto files).
    /// E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
    /// "foo.(bar.baz).qux".
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "UninterpretedOptionNamePart")]
    pub struct NamePartGraphQl {
        pub name_part: ::prost::alloc::string::String,
        pub is_extension: bool,
    }
    /// The name of the uninterpreted option.  Each string represents a segment in
    /// a dot-separated name.  is_extension is true iff a segment represents an
    /// extension (denoted with parentheses in options specs in .proto files).
    /// E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
    /// "foo.(bar.baz).qux".
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "UninterpretedOptionNamePartInput")]
    pub struct NamePartGraphQlInput {
        pub name_part: ::prost::alloc::string::String,
        pub is_extension: bool,
    }
    #[allow(clippy::useless_conversion)]
    impl From<NamePart> for NamePartGraphQl {
        fn from(other: NamePart) -> Self {
            let NamePart {
                name_part,
                is_extension,
                ..
            } = other;
            Self {
                name_part: name_part.into(),
                is_extension: is_extension.into(),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<NamePartGraphQl> for NamePart {
        fn from(other: NamePartGraphQl) -> Self {
            let NamePartGraphQl {
                name_part,
                is_extension,
            } = other;
            Self {
                name_part: name_part.into(),
                is_extension: is_extension.into(),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<NamePart> for NamePartGraphQlInput {
        fn from(other: NamePart) -> Self {
            let NamePart {
                name_part,
                is_extension,
                ..
            } = other;
            Self {
                name_part: name_part.into(),
                is_extension: is_extension.into(),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<NamePartGraphQlInput> for NamePart {
        fn from(other: NamePartGraphQlInput) -> Self {
            let NamePartGraphQlInput {
                name_part,
                is_extension,
            } = other;
            Self {
                name_part: name_part.into(),
                is_extension: is_extension.into(),
            }
        }
    }
    #[allow(unused_imports)]
    pub use ::prost_types::uninterpreted_option::*;
}
/// Nested message and enum types in `SourceCodeInfo`.
pub mod source_code_info {
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "SourceCodeInfoLocation")]
    pub struct LocationGraphQl {
        /// Identifies which part of the FileDescriptorProto was defined at this
        /// location.
        ///
        /// Each element is a field number or an index.  They form a path from
        /// the root FileDescriptorProto to the place where the definition.  For
        /// example, this path:
        ///   [ 4, 3, 2, 7, 1 ]
        /// refers to:
        ///   file.message_type(3)  // 4, 3
        ///       .field(7)         // 2, 7
        ///       .name()           // 1
        /// This is because FileDescriptorProto.message_type has field number 4:
        ///   repeated DescriptorProto message_type = 4;
        /// and DescriptorProto.field has field number 2:
        ///   repeated FieldDescriptorProto field = 2;
        /// and FieldDescriptorProto.name has field number 1:
        ///   optional string name = 1;
        ///
        /// Thus, the above path gives the location of a field name.  If we removed
        /// the last element:
        ///   [ 4, 3, 2, 7 ]
        /// this path refers to the whole field declaration (from the beginning
        /// of the label to the terminating semicolon).
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Always has exactly three or four elements: start line, start column,
        /// end line (optional, otherwise assumed same as start line), end column.
        /// These are packed into a single field for efficiency.  Note that line
        /// and column numbers are zero-based -- typically you will want to add
        /// 1 to each before displaying to a user.
        pub span: ::prost::alloc::vec::Vec<i32>,
        /// If this SourceCodeInfo represents a complete declaration, these are any
        /// comments appearing before and after the declaration which appear to be
        /// attached to the declaration.
        ///
        /// A series of line comments appearing on consecutive lines, with no other
        /// tokens appearing on those lines, will be treated as a single comment.
        ///
        /// leading_detached_comments will keep paragraphs of comments that appear
        /// before (but not connected to) the current element. Each paragraph,
        /// separated by empty lines, will be one comment element in the repeated
        /// field.
        ///
        /// Only the comment content is provided; comment markers (e.g. //) are
        /// stripped out.  For block comments, leading whitespace and an asterisk
        /// will be stripped from the beginning of each line other than the first.
        /// Newlines are included in the output.
        ///
        /// Examples:
        ///
        ///   optional int32 foo = 1;  // Comment attached to foo.
        ///   // Comment attached to bar.
        ///   optional int32 bar = 2;
        ///
        ///   optional string baz = 3;
        ///   // Comment attached to baz.
        ///   // Another line attached to baz.
        ///
        ///   // Comment attached to qux.
        ///   //
        ///   // Another line attached to qux.
        ///   optional double qux = 4;
        ///
        ///   // Detached comment for corge. This is not leading or trailing comments
        ///   // to qux or corge because there are blank lines separating it from
        ///   // both.
        ///
        ///   // Detached comment for corge paragraph 2.
        ///
        ///   optional string corge = 5;
        ///   /* Block comment attached
        ///    * to corge.  Leading asterisks
        ///    * will be removed. */
        ///   /* Block comment attached to
        ///    * grault. */
        ///   optional int32 grault = 6;
        ///
        ///   // ignored detached comments.
        pub leading_comments: ::core::option::Option<::prost::alloc::string::String>,
        pub trailing_comments: ::core::option::Option<::prost::alloc::string::String>,
        pub leading_detached_comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "SourceCodeInfoLocationInput")]
    pub struct LocationGraphQlInput {
        /// Identifies which part of the FileDescriptorProto was defined at this
        /// location.
        ///
        /// Each element is a field number or an index.  They form a path from
        /// the root FileDescriptorProto to the place where the definition.  For
        /// example, this path:
        ///   [ 4, 3, 2, 7, 1 ]
        /// refers to:
        ///   file.message_type(3)  // 4, 3
        ///       .field(7)         // 2, 7
        ///       .name()           // 1
        /// This is because FileDescriptorProto.message_type has field number 4:
        ///   repeated DescriptorProto message_type = 4;
        /// and DescriptorProto.field has field number 2:
        ///   repeated FieldDescriptorProto field = 2;
        /// and FieldDescriptorProto.name has field number 1:
        ///   optional string name = 1;
        ///
        /// Thus, the above path gives the location of a field name.  If we removed
        /// the last element:
        ///   [ 4, 3, 2, 7 ]
        /// this path refers to the whole field declaration (from the beginning
        /// of the label to the terminating semicolon).
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Always has exactly three or four elements: start line, start column,
        /// end line (optional, otherwise assumed same as start line), end column.
        /// These are packed into a single field for efficiency.  Note that line
        /// and column numbers are zero-based -- typically you will want to add
        /// 1 to each before displaying to a user.
        pub span: ::prost::alloc::vec::Vec<i32>,
        /// If this SourceCodeInfo represents a complete declaration, these are any
        /// comments appearing before and after the declaration which appear to be
        /// attached to the declaration.
        ///
        /// A series of line comments appearing on consecutive lines, with no other
        /// tokens appearing on those lines, will be treated as a single comment.
        ///
        /// leading_detached_comments will keep paragraphs of comments that appear
        /// before (but not connected to) the current element. Each paragraph,
        /// separated by empty lines, will be one comment element in the repeated
        /// field.
        ///
        /// Only the comment content is provided; comment markers (e.g. //) are
        /// stripped out.  For block comments, leading whitespace and an asterisk
        /// will be stripped from the beginning of each line other than the first.
        /// Newlines are included in the output.
        ///
        /// Examples:
        ///
        ///   optional int32 foo = 1;  // Comment attached to foo.
        ///   // Comment attached to bar.
        ///   optional int32 bar = 2;
        ///
        ///   optional string baz = 3;
        ///   // Comment attached to baz.
        ///   // Another line attached to baz.
        ///
        ///   // Comment attached to qux.
        ///   //
        ///   // Another line attached to qux.
        ///   optional double qux = 4;
        ///
        ///   // Detached comment for corge. This is not leading or trailing comments
        ///   // to qux or corge because there are blank lines separating it from
        ///   // both.
        ///
        ///   // Detached comment for corge paragraph 2.
        ///
        ///   optional string corge = 5;
        ///   /* Block comment attached
        ///    * to corge.  Leading asterisks
        ///    * will be removed. */
        ///   /* Block comment attached to
        ///    * grault. */
        ///   optional int32 grault = 6;
        ///
        ///   // ignored detached comments.
        pub leading_comments: ::core::option::Option<::prost::alloc::string::String>,
        pub trailing_comments: ::core::option::Option<::prost::alloc::string::String>,
        pub leading_detached_comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::useless_conversion)]
    impl From<Location> for LocationGraphQl {
        fn from(other: Location) -> Self {
            let Location {
                path,
                span,
                leading_comments,
                trailing_comments,
                leading_detached_comments,
                ..
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                span: span.into_iter().map(Into::into).collect(),
                leading_comments: leading_comments.map(Into::into),
                trailing_comments: trailing_comments.map(Into::into),
                leading_detached_comments: leading_detached_comments
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<LocationGraphQl> for Location {
        fn from(other: LocationGraphQl) -> Self {
            let LocationGraphQl {
                path,
                span,
                leading_comments,
                trailing_comments,
                leading_detached_comments,
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                span: span.into_iter().map(Into::into).collect(),
                leading_comments: leading_comments.map(Into::into),
                trailing_comments: trailing_comments.map(Into::into),
                leading_detached_comments: leading_detached_comments
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<Location> for LocationGraphQlInput {
        fn from(other: Location) -> Self {
            let Location {
                path,
                span,
                leading_comments,
                trailing_comments,
                leading_detached_comments,
                ..
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                span: span.into_iter().map(Into::into).collect(),
                leading_comments: leading_comments.map(Into::into),
                trailing_comments: trailing_comments.map(Into::into),
                leading_detached_comments: leading_detached_comments
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<LocationGraphQlInput> for Location {
        fn from(other: LocationGraphQlInput) -> Self {
            let LocationGraphQlInput {
                path,
                span,
                leading_comments,
                trailing_comments,
                leading_detached_comments,
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                span: span.into_iter().map(Into::into).collect(),
                leading_comments: leading_comments.map(Into::into),
                trailing_comments: trailing_comments.map(Into::into),
                leading_detached_comments: leading_detached_comments
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }
    #[allow(unused_imports)]
    pub use ::prost_types::source_code_info::*;
}
/// Nested message and enum types in `GeneratedCodeInfo`.
pub mod generated_code_info {
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "GeneratedCodeInfoAnnotation")]
    pub struct AnnotationGraphQl {
        /// Identifies the element in the original source .proto file. This field
        /// is formatted the same as SourceCodeInfo.Location.path.
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Identifies the filesystem path to the original source .proto.
        pub source_file: ::core::option::Option<::prost::alloc::string::String>,
        /// Identifies the starting offset in bytes in the generated code
        /// that relates to the identified object.
        pub begin: ::core::option::Option<i32>,
        /// Identifies the ending offset in bytes in the generated code that
        /// relates to the identified offset. The end offset should be one past
        /// the last relevant byte (so the length of the text = end - begin).
        pub end: ::core::option::Option<i32>,
    }
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "GeneratedCodeInfoAnnotationInput")]
    pub struct AnnotationGraphQlInput {
        /// Identifies the element in the original source .proto file. This field
        /// is formatted the same as SourceCodeInfo.Location.path.
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Identifies the filesystem path to the original source .proto.
        pub source_file: ::core::option::Option<::prost::alloc::string::String>,
        /// Identifies the starting offset in bytes in the generated code
        /// that relates to the identified object.
        pub begin: ::core::option::Option<i32>,
        /// Identifies the ending offset in bytes in the generated code that
        /// relates to the identified offset. The end offset should be one past
        /// the last relevant byte (so the length of the text = end - begin).
        pub end: ::core::option::Option<i32>,
    }
    #[allow(clippy::useless_conversion)]
    impl From<Annotation> for AnnotationGraphQl {
        fn from(other: Annotation) -> Self {
            let Annotation {
                path,
                source_file,
                begin,
                end,
                ..
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                source_file: source_file.map(Into::into),
                begin: begin.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<AnnotationGraphQl> for Annotation {
        fn from(other: AnnotationGraphQl) -> Self {
            let AnnotationGraphQl {
                path,
                source_file,
                begin,
                end,
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                source_file: source_file.map(Into::into),
                begin: begin.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<Annotation> for AnnotationGraphQlInput {
        fn from(other: Annotation) -> Self {
            let Annotation {
                path,
                source_file,
                begin,
                end,
                ..
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                source_file: source_file.map(Into::into),
                begin: begin.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<AnnotationGraphQlInput> for Annotation {
        fn from(other: AnnotationGraphQlInput) -> Self {
            let AnnotationGraphQlInput {
                path,
                source_file,
                begin,
                end,
            } = other;
            Self {
                path: path.into_iter().map(Into::into).collect(),
                source_file: source_file.map(Into::into),
                begin: begin.map(Into::into),
                end: end.map(Into::into),
            }
        }
    }
    #[allow(unused_imports)]
    pub use ::prost_types::generated_code_info::*;
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[allow(clippy::useless_conversion)]
    impl From<Kind> for KindGraphQl {
        fn from(other: Kind) -> Self {
            match other {
                Kind::TypeUnknown => Self::TypeUnknown,
                Kind::TypeDouble => Self::TypeDouble,
                Kind::TypeFloat => Self::TypeFloat,
                Kind::TypeInt64 => Self::TypeInt64,
                Kind::TypeUint64 => Self::TypeUint64,
                Kind::TypeInt32 => Self::TypeInt32,
                Kind::TypeFixed64 => Self::TypeFixed64,
                Kind::TypeFixed32 => Self::TypeFixed32,
                Kind::TypeBool => Self::TypeBool,
                Kind::TypeString => Self::TypeString,
                Kind::TypeGroup => Self::TypeGroup,
                Kind::TypeMessage => Self::TypeMessage,
                Kind::TypeBytes => Self::TypeBytes,
                Kind::TypeUint32 => Self::TypeUint32,
                Kind::TypeEnum => Self::TypeEnum,
                Kind::TypeSfixed32 => Self::TypeSfixed32,
                Kind::TypeSfixed64 => Self::TypeSfixed64,
                Kind::TypeSint32 => Self::TypeSint32,
                Kind::TypeSint64 => Self::TypeSint64,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<KindGraphQl> for Kind {
        fn from(other: KindGraphQl) -> Self {
            match other {
                KindGraphQl::TypeUnknown => Self::TypeUnknown,
                KindGraphQl::TypeDouble => Self::TypeDouble,
                KindGraphQl::TypeFloat => Self::TypeFloat,
                KindGraphQl::TypeInt64 => Self::TypeInt64,
                KindGraphQl::TypeUint64 => Self::TypeUint64,
                KindGraphQl::TypeInt32 => Self::TypeInt32,
                KindGraphQl::TypeFixed64 => Self::TypeFixed64,
                KindGraphQl::TypeFixed32 => Self::TypeFixed32,
                KindGraphQl::TypeBool => Self::TypeBool,
                KindGraphQl::TypeString => Self::TypeString,
                KindGraphQl::TypeGroup => Self::TypeGroup,
                KindGraphQl::TypeMessage => Self::TypeMessage,
                KindGraphQl::TypeBytes => Self::TypeBytes,
                KindGraphQl::TypeUint32 => Self::TypeUint32,
                KindGraphQl::TypeEnum => Self::TypeEnum,
                KindGraphQl::TypeSfixed32 => Self::TypeSfixed32,
                KindGraphQl::TypeSfixed64 => Self::TypeSfixed64,
                KindGraphQl::TypeSint32 => Self::TypeSint32,
                KindGraphQl::TypeSint64 => Self::TypeSint64,
            }
        }
    }
    /// Basic field types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "FieldKind")]
    pub enum KindGraphQl {
        /// Field type unknown.
        TypeUnknown = 0,
        /// Field type double.
        TypeDouble = 1,
        /// Field type float.
        TypeFloat = 2,
        /// Field type int64.
        TypeInt64 = 3,
        /// Field type uint64.
        TypeUint64 = 4,
        /// Field type int32.
        TypeInt32 = 5,
        /// Field type fixed64.
        TypeFixed64 = 6,
        /// Field type fixed32.
        TypeFixed32 = 7,
        /// Field type bool.
        TypeBool = 8,
        /// Field type string.
        TypeString = 9,
        /// Field type group. Proto2 syntax only, and deprecated.
        TypeGroup = 10,
        /// Field type message.
        TypeMessage = 11,
        /// Field type bytes.
        TypeBytes = 12,
        /// Field type uint32.
        TypeUint32 = 13,
        /// Field type enum.
        TypeEnum = 14,
        /// Field type sfixed32.
        TypeSfixed32 = 15,
        /// Field type sfixed64.
        TypeSfixed64 = 16,
        /// Field type sint32.
        TypeSint32 = 17,
        /// Field type sint64.
        TypeSint64 = 18,
    }
    pub use self::KindGraphQl as KindGraphQlInput;
    #[allow(clippy::useless_conversion)]
    impl From<Cardinality> for CardinalityGraphQl {
        fn from(other: Cardinality) -> Self {
            match other {
                Cardinality::Unknown => Self::Unknown,
                Cardinality::Optional => Self::Optional,
                Cardinality::Required => Self::Required,
                Cardinality::Repeated => Self::Repeated,
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<CardinalityGraphQl> for Cardinality {
        fn from(other: CardinalityGraphQl) -> Self {
            match other {
                CardinalityGraphQl::Unknown => Self::Unknown,
                CardinalityGraphQl::Optional => Self::Optional,
                CardinalityGraphQl::Required => Self::Required,
                CardinalityGraphQl::Repeated => Self::Repeated,
            }
        }
    }
    /// Whether a field is optional, required, or repeated.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    #[derive(
        :: async_graphql :: Enum,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "FieldCardinality")]
    pub enum CardinalityGraphQl {
        /// For fields with unknown cardinality.
        Unknown = 0,
        /// For optional fields.
        Optional = 1,
        /// For required fields. Proto2 syntax only.
        Required = 2,
        /// For repeated fields.
        Repeated = 3,
    }
    pub use self::CardinalityGraphQl as CardinalityGraphQlInput;
    #[allow(unused_imports)]
    pub use ::prost_types::field::*;
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[graphql(name = "ValueKindNullValue")]
    pub struct KindGraphQlNullValue {
        null_value: i32,
    }
    #[allow(clippy::useless_conversion)]
    impl From<KindGraphQlNullValue> for i32 {
        fn from(other: KindGraphQlNullValue) -> Self {
            other.null_value
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<i32> for KindGraphQlNullValue {
        fn from(other: i32) -> Self {
            Self { null_value: other }
        }
    }
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: SimpleObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[graphql(name = "ValueKindNumberValue")]
    pub struct KindGraphQlNumberValue {
        number_value: f64,
    }
    #[allow(clippy::useless_conversion)]
    impl From<KindGraphQlNumberValue> for f64 {
        fn from(other: KindGraphQlNumberValue) -> Self {
            other.number_value
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<f64> for KindGraphQlNumberValue {
        fn from(other: f64) -> Self {
            Self {
                number_value: other,
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
    #[graphql(name = "ValueKindStringValue")]
    pub struct KindGraphQlStringValue {
        string_value: ::prost::alloc::string::String,
    }
    #[allow(clippy::useless_conversion)]
    impl From<KindGraphQlStringValue> for ::prost::alloc::string::String {
        fn from(other: KindGraphQlStringValue) -> Self {
            other.string_value
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<::prost::alloc::string::String> for KindGraphQlStringValue {
        fn from(other: ::prost::alloc::string::String) -> Self {
            Self {
                string_value: other,
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
    #[graphql(name = "ValueKindBoolValue")]
    pub struct KindGraphQlBoolValue {
        bool_value: bool,
    }
    #[allow(clippy::useless_conversion)]
    impl From<KindGraphQlBoolValue> for bool {
        fn from(other: KindGraphQlBoolValue) -> Self {
            other.bool_value
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<bool> for KindGraphQlBoolValue {
        fn from(other: bool) -> Self {
            Self { bool_value: other }
        }
    }
    /// The kind of value.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: InputObject,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "ValueKindInput")]
    pub struct KindGraphQlInput {
        null_value: ::core::option::Option<i32>,
        number_value: ::core::option::Option<f64>,
        string_value: ::core::option::Option<::prost::alloc::string::String>,
        bool_value: ::core::option::Option<bool>,
        struct_value: ::core::option::Option<super::StructGraphQlInput>,
        list_value: ::core::option::Option<super::ListValueGraphQlInput>,
    }
    #[allow(clippy::useless_conversion)]
    impl From<Kind> for KindGraphQlInput {
        fn from(other: Kind) -> Self {
            match other {
                Kind::NullValue(null_value) => Self {
                    null_value: ::core::option::Option::Some(null_value.into()),
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                },
                Kind::NumberValue(number_value) => Self {
                    number_value: ::core::option::Option::Some(number_value.into()),
                    null_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                },
                Kind::StringValue(string_value) => Self {
                    string_value: ::core::option::Option::Some(string_value.into()),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                },
                Kind::BoolValue(bool_value) => Self {
                    bool_value: ::core::option::Option::Some(bool_value.into()),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                },
                Kind::StructValue(struct_value) => Self {
                    struct_value: ::core::option::Option::Some(struct_value.into()),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                },
                Kind::ListValue(list_value) => Self {
                    list_value: ::core::option::Option::Some(list_value.into()),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                },
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<KindGraphQlInput> for Kind {
        fn from(other: KindGraphQlInput) -> Self {
            match other {
                KindGraphQlInput {
                    null_value: ::core::option::Option::Some(null_value),
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                } => Self::NullValue(null_value.into()),
                KindGraphQlInput {
                    number_value: ::core::option::Option::Some(number_value),
                    null_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                } => Self::NumberValue(number_value.into()),
                KindGraphQlInput {
                    string_value: ::core::option::Option::Some(string_value),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                } => Self::StringValue(string_value.into()),
                KindGraphQlInput {
                    bool_value: ::core::option::Option::Some(bool_value),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                } => Self::BoolValue(bool_value.into()),
                KindGraphQlInput {
                    struct_value: ::core::option::Option::Some(struct_value),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    list_value: ::core::option::Option::None,
                } => Self::StructValue(struct_value.into()),
                KindGraphQlInput {
                    list_value: ::core::option::Option::Some(list_value),
                    null_value: ::core::option::Option::None,
                    number_value: ::core::option::Option::None,
                    string_value: ::core::option::Option::None,
                    bool_value: ::core::option::Option::None,
                    struct_value: ::core::option::Option::None,
                } => Self::ListValue(list_value.into()),
                _ => panic!("invalid input"),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<Kind> for KindGraphQl {
        fn from(other: Kind) -> Self {
            match other {
                Kind::NullValue(_binding0) => Self::NullValue(_binding0.into()),
                Kind::NumberValue(_binding0) => Self::NumberValue(_binding0.into()),
                Kind::StringValue(_binding0) => Self::StringValue(_binding0.into()),
                Kind::BoolValue(_binding0) => Self::BoolValue(_binding0.into()),
                Kind::StructValue(_binding0) => Self::StructValue(_binding0.into()),
                Kind::ListValue(_binding0) => Self::ListValue(_binding0.into()),
            }
        }
    }
    #[allow(clippy::useless_conversion)]
    impl From<KindGraphQl> for Kind {
        fn from(other: KindGraphQl) -> Self {
            match other {
                KindGraphQl::NullValue(_binding0) => Self::NullValue(_binding0.into()),
                KindGraphQl::NumberValue(_binding0) => Self::NumberValue(_binding0.into()),
                KindGraphQl::StringValue(_binding0) => Self::StringValue(_binding0.into()),
                KindGraphQl::BoolValue(_binding0) => Self::BoolValue(_binding0.into()),
                KindGraphQl::StructValue(_binding0) => Self::StructValue(_binding0.into()),
                KindGraphQl::ListValue(_binding0) => Self::ListValue(_binding0.into()),
            }
        }
    }
    /// The kind of value.
    #[derive(
        Clone,
        PartialEq,
        :: async_graphql :: Union,
        :: proto_graphql :: serde :: Serialize,
        :: proto_graphql :: serde :: Deserialize,
    )]
    #[serde(crate = "::proto_graphql::serde")]
    #[graphql(name = "ValueKind")]
    pub enum KindGraphQl {
        /// Represents a null value.
        NullValue(KindGraphQlNullValue),
        /// Represents a double value.
        NumberValue(KindGraphQlNumberValue),
        /// Represents a string value.
        StringValue(KindGraphQlStringValue),
        /// Represents a boolean value.
        BoolValue(KindGraphQlBoolValue),
        /// Represents a structured value.
        StructValue(super::StructGraphQl),
        /// Represents a repeated `Value`.
        ListValue(super::ListValueGraphQl),
    }
    #[allow(unused_imports)]
    pub use ::prost_types::value::*;
}
/// The protocol compiler can output a FileDescriptorSet containing the .proto
/// files it parses.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FileDescriptorSet")]
pub struct FileDescriptorSetGraphQl {
    pub file: ::prost::alloc::vec::Vec<FileDescriptorProtoGraphQl>,
}
/// The protocol compiler can output a FileDescriptorSet containing the .proto
/// files it parses.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FileDescriptorSetInput")]
pub struct FileDescriptorSetGraphQlInput {
    pub file: ::prost::alloc::vec::Vec<FileDescriptorProtoGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorSet> for FileDescriptorSetGraphQl {
    fn from(other: FileDescriptorSet) -> Self {
        let FileDescriptorSet { file, .. } = other;
        Self {
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorSetGraphQl> for FileDescriptorSet {
    fn from(other: FileDescriptorSetGraphQl) -> Self {
        let FileDescriptorSetGraphQl { file } = other;
        Self {
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorSet> for FileDescriptorSetGraphQlInput {
    fn from(other: FileDescriptorSet) -> Self {
        let FileDescriptorSet { file, .. } = other;
        Self {
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorSetGraphQlInput> for FileDescriptorSet {
    fn from(other: FileDescriptorSetGraphQlInput) -> Self {
        let FileDescriptorSetGraphQlInput { file } = other;
        Self {
            file: file.into_iter().map(Into::into).collect(),
        }
    }
}
/// Describes a complete .proto file.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FileDescriptorProto")]
pub struct FileDescriptorProtoGraphQl {
    /// file name, relative to root of source tree
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// e.g. "foo", "foo.bar", etc.
    pub package: ::core::option::Option<::prost::alloc::string::String>,
    /// Names of files imported by this file.
    pub dependency: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indexes of the public imported files in the dependency list above.
    pub public_dependency: ::prost::alloc::vec::Vec<i32>,
    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    pub weak_dependency: ::prost::alloc::vec::Vec<i32>,
    /// All top-level definitions in this file.
    pub message_type: ::prost::alloc::vec::Vec<DescriptorProtoGraphQl>,
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProtoGraphQl>,
    pub service: ::prost::alloc::vec::Vec<ServiceDescriptorProtoGraphQl>,
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProtoGraphQl>,
    pub options: ::core::option::Option<FileOptionsGraphQl>,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    pub source_code_info: ::core::option::Option<SourceCodeInfoGraphQl>,
    /// The syntax of the proto file.
    /// The supported values are "proto2" and "proto3".
    pub syntax: ::core::option::Option<::prost::alloc::string::String>,
}
/// Describes a complete .proto file.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FileDescriptorProtoInput")]
pub struct FileDescriptorProtoGraphQlInput {
    /// file name, relative to root of source tree
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// e.g. "foo", "foo.bar", etc.
    pub package: ::core::option::Option<::prost::alloc::string::String>,
    /// Names of files imported by this file.
    pub dependency: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indexes of the public imported files in the dependency list above.
    pub public_dependency: ::prost::alloc::vec::Vec<i32>,
    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    pub weak_dependency: ::prost::alloc::vec::Vec<i32>,
    /// All top-level definitions in this file.
    pub message_type: ::prost::alloc::vec::Vec<DescriptorProtoGraphQlInput>,
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProtoGraphQlInput>,
    pub service: ::prost::alloc::vec::Vec<ServiceDescriptorProtoGraphQlInput>,
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProtoGraphQlInput>,
    pub options: ::core::option::Option<FileOptionsGraphQlInput>,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    pub source_code_info: ::core::option::Option<SourceCodeInfoGraphQlInput>,
    /// The syntax of the proto file.
    /// The supported values are "proto2" and "proto3".
    pub syntax: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorProto> for FileDescriptorProtoGraphQl {
    fn from(other: FileDescriptorProto) -> Self {
        let FileDescriptorProto {
            name,
            package,
            dependency,
            public_dependency,
            weak_dependency,
            message_type,
            enum_type,
            service,
            extension,
            options,
            source_code_info,
            syntax,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            package: package.map(Into::into),
            dependency: dependency.into_iter().map(Into::into).collect(),
            public_dependency: public_dependency.into_iter().map(Into::into).collect(),
            weak_dependency: weak_dependency.into_iter().map(Into::into).collect(),
            message_type: message_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            service: service.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            source_code_info: source_code_info.map(Into::into),
            syntax: syntax.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorProtoGraphQl> for FileDescriptorProto {
    fn from(other: FileDescriptorProtoGraphQl) -> Self {
        let FileDescriptorProtoGraphQl {
            name,
            package,
            dependency,
            public_dependency,
            weak_dependency,
            message_type,
            enum_type,
            service,
            extension,
            options,
            source_code_info,
            syntax,
        } = other;
        Self {
            name: name.map(Into::into),
            package: package.map(Into::into),
            dependency: dependency.into_iter().map(Into::into).collect(),
            public_dependency: public_dependency.into_iter().map(Into::into).collect(),
            weak_dependency: weak_dependency.into_iter().map(Into::into).collect(),
            message_type: message_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            service: service.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            source_code_info: source_code_info.map(Into::into),
            syntax: syntax.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorProto> for FileDescriptorProtoGraphQlInput {
    fn from(other: FileDescriptorProto) -> Self {
        let FileDescriptorProto {
            name,
            package,
            dependency,
            public_dependency,
            weak_dependency,
            message_type,
            enum_type,
            service,
            extension,
            options,
            source_code_info,
            syntax,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            package: package.map(Into::into),
            dependency: dependency.into_iter().map(Into::into).collect(),
            public_dependency: public_dependency.into_iter().map(Into::into).collect(),
            weak_dependency: weak_dependency.into_iter().map(Into::into).collect(),
            message_type: message_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            service: service.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            source_code_info: source_code_info.map(Into::into),
            syntax: syntax.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileDescriptorProtoGraphQlInput> for FileDescriptorProto {
    fn from(other: FileDescriptorProtoGraphQlInput) -> Self {
        let FileDescriptorProtoGraphQlInput {
            name,
            package,
            dependency,
            public_dependency,
            weak_dependency,
            message_type,
            enum_type,
            service,
            extension,
            options,
            source_code_info,
            syntax,
        } = other;
        Self {
            name: name.map(Into::into),
            package: package.map(Into::into),
            dependency: dependency.into_iter().map(Into::into).collect(),
            public_dependency: public_dependency.into_iter().map(Into::into).collect(),
            weak_dependency: weak_dependency.into_iter().map(Into::into).collect(),
            message_type: message_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            service: service.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            source_code_info: source_code_info.map(Into::into),
            syntax: syntax.map(Into::into),
        }
    }
}
/// Describes a message type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "DescriptorProto")]
pub struct DescriptorProtoGraphQl {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub field: ::prost::alloc::vec::Vec<FieldDescriptorProtoGraphQl>,
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProtoGraphQl>,
    pub nested_type: ::prost::alloc::vec::Vec<DescriptorProtoGraphQl>,
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProtoGraphQl>,
    pub extension_range: ::prost::alloc::vec::Vec<descriptor_proto::ExtensionRangeGraphQl>,
    pub oneof_decl: ::prost::alloc::vec::Vec<OneofDescriptorProtoGraphQl>,
    pub options: ::core::option::Option<MessageOptionsGraphQl>,
    pub reserved_range: ::prost::alloc::vec::Vec<descriptor_proto::ReservedRangeGraphQl>,
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Describes a message type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "DescriptorProtoInput")]
pub struct DescriptorProtoGraphQlInput {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub field: ::prost::alloc::vec::Vec<FieldDescriptorProtoGraphQlInput>,
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProtoGraphQlInput>,
    pub nested_type: ::prost::alloc::vec::Vec<DescriptorProtoGraphQlInput>,
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProtoGraphQlInput>,
    pub extension_range: ::prost::alloc::vec::Vec<descriptor_proto::ExtensionRangeGraphQlInput>,
    pub oneof_decl: ::prost::alloc::vec::Vec<OneofDescriptorProtoGraphQlInput>,
    pub options: ::core::option::Option<MessageOptionsGraphQlInput>,
    pub reserved_range: ::prost::alloc::vec::Vec<descriptor_proto::ReservedRangeGraphQlInput>,
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::useless_conversion)]
impl From<DescriptorProto> for DescriptorProtoGraphQl {
    fn from(other: DescriptorProto) -> Self {
        let DescriptorProto {
            name,
            field,
            extension,
            nested_type,
            enum_type,
            extension_range,
            oneof_decl,
            options,
            reserved_range,
            reserved_name,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            field: field.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            nested_type: nested_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            extension_range: extension_range.into_iter().map(Into::into).collect(),
            oneof_decl: oneof_decl.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<DescriptorProtoGraphQl> for DescriptorProto {
    fn from(other: DescriptorProtoGraphQl) -> Self {
        let DescriptorProtoGraphQl {
            name,
            field,
            extension,
            nested_type,
            enum_type,
            extension_range,
            oneof_decl,
            options,
            reserved_range,
            reserved_name,
        } = other;
        Self {
            name: name.map(Into::into),
            field: field.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            nested_type: nested_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            extension_range: extension_range.into_iter().map(Into::into).collect(),
            oneof_decl: oneof_decl.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<DescriptorProto> for DescriptorProtoGraphQlInput {
    fn from(other: DescriptorProto) -> Self {
        let DescriptorProto {
            name,
            field,
            extension,
            nested_type,
            enum_type,
            extension_range,
            oneof_decl,
            options,
            reserved_range,
            reserved_name,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            field: field.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            nested_type: nested_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            extension_range: extension_range.into_iter().map(Into::into).collect(),
            oneof_decl: oneof_decl.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<DescriptorProtoGraphQlInput> for DescriptorProto {
    fn from(other: DescriptorProtoGraphQlInput) -> Self {
        let DescriptorProtoGraphQlInput {
            name,
            field,
            extension,
            nested_type,
            enum_type,
            extension_range,
            oneof_decl,
            options,
            reserved_range,
            reserved_name,
        } = other;
        Self {
            name: name.map(Into::into),
            field: field.into_iter().map(Into::into).collect(),
            extension: extension.into_iter().map(Into::into).collect(),
            nested_type: nested_type.into_iter().map(Into::into).collect(),
            enum_type: enum_type.into_iter().map(Into::into).collect(),
            extension_range: extension_range.into_iter().map(Into::into).collect(),
            oneof_decl: oneof_decl.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
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
#[graphql(name = "ExtensionRangeOptions")]
pub struct ExtensionRangeOptionsGraphQl {
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ExtensionRangeOptionsInput")]
pub struct ExtensionRangeOptionsGraphQlInput {
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ExtensionRangeOptions> for ExtensionRangeOptionsGraphQl {
    fn from(other: ExtensionRangeOptions) -> Self {
        let ExtensionRangeOptions {
            uninterpreted_option,
            ..
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ExtensionRangeOptionsGraphQl> for ExtensionRangeOptions {
    fn from(other: ExtensionRangeOptionsGraphQl) -> Self {
        let ExtensionRangeOptionsGraphQl {
            uninterpreted_option,
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ExtensionRangeOptions> for ExtensionRangeOptionsGraphQlInput {
    fn from(other: ExtensionRangeOptions) -> Self {
        let ExtensionRangeOptions {
            uninterpreted_option,
            ..
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ExtensionRangeOptionsGraphQlInput> for ExtensionRangeOptions {
    fn from(other: ExtensionRangeOptionsGraphQlInput) -> Self {
        let ExtensionRangeOptionsGraphQlInput {
            uninterpreted_option,
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
/// Describes a field within a message.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FieldDescriptorProto")]
pub struct FieldDescriptorProtoGraphQl {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub number: ::core::option::Option<i32>,
    pub label: ::core::option::Option<field_descriptor_proto::LabelGraphQl>,
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    pub r#type: ::core::option::Option<field_descriptor_proto::TypeGraphQl>,
    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as type_name.
    pub extendee: ::core::option::Option<::prost::alloc::string::String>,
    /// For numeric types, contains the original text representation of the value.
    /// For booleans, "true" or "false".
    /// For strings, contains the default text contents (not escaped in any way).
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    /// TODO(kenton):  Base-64 encode?
    pub default_value: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.  This field is a member of that oneof.
    pub oneof_index: ::core::option::Option<i32>,
    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    pub json_name: ::core::option::Option<::prost::alloc::string::String>,
    pub options: ::core::option::Option<FieldOptionsGraphQl>,
    /// If true, this is a proto3 "optional". When a proto3 field is optional, it
    /// tracks presence regardless of field type.
    ///
    /// When proto3_optional is true, this field must be belong to a oneof to
    /// signal to old proto3 clients that presence is tracked for this field. This
    /// oneof is known as a "synthetic" oneof, and this field must be its sole
    /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
    /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
    /// oneofs must be ordered after all "real" oneofs.
    ///
    /// For message fields, proto3_optional doesn't create any semantic change,
    /// since non-repeated message fields always track presence. However it still
    /// indicates the semantic detail of whether the user wrote "optional" or not.
    /// This can be useful for round-tripping the .proto file. For consistency we
    /// give message fields a synthetic oneof also, even though it is not required
    /// to track presence. This is especially important because the parser can't
    /// tell if a field is a message or an enum, so it must always create a
    /// synthetic oneof.
    ///
    /// Proto2 optional fields do not set this flag, because they already indicate
    /// optional with `LABEL_OPTIONAL`.
    pub proto3_optional: ::core::option::Option<bool>,
}
/// Describes a field within a message.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FieldDescriptorProtoInput")]
pub struct FieldDescriptorProtoGraphQlInput {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub number: ::core::option::Option<i32>,
    pub label: ::core::option::Option<field_descriptor_proto::LabelGraphQlInput>,
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    pub r#type: ::core::option::Option<field_descriptor_proto::TypeGraphQlInput>,
    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as type_name.
    pub extendee: ::core::option::Option<::prost::alloc::string::String>,
    /// For numeric types, contains the original text representation of the value.
    /// For booleans, "true" or "false".
    /// For strings, contains the default text contents (not escaped in any way).
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    /// TODO(kenton):  Base-64 encode?
    pub default_value: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.  This field is a member of that oneof.
    pub oneof_index: ::core::option::Option<i32>,
    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    pub json_name: ::core::option::Option<::prost::alloc::string::String>,
    pub options: ::core::option::Option<FieldOptionsGraphQlInput>,
    /// If true, this is a proto3 "optional". When a proto3 field is optional, it
    /// tracks presence regardless of field type.
    ///
    /// When proto3_optional is true, this field must be belong to a oneof to
    /// signal to old proto3 clients that presence is tracked for this field. This
    /// oneof is known as a "synthetic" oneof, and this field must be its sole
    /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
    /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
    /// oneofs must be ordered after all "real" oneofs.
    ///
    /// For message fields, proto3_optional doesn't create any semantic change,
    /// since non-repeated message fields always track presence. However it still
    /// indicates the semantic detail of whether the user wrote "optional" or not.
    /// This can be useful for round-tripping the .proto file. For consistency we
    /// give message fields a synthetic oneof also, even though it is not required
    /// to track presence. This is especially important because the parser can't
    /// tell if a field is a message or an enum, so it must always create a
    /// synthetic oneof.
    ///
    /// Proto2 optional fields do not set this flag, because they already indicate
    /// optional with `LABEL_OPTIONAL`.
    pub proto3_optional: ::core::option::Option<bool>,
}
#[allow(clippy::useless_conversion)]
impl From<FieldDescriptorProto> for FieldDescriptorProtoGraphQl {
    fn from(other: FieldDescriptorProto) -> Self {
        let label = if other.label.is_some() {
            Some(other.label())
        } else {
            None
        };
        let r#type = if other.r#type.is_some() {
            Some(other.r#type())
        } else {
            None
        };
        let FieldDescriptorProto {
            name,
            number,
            type_name,
            extendee,
            default_value,
            oneof_index,
            json_name,
            options,
            proto3_optional,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            label: label.map(Into::into),
            r#type: r#type.map(Into::into),
            type_name: type_name.map(Into::into),
            extendee: extendee.map(Into::into),
            default_value: default_value.map(Into::into),
            oneof_index: oneof_index.map(Into::into),
            json_name: json_name.map(Into::into),
            options: options.map(Into::into),
            proto3_optional: proto3_optional.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldDescriptorProtoGraphQl> for FieldDescriptorProto {
    fn from(other: FieldDescriptorProtoGraphQl) -> Self {
        let FieldDescriptorProtoGraphQl {
            name,
            number,
            label,
            r#type,
            type_name,
            extendee,
            default_value,
            oneof_index,
            json_name,
            options,
            proto3_optional,
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            label: label.map(|b| b as i32),
            r#type: r#type.map(|b| b as i32),
            type_name: type_name.map(Into::into),
            extendee: extendee.map(Into::into),
            default_value: default_value.map(Into::into),
            oneof_index: oneof_index.map(Into::into),
            json_name: json_name.map(Into::into),
            options: options.map(Into::into),
            proto3_optional: proto3_optional.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldDescriptorProto> for FieldDescriptorProtoGraphQlInput {
    fn from(other: FieldDescriptorProto) -> Self {
        let label = if other.label.is_some() {
            Some(other.label())
        } else {
            None
        };
        let r#type = if other.r#type.is_some() {
            Some(other.r#type())
        } else {
            None
        };
        let FieldDescriptorProto {
            name,
            number,
            type_name,
            extendee,
            default_value,
            oneof_index,
            json_name,
            options,
            proto3_optional,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            label: label.map(Into::into),
            r#type: r#type.map(Into::into),
            type_name: type_name.map(Into::into),
            extendee: extendee.map(Into::into),
            default_value: default_value.map(Into::into),
            oneof_index: oneof_index.map(Into::into),
            json_name: json_name.map(Into::into),
            options: options.map(Into::into),
            proto3_optional: proto3_optional.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldDescriptorProtoGraphQlInput> for FieldDescriptorProto {
    fn from(other: FieldDescriptorProtoGraphQlInput) -> Self {
        let FieldDescriptorProtoGraphQlInput {
            name,
            number,
            label,
            r#type,
            type_name,
            extendee,
            default_value,
            oneof_index,
            json_name,
            options,
            proto3_optional,
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            label: label.map(|b| b as i32),
            r#type: r#type.map(|b| b as i32),
            type_name: type_name.map(Into::into),
            extendee: extendee.map(Into::into),
            default_value: default_value.map(Into::into),
            oneof_index: oneof_index.map(Into::into),
            json_name: json_name.map(Into::into),
            options: options.map(Into::into),
            proto3_optional: proto3_optional.map(Into::into),
        }
    }
}
/// Describes a oneof.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "OneofDescriptorProto")]
pub struct OneofDescriptorProtoGraphQl {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub options: ::core::option::Option<OneofOptionsGraphQl>,
}
/// Describes a oneof.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "OneofDescriptorProtoInput")]
pub struct OneofDescriptorProtoGraphQlInput {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub options: ::core::option::Option<OneofOptionsGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<OneofDescriptorProto> for OneofDescriptorProtoGraphQl {
    fn from(other: OneofDescriptorProto) -> Self {
        let OneofDescriptorProto { name, options, .. } = other;
        Self {
            name: name.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OneofDescriptorProtoGraphQl> for OneofDescriptorProto {
    fn from(other: OneofDescriptorProtoGraphQl) -> Self {
        let OneofDescriptorProtoGraphQl { name, options } = other;
        Self {
            name: name.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OneofDescriptorProto> for OneofDescriptorProtoGraphQlInput {
    fn from(other: OneofDescriptorProto) -> Self {
        let OneofDescriptorProto { name, options, .. } = other;
        Self {
            name: name.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OneofDescriptorProtoGraphQlInput> for OneofDescriptorProto {
    fn from(other: OneofDescriptorProtoGraphQlInput) -> Self {
        let OneofDescriptorProtoGraphQlInput { name, options } = other;
        Self {
            name: name.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
/// Describes an enum type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumDescriptorProto")]
pub struct EnumDescriptorProtoGraphQl {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub value: ::prost::alloc::vec::Vec<EnumValueDescriptorProtoGraphQl>,
    pub options: ::core::option::Option<EnumOptionsGraphQl>,
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.
    pub reserved_range: ::prost::alloc::vec::Vec<enum_descriptor_proto::EnumReservedRangeGraphQl>,
    /// Reserved enum value names, which may not be reused. A given name may only
    /// be reserved once.
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Describes an enum type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumDescriptorProtoInput")]
pub struct EnumDescriptorProtoGraphQlInput {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub value: ::prost::alloc::vec::Vec<EnumValueDescriptorProtoGraphQlInput>,
    pub options: ::core::option::Option<EnumOptionsGraphQlInput>,
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.
    pub reserved_range:
        ::prost::alloc::vec::Vec<enum_descriptor_proto::EnumReservedRangeGraphQlInput>,
    /// Reserved enum value names, which may not be reused. A given name may only
    /// be reserved once.
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::useless_conversion)]
impl From<EnumDescriptorProto> for EnumDescriptorProtoGraphQl {
    fn from(other: EnumDescriptorProto) -> Self {
        let EnumDescriptorProto {
            name,
            value,
            options,
            reserved_range,
            reserved_name,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            value: value.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumDescriptorProtoGraphQl> for EnumDescriptorProto {
    fn from(other: EnumDescriptorProtoGraphQl) -> Self {
        let EnumDescriptorProtoGraphQl {
            name,
            value,
            options,
            reserved_range,
            reserved_name,
        } = other;
        Self {
            name: name.map(Into::into),
            value: value.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumDescriptorProto> for EnumDescriptorProtoGraphQlInput {
    fn from(other: EnumDescriptorProto) -> Self {
        let EnumDescriptorProto {
            name,
            value,
            options,
            reserved_range,
            reserved_name,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            value: value.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumDescriptorProtoGraphQlInput> for EnumDescriptorProto {
    fn from(other: EnumDescriptorProtoGraphQlInput) -> Self {
        let EnumDescriptorProtoGraphQlInput {
            name,
            value,
            options,
            reserved_range,
            reserved_name,
        } = other;
        Self {
            name: name.map(Into::into),
            value: value.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
            reserved_range: reserved_range.into_iter().map(Into::into).collect(),
            reserved_name: reserved_name.into_iter().map(Into::into).collect(),
        }
    }
}
/// Describes a value within an enum.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumValueDescriptorProto")]
pub struct EnumValueDescriptorProtoGraphQl {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub number: ::core::option::Option<i32>,
    pub options: ::core::option::Option<EnumValueOptionsGraphQl>,
}
/// Describes a value within an enum.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumValueDescriptorProtoInput")]
pub struct EnumValueDescriptorProtoGraphQlInput {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub number: ::core::option::Option<i32>,
    pub options: ::core::option::Option<EnumValueOptionsGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueDescriptorProto> for EnumValueDescriptorProtoGraphQl {
    fn from(other: EnumValueDescriptorProto) -> Self {
        let EnumValueDescriptorProto {
            name,
            number,
            options,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueDescriptorProtoGraphQl> for EnumValueDescriptorProto {
    fn from(other: EnumValueDescriptorProtoGraphQl) -> Self {
        let EnumValueDescriptorProtoGraphQl {
            name,
            number,
            options,
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueDescriptorProto> for EnumValueDescriptorProtoGraphQlInput {
    fn from(other: EnumValueDescriptorProto) -> Self {
        let EnumValueDescriptorProto {
            name,
            number,
            options,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueDescriptorProtoGraphQlInput> for EnumValueDescriptorProto {
    fn from(other: EnumValueDescriptorProtoGraphQlInput) -> Self {
        let EnumValueDescriptorProtoGraphQlInput {
            name,
            number,
            options,
        } = other;
        Self {
            name: name.map(Into::into),
            number: number.map(Into::into),
            options: options.map(Into::into),
        }
    }
}
/// Describes a service.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ServiceDescriptorProto")]
pub struct ServiceDescriptorProtoGraphQl {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub method: ::prost::alloc::vec::Vec<MethodDescriptorProtoGraphQl>,
    pub options: ::core::option::Option<ServiceOptionsGraphQl>,
}
/// Describes a service.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ServiceDescriptorProtoInput")]
pub struct ServiceDescriptorProtoGraphQlInput {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    pub method: ::prost::alloc::vec::Vec<MethodDescriptorProtoGraphQlInput>,
    pub options: ::core::option::Option<ServiceOptionsGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ServiceDescriptorProto> for ServiceDescriptorProtoGraphQl {
    fn from(other: ServiceDescriptorProto) -> Self {
        let ServiceDescriptorProto {
            name,
            method,
            options,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            method: method.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ServiceDescriptorProtoGraphQl> for ServiceDescriptorProto {
    fn from(other: ServiceDescriptorProtoGraphQl) -> Self {
        let ServiceDescriptorProtoGraphQl {
            name,
            method,
            options,
        } = other;
        Self {
            name: name.map(Into::into),
            method: method.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ServiceDescriptorProto> for ServiceDescriptorProtoGraphQlInput {
    fn from(other: ServiceDescriptorProto) -> Self {
        let ServiceDescriptorProto {
            name,
            method,
            options,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            method: method.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ServiceDescriptorProtoGraphQlInput> for ServiceDescriptorProto {
    fn from(other: ServiceDescriptorProtoGraphQlInput) -> Self {
        let ServiceDescriptorProtoGraphQlInput {
            name,
            method,
            options,
        } = other;
        Self {
            name: name.map(Into::into),
            method: method.into_iter().map(Into::into).collect(),
            options: options.map(Into::into),
        }
    }
}
/// Describes a method of a service.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "MethodDescriptorProto")]
pub struct MethodDescriptorProtoGraphQl {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Input and output type names.  These are resolved in the same way as
    /// FieldDescriptorProto.type_name, but must refer to a message type.
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    pub options: ::core::option::Option<MethodOptionsGraphQl>,
    /// Identifies if client streams multiple client messages
    pub client_streaming: ::core::option::Option<bool>,
    /// Identifies if server streams multiple server messages
    pub server_streaming: ::core::option::Option<bool>,
}
/// Describes a method of a service.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "MethodDescriptorProtoInput")]
pub struct MethodDescriptorProtoGraphQlInput {
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Input and output type names.  These are resolved in the same way as
    /// FieldDescriptorProto.type_name, but must refer to a message type.
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    pub options: ::core::option::Option<MethodOptionsGraphQlInput>,
    /// Identifies if client streams multiple client messages
    pub client_streaming: ::core::option::Option<bool>,
    /// Identifies if server streams multiple server messages
    pub server_streaming: ::core::option::Option<bool>,
}
#[allow(clippy::useless_conversion)]
impl From<MethodDescriptorProto> for MethodDescriptorProtoGraphQl {
    fn from(other: MethodDescriptorProto) -> Self {
        let MethodDescriptorProto {
            name,
            input_type,
            output_type,
            options,
            client_streaming,
            server_streaming,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            input_type: input_type.map(Into::into),
            output_type: output_type.map(Into::into),
            options: options.map(Into::into),
            client_streaming: client_streaming.map(Into::into),
            server_streaming: server_streaming.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodDescriptorProtoGraphQl> for MethodDescriptorProto {
    fn from(other: MethodDescriptorProtoGraphQl) -> Self {
        let MethodDescriptorProtoGraphQl {
            name,
            input_type,
            output_type,
            options,
            client_streaming,
            server_streaming,
        } = other;
        Self {
            name: name.map(Into::into),
            input_type: input_type.map(Into::into),
            output_type: output_type.map(Into::into),
            options: options.map(Into::into),
            client_streaming: client_streaming.map(Into::into),
            server_streaming: server_streaming.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodDescriptorProto> for MethodDescriptorProtoGraphQlInput {
    fn from(other: MethodDescriptorProto) -> Self {
        let MethodDescriptorProto {
            name,
            input_type,
            output_type,
            options,
            client_streaming,
            server_streaming,
            ..
        } = other;
        Self {
            name: name.map(Into::into),
            input_type: input_type.map(Into::into),
            output_type: output_type.map(Into::into),
            options: options.map(Into::into),
            client_streaming: client_streaming.map(Into::into),
            server_streaming: server_streaming.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodDescriptorProtoGraphQlInput> for MethodDescriptorProto {
    fn from(other: MethodDescriptorProtoGraphQlInput) -> Self {
        let MethodDescriptorProtoGraphQlInput {
            name,
            input_type,
            output_type,
            options,
            client_streaming,
            server_streaming,
        } = other;
        Self {
            name: name.map(Into::into),
            input_type: input_type.map(Into::into),
            output_type: output_type.map(Into::into),
            options: options.map(Into::into),
            client_streaming: client_streaming.map(Into::into),
            server_streaming: server_streaming.map(Into::into),
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
#[graphql(name = "FileOptions")]
pub struct FileOptionsGraphQl {
    /// Sets the Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    pub java_package: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, all the classes from the .proto file are wrapped in a single
    /// outer class with the given name.  This applies to both Proto1
    /// (equivalent to the old "--one_java_file" option) and Proto2 (where
    /// a .proto always translates to a single class, but you may want to
    /// explicitly choose the class name).
    pub java_outer_classname: ::core::option::Option<::prost::alloc::string::String>,
    /// If set true, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the outer class
    /// named by java_outer_classname.  However, the outer class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    pub java_multiple_files: ::core::option::Option<bool>,
    /// This option does nothing.
    #[deprecated]
    #[graphql(deprecation)]
    pub java_generate_equals_and_hash: ::core::option::Option<bool>,
    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    /// This option has no effect on when used with the lite runtime.
    pub java_string_check_utf8: ::core::option::Option<bool>,
    pub optimize_for: ::core::option::Option<file_options::OptimizeModeGraphQl>,
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///   - The basename of the package import path, if provided.
    ///   - Otherwise, the package statement in the .proto file, if present.
    ///   - Otherwise, the basename of the .proto file, without extension.
    pub go_package: ::core::option::Option<::prost::alloc::string::String>,
    /// Should generic services be generated in each language?  "Generic" services
    /// are not specific to any particular RPC system.  They are generated by the
    /// main code generators in each language (without additional plugins).
    /// Generic services were the only kind of service generation supported by
    /// early versions of google.protobuf.
    ///
    /// Generic services are now considered deprecated in favor of using plugins
    /// that generate code specific to your particular RPC system.  Therefore,
    /// these default to false.  Old code which depends on generic services should
    /// explicitly set them to true.
    pub cc_generic_services: ::core::option::Option<bool>,
    pub java_generic_services: ::core::option::Option<bool>,
    pub py_generic_services: ::core::option::Option<bool>,
    pub php_generic_services: ::core::option::Option<bool>,
    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    pub deprecated: ::core::option::Option<bool>,
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    pub cc_enable_arenas: ::core::option::Option<bool>,
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    pub objc_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace for generated classes; defaults to the package.
    pub csharp_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    pub swift_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    pub php_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    pub php_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    pub php_metadata_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    pub ruby_package: ::core::option::Option<::prost::alloc::string::String>,
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FileOptionsInput")]
pub struct FileOptionsGraphQlInput {
    /// Sets the Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    pub java_package: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, all the classes from the .proto file are wrapped in a single
    /// outer class with the given name.  This applies to both Proto1
    /// (equivalent to the old "--one_java_file" option) and Proto2 (where
    /// a .proto always translates to a single class, but you may want to
    /// explicitly choose the class name).
    pub java_outer_classname: ::core::option::Option<::prost::alloc::string::String>,
    /// If set true, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the outer class
    /// named by java_outer_classname.  However, the outer class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    pub java_multiple_files: ::core::option::Option<bool>,
    /// This option does nothing.
    #[deprecated]
    pub java_generate_equals_and_hash: ::core::option::Option<bool>,
    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    /// This option has no effect on when used with the lite runtime.
    pub java_string_check_utf8: ::core::option::Option<bool>,
    pub optimize_for: ::core::option::Option<file_options::OptimizeModeGraphQlInput>,
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///   - The basename of the package import path, if provided.
    ///   - Otherwise, the package statement in the .proto file, if present.
    ///   - Otherwise, the basename of the .proto file, without extension.
    pub go_package: ::core::option::Option<::prost::alloc::string::String>,
    /// Should generic services be generated in each language?  "Generic" services
    /// are not specific to any particular RPC system.  They are generated by the
    /// main code generators in each language (without additional plugins).
    /// Generic services were the only kind of service generation supported by
    /// early versions of google.protobuf.
    ///
    /// Generic services are now considered deprecated in favor of using plugins
    /// that generate code specific to your particular RPC system.  Therefore,
    /// these default to false.  Old code which depends on generic services should
    /// explicitly set them to true.
    pub cc_generic_services: ::core::option::Option<bool>,
    pub java_generic_services: ::core::option::Option<bool>,
    pub py_generic_services: ::core::option::Option<bool>,
    pub php_generic_services: ::core::option::Option<bool>,
    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    pub deprecated: ::core::option::Option<bool>,
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    pub cc_enable_arenas: ::core::option::Option<bool>,
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    pub objc_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace for generated classes; defaults to the package.
    pub csharp_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    pub swift_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    pub php_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    pub php_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    pub php_metadata_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    pub ruby_package: ::core::option::Option<::prost::alloc::string::String>,
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<FileOptions> for FileOptionsGraphQl {
    fn from(other: FileOptions) -> Self {
        let optimize_for = if other.optimize_for.is_some() {
            Some(other.optimize_for())
        } else {
            None
        };
        let FileOptions {
            java_package,
            java_outer_classname,
            java_multiple_files,
            java_generate_equals_and_hash,
            java_string_check_utf8,
            go_package,
            cc_generic_services,
            java_generic_services,
            py_generic_services,
            php_generic_services,
            deprecated,
            cc_enable_arenas,
            objc_class_prefix,
            csharp_namespace,
            swift_prefix,
            php_class_prefix,
            php_namespace,
            php_metadata_namespace,
            ruby_package,
            uninterpreted_option,
            ..
        } = other;
        Self {
            java_package: java_package.map(Into::into),
            java_outer_classname: java_outer_classname.map(Into::into),
            java_multiple_files: java_multiple_files.map(Into::into),
            java_generate_equals_and_hash: java_generate_equals_and_hash.map(Into::into),
            java_string_check_utf8: java_string_check_utf8.map(Into::into),
            optimize_for: optimize_for.map(Into::into),
            go_package: go_package.map(Into::into),
            cc_generic_services: cc_generic_services.map(Into::into),
            java_generic_services: java_generic_services.map(Into::into),
            py_generic_services: py_generic_services.map(Into::into),
            php_generic_services: php_generic_services.map(Into::into),
            deprecated: deprecated.map(Into::into),
            cc_enable_arenas: cc_enable_arenas.map(Into::into),
            objc_class_prefix: objc_class_prefix.map(Into::into),
            csharp_namespace: csharp_namespace.map(Into::into),
            swift_prefix: swift_prefix.map(Into::into),
            php_class_prefix: php_class_prefix.map(Into::into),
            php_namespace: php_namespace.map(Into::into),
            php_metadata_namespace: php_metadata_namespace.map(Into::into),
            ruby_package: ruby_package.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileOptionsGraphQl> for FileOptions {
    fn from(other: FileOptionsGraphQl) -> Self {
        let FileOptionsGraphQl {
            java_package,
            java_outer_classname,
            java_multiple_files,
            java_generate_equals_and_hash,
            java_string_check_utf8,
            optimize_for,
            go_package,
            cc_generic_services,
            java_generic_services,
            py_generic_services,
            php_generic_services,
            deprecated,
            cc_enable_arenas,
            objc_class_prefix,
            csharp_namespace,
            swift_prefix,
            php_class_prefix,
            php_namespace,
            php_metadata_namespace,
            ruby_package,
            uninterpreted_option,
        } = other;
        Self {
            java_package: java_package.map(Into::into),
            java_outer_classname: java_outer_classname.map(Into::into),
            java_multiple_files: java_multiple_files.map(Into::into),
            java_generate_equals_and_hash: java_generate_equals_and_hash.map(Into::into),
            java_string_check_utf8: java_string_check_utf8.map(Into::into),
            optimize_for: optimize_for.map(|b| b as i32),
            go_package: go_package.map(Into::into),
            cc_generic_services: cc_generic_services.map(Into::into),
            java_generic_services: java_generic_services.map(Into::into),
            py_generic_services: py_generic_services.map(Into::into),
            php_generic_services: php_generic_services.map(Into::into),
            deprecated: deprecated.map(Into::into),
            cc_enable_arenas: cc_enable_arenas.map(Into::into),
            objc_class_prefix: objc_class_prefix.map(Into::into),
            csharp_namespace: csharp_namespace.map(Into::into),
            swift_prefix: swift_prefix.map(Into::into),
            php_class_prefix: php_class_prefix.map(Into::into),
            php_namespace: php_namespace.map(Into::into),
            php_metadata_namespace: php_metadata_namespace.map(Into::into),
            ruby_package: ruby_package.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileOptions> for FileOptionsGraphQlInput {
    fn from(other: FileOptions) -> Self {
        let optimize_for = if other.optimize_for.is_some() {
            Some(other.optimize_for())
        } else {
            None
        };
        let FileOptions {
            java_package,
            java_outer_classname,
            java_multiple_files,
            java_generate_equals_and_hash,
            java_string_check_utf8,
            go_package,
            cc_generic_services,
            java_generic_services,
            py_generic_services,
            php_generic_services,
            deprecated,
            cc_enable_arenas,
            objc_class_prefix,
            csharp_namespace,
            swift_prefix,
            php_class_prefix,
            php_namespace,
            php_metadata_namespace,
            ruby_package,
            uninterpreted_option,
            ..
        } = other;
        Self {
            java_package: java_package.map(Into::into),
            java_outer_classname: java_outer_classname.map(Into::into),
            java_multiple_files: java_multiple_files.map(Into::into),
            java_generate_equals_and_hash: java_generate_equals_and_hash.map(Into::into),
            java_string_check_utf8: java_string_check_utf8.map(Into::into),
            optimize_for: optimize_for.map(Into::into),
            go_package: go_package.map(Into::into),
            cc_generic_services: cc_generic_services.map(Into::into),
            java_generic_services: java_generic_services.map(Into::into),
            py_generic_services: py_generic_services.map(Into::into),
            php_generic_services: php_generic_services.map(Into::into),
            deprecated: deprecated.map(Into::into),
            cc_enable_arenas: cc_enable_arenas.map(Into::into),
            objc_class_prefix: objc_class_prefix.map(Into::into),
            csharp_namespace: csharp_namespace.map(Into::into),
            swift_prefix: swift_prefix.map(Into::into),
            php_class_prefix: php_class_prefix.map(Into::into),
            php_namespace: php_namespace.map(Into::into),
            php_metadata_namespace: php_metadata_namespace.map(Into::into),
            ruby_package: ruby_package.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FileOptionsGraphQlInput> for FileOptions {
    fn from(other: FileOptionsGraphQlInput) -> Self {
        let FileOptionsGraphQlInput {
            java_package,
            java_outer_classname,
            java_multiple_files,
            java_generate_equals_and_hash,
            java_string_check_utf8,
            optimize_for,
            go_package,
            cc_generic_services,
            java_generic_services,
            py_generic_services,
            php_generic_services,
            deprecated,
            cc_enable_arenas,
            objc_class_prefix,
            csharp_namespace,
            swift_prefix,
            php_class_prefix,
            php_namespace,
            php_metadata_namespace,
            ruby_package,
            uninterpreted_option,
        } = other;
        Self {
            java_package: java_package.map(Into::into),
            java_outer_classname: java_outer_classname.map(Into::into),
            java_multiple_files: java_multiple_files.map(Into::into),
            java_generate_equals_and_hash: java_generate_equals_and_hash.map(Into::into),
            java_string_check_utf8: java_string_check_utf8.map(Into::into),
            optimize_for: optimize_for.map(|b| b as i32),
            go_package: go_package.map(Into::into),
            cc_generic_services: cc_generic_services.map(Into::into),
            java_generic_services: java_generic_services.map(Into::into),
            py_generic_services: py_generic_services.map(Into::into),
            php_generic_services: php_generic_services.map(Into::into),
            deprecated: deprecated.map(Into::into),
            cc_enable_arenas: cc_enable_arenas.map(Into::into),
            objc_class_prefix: objc_class_prefix.map(Into::into),
            csharp_namespace: csharp_namespace.map(Into::into),
            swift_prefix: swift_prefix.map(Into::into),
            php_class_prefix: php_class_prefix.map(Into::into),
            php_namespace: php_namespace.map(Into::into),
            php_metadata_namespace: php_metadata_namespace.map(Into::into),
            ruby_package: ruby_package.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
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
#[graphql(name = "MessageOptions")]
pub struct MessageOptionsGraphQl {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    ///   message Foo {
    ///     option message_set_wire_format = true;
    ///     extensions 4 to max;
    ///   }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    pub message_set_wire_format: ::core::option::Option<bool>,
    /// Disables the generation of the standard "descriptor()" accessor, which can
    /// conflict with a field of the same name.  This is meant to make migration
    /// from proto1 easier; new code should avoid fields named "descriptor".
    pub no_standard_descriptor_accessor: ::core::option::Option<bool>,
    /// Is this message deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the message, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating messages.
    pub deprecated: ::core::option::Option<bool>,
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    ///     map<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    ///     message MapFieldEntry {
    ///         option map_entry = true;
    ///         optional KeyType key = 1;
    ///         optional ValueType value = 2;
    ///     }
    ///     repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    pub map_entry: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "MessageOptionsInput")]
pub struct MessageOptionsGraphQlInput {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    ///   message Foo {
    ///     option message_set_wire_format = true;
    ///     extensions 4 to max;
    ///   }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    pub message_set_wire_format: ::core::option::Option<bool>,
    /// Disables the generation of the standard "descriptor()" accessor, which can
    /// conflict with a field of the same name.  This is meant to make migration
    /// from proto1 easier; new code should avoid fields named "descriptor".
    pub no_standard_descriptor_accessor: ::core::option::Option<bool>,
    /// Is this message deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the message, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating messages.
    pub deprecated: ::core::option::Option<bool>,
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    ///     map<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    ///     message MapFieldEntry {
    ///         option map_entry = true;
    ///         optional KeyType key = 1;
    ///         optional ValueType value = 2;
    ///     }
    ///     repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    pub map_entry: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<MessageOptions> for MessageOptionsGraphQl {
    fn from(other: MessageOptions) -> Self {
        let MessageOptions {
            message_set_wire_format,
            no_standard_descriptor_accessor,
            deprecated,
            map_entry,
            uninterpreted_option,
            ..
        } = other;
        Self {
            message_set_wire_format: message_set_wire_format.map(Into::into),
            no_standard_descriptor_accessor: no_standard_descriptor_accessor.map(Into::into),
            deprecated: deprecated.map(Into::into),
            map_entry: map_entry.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MessageOptionsGraphQl> for MessageOptions {
    fn from(other: MessageOptionsGraphQl) -> Self {
        let MessageOptionsGraphQl {
            message_set_wire_format,
            no_standard_descriptor_accessor,
            deprecated,
            map_entry,
            uninterpreted_option,
        } = other;
        Self {
            message_set_wire_format: message_set_wire_format.map(Into::into),
            no_standard_descriptor_accessor: no_standard_descriptor_accessor.map(Into::into),
            deprecated: deprecated.map(Into::into),
            map_entry: map_entry.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MessageOptions> for MessageOptionsGraphQlInput {
    fn from(other: MessageOptions) -> Self {
        let MessageOptions {
            message_set_wire_format,
            no_standard_descriptor_accessor,
            deprecated,
            map_entry,
            uninterpreted_option,
            ..
        } = other;
        Self {
            message_set_wire_format: message_set_wire_format.map(Into::into),
            no_standard_descriptor_accessor: no_standard_descriptor_accessor.map(Into::into),
            deprecated: deprecated.map(Into::into),
            map_entry: map_entry.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MessageOptionsGraphQlInput> for MessageOptions {
    fn from(other: MessageOptionsGraphQlInput) -> Self {
        let MessageOptionsGraphQlInput {
            message_set_wire_format,
            no_standard_descriptor_accessor,
            deprecated,
            map_entry,
            uninterpreted_option,
        } = other;
        Self {
            message_set_wire_format: message_set_wire_format.map(Into::into),
            no_standard_descriptor_accessor: no_standard_descriptor_accessor.map(Into::into),
            deprecated: deprecated.map(Into::into),
            map_entry: map_entry.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
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
#[graphql(name = "FieldOptions")]
pub struct FieldOptionsGraphQl {
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    pub ctype: ::core::option::Option<field_options::CTypeGraphQl>,
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub packed: ::core::option::Option<bool>,
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub jstype: ::core::option::Option<field_options::JsTypeGraphQl>,
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub lazy: ::core::option::Option<bool>,
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub deprecated: ::core::option::Option<bool>,
    /// For Google-internal migration only. Do not use.
    pub weak: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FieldOptionsInput")]
pub struct FieldOptionsGraphQlInput {
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    pub ctype: ::core::option::Option<field_options::CTypeGraphQlInput>,
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub packed: ::core::option::Option<bool>,
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub jstype: ::core::option::Option<field_options::JsTypeGraphQlInput>,
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub lazy: ::core::option::Option<bool>,
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub deprecated: ::core::option::Option<bool>,
    /// For Google-internal migration only. Do not use.
    pub weak: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<FieldOptions> for FieldOptionsGraphQl {
    fn from(other: FieldOptions) -> Self {
        let ctype = if other.ctype.is_some() {
            Some(other.ctype())
        } else {
            None
        };
        let jstype = if other.jstype.is_some() {
            Some(other.jstype())
        } else {
            None
        };
        let FieldOptions {
            packed,
            lazy,
            deprecated,
            weak,
            uninterpreted_option,
            ..
        } = other;
        Self {
            ctype: ctype.map(Into::into),
            packed: packed.map(Into::into),
            jstype: jstype.map(Into::into),
            lazy: lazy.map(Into::into),
            deprecated: deprecated.map(Into::into),
            weak: weak.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldOptionsGraphQl> for FieldOptions {
    fn from(other: FieldOptionsGraphQl) -> Self {
        let FieldOptionsGraphQl {
            ctype,
            packed,
            jstype,
            lazy,
            deprecated,
            weak,
            uninterpreted_option,
        } = other;
        Self {
            ctype: ctype.map(|b| b as i32),
            packed: packed.map(Into::into),
            jstype: jstype.map(|b| b as i32),
            lazy: lazy.map(Into::into),
            deprecated: deprecated.map(Into::into),
            weak: weak.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldOptions> for FieldOptionsGraphQlInput {
    fn from(other: FieldOptions) -> Self {
        let ctype = if other.ctype.is_some() {
            Some(other.ctype())
        } else {
            None
        };
        let jstype = if other.jstype.is_some() {
            Some(other.jstype())
        } else {
            None
        };
        let FieldOptions {
            packed,
            lazy,
            deprecated,
            weak,
            uninterpreted_option,
            ..
        } = other;
        Self {
            ctype: ctype.map(Into::into),
            packed: packed.map(Into::into),
            jstype: jstype.map(Into::into),
            lazy: lazy.map(Into::into),
            deprecated: deprecated.map(Into::into),
            weak: weak.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldOptionsGraphQlInput> for FieldOptions {
    fn from(other: FieldOptionsGraphQlInput) -> Self {
        let FieldOptionsGraphQlInput {
            ctype,
            packed,
            jstype,
            lazy,
            deprecated,
            weak,
            uninterpreted_option,
        } = other;
        Self {
            ctype: ctype.map(|b| b as i32),
            packed: packed.map(Into::into),
            jstype: jstype.map(|b| b as i32),
            lazy: lazy.map(Into::into),
            deprecated: deprecated.map(Into::into),
            weak: weak.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
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
#[graphql(name = "OneofOptions")]
pub struct OneofOptionsGraphQl {
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "OneofOptionsInput")]
pub struct OneofOptionsGraphQlInput {
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<OneofOptions> for OneofOptionsGraphQl {
    fn from(other: OneofOptions) -> Self {
        let OneofOptions {
            uninterpreted_option,
            ..
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OneofOptionsGraphQl> for OneofOptions {
    fn from(other: OneofOptionsGraphQl) -> Self {
        let OneofOptionsGraphQl {
            uninterpreted_option,
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OneofOptions> for OneofOptionsGraphQlInput {
    fn from(other: OneofOptions) -> Self {
        let OneofOptions {
            uninterpreted_option,
            ..
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OneofOptionsGraphQlInput> for OneofOptions {
    fn from(other: OneofOptionsGraphQlInput) -> Self {
        let OneofOptionsGraphQlInput {
            uninterpreted_option,
        } = other;
        Self {
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
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
#[graphql(name = "EnumOptions")]
pub struct EnumOptionsGraphQl {
    /// Set this option to true to allow mapping different tag names to the same
    /// value.
    pub allow_alias: ::core::option::Option<bool>,
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumOptionsInput")]
pub struct EnumOptionsGraphQlInput {
    /// Set this option to true to allow mapping different tag names to the same
    /// value.
    pub allow_alias: ::core::option::Option<bool>,
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<EnumOptions> for EnumOptionsGraphQl {
    fn from(other: EnumOptions) -> Self {
        let EnumOptions {
            allow_alias,
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            allow_alias: allow_alias.map(Into::into),
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumOptionsGraphQl> for EnumOptions {
    fn from(other: EnumOptionsGraphQl) -> Self {
        let EnumOptionsGraphQl {
            allow_alias,
            deprecated,
            uninterpreted_option,
        } = other;
        Self {
            allow_alias: allow_alias.map(Into::into),
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumOptions> for EnumOptionsGraphQlInput {
    fn from(other: EnumOptions) -> Self {
        let EnumOptions {
            allow_alias,
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            allow_alias: allow_alias.map(Into::into),
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumOptionsGraphQlInput> for EnumOptions {
    fn from(other: EnumOptionsGraphQlInput) -> Self {
        let EnumOptionsGraphQlInput {
            allow_alias,
            deprecated,
            uninterpreted_option,
        } = other;
        Self {
            allow_alias: allow_alias.map(Into::into),
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
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
#[graphql(name = "EnumValueOptions")]
pub struct EnumValueOptionsGraphQl {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumValueOptionsInput")]
pub struct EnumValueOptionsGraphQlInput {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueOptions> for EnumValueOptionsGraphQl {
    fn from(other: EnumValueOptions) -> Self {
        let EnumValueOptions {
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueOptionsGraphQl> for EnumValueOptions {
    fn from(other: EnumValueOptionsGraphQl) -> Self {
        let EnumValueOptionsGraphQl {
            deprecated,
            uninterpreted_option,
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueOptions> for EnumValueOptionsGraphQlInput {
    fn from(other: EnumValueOptions) -> Self {
        let EnumValueOptions {
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueOptionsGraphQlInput> for EnumValueOptions {
    fn from(other: EnumValueOptionsGraphQlInput) -> Self {
        let EnumValueOptionsGraphQlInput {
            deprecated,
            uninterpreted_option,
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
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
#[graphql(name = "ServiceOptions")]
pub struct ServiceOptionsGraphQl {
    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ServiceOptionsInput")]
pub struct ServiceOptionsGraphQlInput {
    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ServiceOptions> for ServiceOptionsGraphQl {
    fn from(other: ServiceOptions) -> Self {
        let ServiceOptions {
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ServiceOptionsGraphQl> for ServiceOptions {
    fn from(other: ServiceOptionsGraphQl) -> Self {
        let ServiceOptionsGraphQl {
            deprecated,
            uninterpreted_option,
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ServiceOptions> for ServiceOptionsGraphQlInput {
    fn from(other: ServiceOptions) -> Self {
        let ServiceOptions {
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ServiceOptionsGraphQlInput> for ServiceOptions {
    fn from(other: ServiceOptionsGraphQlInput) -> Self {
        let ServiceOptionsGraphQlInput {
            deprecated,
            uninterpreted_option,
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
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
#[graphql(name = "MethodOptions")]
pub struct MethodOptionsGraphQl {
    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    pub deprecated: ::core::option::Option<bool>,
    pub idempotency_level: ::core::option::Option<method_options::IdempotencyLevelGraphQl>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQl>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "MethodOptionsInput")]
pub struct MethodOptionsGraphQlInput {
    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    pub deprecated: ::core::option::Option<bool>,
    pub idempotency_level: ::core::option::Option<method_options::IdempotencyLevelGraphQlInput>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<MethodOptions> for MethodOptionsGraphQl {
    fn from(other: MethodOptions) -> Self {
        let idempotency_level = if other.idempotency_level.is_some() {
            Some(other.idempotency_level())
        } else {
            None
        };
        let MethodOptions {
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            idempotency_level: idempotency_level.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodOptionsGraphQl> for MethodOptions {
    fn from(other: MethodOptionsGraphQl) -> Self {
        let MethodOptionsGraphQl {
            deprecated,
            idempotency_level,
            uninterpreted_option,
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            idempotency_level: idempotency_level.map(|b| b as i32),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodOptions> for MethodOptionsGraphQlInput {
    fn from(other: MethodOptions) -> Self {
        let idempotency_level = if other.idempotency_level.is_some() {
            Some(other.idempotency_level())
        } else {
            None
        };
        let MethodOptions {
            deprecated,
            uninterpreted_option,
            ..
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            idempotency_level: idempotency_level.map(Into::into),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodOptionsGraphQlInput> for MethodOptions {
    fn from(other: MethodOptionsGraphQlInput) -> Self {
        let MethodOptionsGraphQlInput {
            deprecated,
            idempotency_level,
            uninterpreted_option,
        } = other;
        Self {
            deprecated: deprecated.map(Into::into),
            idempotency_level: idempotency_level.map(|b| b as i32),
            uninterpreted_option: uninterpreted_option.into_iter().map(Into::into).collect(),
        }
    }
}
/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "UninterpretedOption")]
pub struct UninterpretedOptionGraphQl {
    pub name: ::prost::alloc::vec::Vec<uninterpreted_option::NamePartGraphQl>,
    /// The value of the uninterpreted option, in whatever type the tokenizer
    /// identified it as during parsing. Exactly one of these should be set.
    pub identifier_value: ::core::option::Option<::prost::alloc::string::String>,
    pub positive_int_value: ::core::option::Option<u64>,
    pub negative_int_value: ::core::option::Option<i64>,
    pub double_value: ::core::option::Option<f64>,
    pub string_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    pub aggregate_value: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "UninterpretedOptionInput")]
pub struct UninterpretedOptionGraphQlInput {
    pub name: ::prost::alloc::vec::Vec<uninterpreted_option::NamePartGraphQlInput>,
    /// The value of the uninterpreted option, in whatever type the tokenizer
    /// identified it as during parsing. Exactly one of these should be set.
    pub identifier_value: ::core::option::Option<::prost::alloc::string::String>,
    pub positive_int_value: ::core::option::Option<u64>,
    pub negative_int_value: ::core::option::Option<i64>,
    pub double_value: ::core::option::Option<f64>,
    pub string_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    pub aggregate_value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::useless_conversion)]
impl From<UninterpretedOption> for UninterpretedOptionGraphQl {
    fn from(other: UninterpretedOption) -> Self {
        let UninterpretedOption {
            name,
            identifier_value,
            positive_int_value,
            negative_int_value,
            double_value,
            string_value,
            aggregate_value,
            ..
        } = other;
        Self {
            name: name.into_iter().map(Into::into).collect(),
            identifier_value: identifier_value.map(Into::into),
            positive_int_value: positive_int_value.map(Into::into),
            negative_int_value: negative_int_value.map(Into::into),
            double_value: double_value.map(Into::into),
            string_value: string_value.map(Into::into),
            aggregate_value: aggregate_value.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UninterpretedOptionGraphQl> for UninterpretedOption {
    fn from(other: UninterpretedOptionGraphQl) -> Self {
        let UninterpretedOptionGraphQl {
            name,
            identifier_value,
            positive_int_value,
            negative_int_value,
            double_value,
            string_value,
            aggregate_value,
        } = other;
        Self {
            name: name.into_iter().map(Into::into).collect(),
            identifier_value: identifier_value.map(Into::into),
            positive_int_value: positive_int_value.map(Into::into),
            negative_int_value: negative_int_value.map(Into::into),
            double_value: double_value.map(Into::into),
            string_value: string_value.map(Into::into),
            aggregate_value: aggregate_value.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UninterpretedOption> for UninterpretedOptionGraphQlInput {
    fn from(other: UninterpretedOption) -> Self {
        let UninterpretedOption {
            name,
            identifier_value,
            positive_int_value,
            negative_int_value,
            double_value,
            string_value,
            aggregate_value,
            ..
        } = other;
        Self {
            name: name.into_iter().map(Into::into).collect(),
            identifier_value: identifier_value.map(Into::into),
            positive_int_value: positive_int_value.map(Into::into),
            negative_int_value: negative_int_value.map(Into::into),
            double_value: double_value.map(Into::into),
            string_value: string_value.map(Into::into),
            aggregate_value: aggregate_value.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<UninterpretedOptionGraphQlInput> for UninterpretedOption {
    fn from(other: UninterpretedOptionGraphQlInput) -> Self {
        let UninterpretedOptionGraphQlInput {
            name,
            identifier_value,
            positive_int_value,
            negative_int_value,
            double_value,
            string_value,
            aggregate_value,
        } = other;
        Self {
            name: name.into_iter().map(Into::into).collect(),
            identifier_value: identifier_value.map(Into::into),
            positive_int_value: positive_int_value.map(Into::into),
            negative_int_value: negative_int_value.map(Into::into),
            double_value: double_value.map(Into::into),
            string_value: string_value.map(Into::into),
            aggregate_value: aggregate_value.map(Into::into),
        }
    }
}
/// Encapsulates information about the original source file from which a
/// FileDescriptorProto was generated.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "SourceCodeInfo")]
pub struct SourceCodeInfoGraphQl {
    /// A Location identifies a piece of source code in a .proto file which
    /// corresponds to a particular definition.  This information is intended
    /// to be useful to IDEs, code indexers, documentation generators, and similar
    /// tools.
    ///
    /// For example, say we have a file like:
    ///   message Foo {
    ///     optional string foo = 1;
    ///   }
    /// Let's look at just the field definition:
    ///   optional string foo = 1;
    ///   ^       ^^     ^^  ^  ^^^
    ///   a       bc     de  f  ghi
    /// We have the following locations:
    ///   span   path               represents
    ///   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
    ///   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
    ///   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
    ///   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
    ///   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).
    ///
    /// Notes:
    /// - A location may refer to a repeated field itself (i.e. not to any
    ///   particular index within it).  This is used whenever a set of elements are
    ///   logically enclosed in a single code segment.  For example, an entire
    ///   extend block (possibly containing multiple extension definitions) will
    ///   have an outer location whose path refers to the "extensions" repeated
    ///   field without an index.
    /// - Multiple locations may have the same path.  This happens when a single
    ///   logical declaration is spread out across multiple places.  The most
    ///   obvious example is the "extend" block again -- there may be multiple
    ///   extend blocks in the same scope, each of which will have the same path.
    /// - A location's span is not always a subset of its parent's span.  For
    ///   example, the "extendee" of an extension declaration appears at the
    ///   beginning of the "extend" block and is shared by all extensions within
    ///   the block.
    /// - Just because a location's span is a subset of some other location's span
    ///   does not mean that it is a descendant.  For example, a "group" defines
    ///   both a type and a field in a single declaration.  Thus, the locations
    ///   corresponding to the type and field and their components will overlap.
    /// - Code which tries to interpret locations should probably be designed to
    ///   ignore those that it doesn't understand, as more types of locations could
    ///   be recorded in the future.
    pub location: ::prost::alloc::vec::Vec<source_code_info::LocationGraphQl>,
}
/// Encapsulates information about the original source file from which a
/// FileDescriptorProto was generated.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "SourceCodeInfoInput")]
pub struct SourceCodeInfoGraphQlInput {
    /// A Location identifies a piece of source code in a .proto file which
    /// corresponds to a particular definition.  This information is intended
    /// to be useful to IDEs, code indexers, documentation generators, and similar
    /// tools.
    ///
    /// For example, say we have a file like:
    ///   message Foo {
    ///     optional string foo = 1;
    ///   }
    /// Let's look at just the field definition:
    ///   optional string foo = 1;
    ///   ^       ^^     ^^  ^  ^^^
    ///   a       bc     de  f  ghi
    /// We have the following locations:
    ///   span   path               represents
    ///   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
    ///   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
    ///   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
    ///   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
    ///   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).
    ///
    /// Notes:
    /// - A location may refer to a repeated field itself (i.e. not to any
    ///   particular index within it).  This is used whenever a set of elements are
    ///   logically enclosed in a single code segment.  For example, an entire
    ///   extend block (possibly containing multiple extension definitions) will
    ///   have an outer location whose path refers to the "extensions" repeated
    ///   field without an index.
    /// - Multiple locations may have the same path.  This happens when a single
    ///   logical declaration is spread out across multiple places.  The most
    ///   obvious example is the "extend" block again -- there may be multiple
    ///   extend blocks in the same scope, each of which will have the same path.
    /// - A location's span is not always a subset of its parent's span.  For
    ///   example, the "extendee" of an extension declaration appears at the
    ///   beginning of the "extend" block and is shared by all extensions within
    ///   the block.
    /// - Just because a location's span is a subset of some other location's span
    ///   does not mean that it is a descendant.  For example, a "group" defines
    ///   both a type and a field in a single declaration.  Thus, the locations
    ///   corresponding to the type and field and their components will overlap.
    /// - Code which tries to interpret locations should probably be designed to
    ///   ignore those that it doesn't understand, as more types of locations could
    ///   be recorded in the future.
    pub location: ::prost::alloc::vec::Vec<source_code_info::LocationGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<SourceCodeInfo> for SourceCodeInfoGraphQl {
    fn from(other: SourceCodeInfo) -> Self {
        let SourceCodeInfo { location, .. } = other;
        Self {
            location: location.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<SourceCodeInfoGraphQl> for SourceCodeInfo {
    fn from(other: SourceCodeInfoGraphQl) -> Self {
        let SourceCodeInfoGraphQl { location } = other;
        Self {
            location: location.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<SourceCodeInfo> for SourceCodeInfoGraphQlInput {
    fn from(other: SourceCodeInfo) -> Self {
        let SourceCodeInfo { location, .. } = other;
        Self {
            location: location.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<SourceCodeInfoGraphQlInput> for SourceCodeInfo {
    fn from(other: SourceCodeInfoGraphQlInput) -> Self {
        let SourceCodeInfoGraphQlInput { location } = other;
        Self {
            location: location.into_iter().map(Into::into).collect(),
        }
    }
}
/// Describes the relationship between generated code and its original source
/// file. A GeneratedCodeInfo message is associated with only one generated
/// source file, but may contain references to different source .proto files.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "GeneratedCodeInfo")]
pub struct GeneratedCodeInfoGraphQl {
    /// An Annotation connects some span of text in generated code to an element
    /// of its generating .proto file.
    pub annotation: ::prost::alloc::vec::Vec<generated_code_info::AnnotationGraphQl>,
}
/// Describes the relationship between generated code and its original source
/// file. A GeneratedCodeInfo message is associated with only one generated
/// source file, but may contain references to different source .proto files.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "GeneratedCodeInfoInput")]
pub struct GeneratedCodeInfoGraphQlInput {
    /// An Annotation connects some span of text in generated code to an element
    /// of its generating .proto file.
    pub annotation: ::prost::alloc::vec::Vec<generated_code_info::AnnotationGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<GeneratedCodeInfo> for GeneratedCodeInfoGraphQl {
    fn from(other: GeneratedCodeInfo) -> Self {
        let GeneratedCodeInfo { annotation, .. } = other;
        Self {
            annotation: annotation.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<GeneratedCodeInfoGraphQl> for GeneratedCodeInfo {
    fn from(other: GeneratedCodeInfoGraphQl) -> Self {
        let GeneratedCodeInfoGraphQl { annotation } = other;
        Self {
            annotation: annotation.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<GeneratedCodeInfo> for GeneratedCodeInfoGraphQlInput {
    fn from(other: GeneratedCodeInfo) -> Self {
        let GeneratedCodeInfo { annotation, .. } = other;
        Self {
            annotation: annotation.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<GeneratedCodeInfoGraphQlInput> for GeneratedCodeInfo {
    fn from(other: GeneratedCodeInfoGraphQlInput) -> Self {
        let GeneratedCodeInfoGraphQlInput { annotation } = other;
        Self {
            annotation: annotation.into_iter().map(Into::into).collect(),
        }
    }
}
/// `Any` contains an arbitrary serialized protocol buffer message along with a
/// URL that describes the type of the serialized message.
///
/// Protobuf library provides support to pack/unpack Any values in the form
/// of utility functions or additional generated methods of the Any type.
///
/// Example 1: Pack and unpack a message in C++.
///
///     Foo foo = ...;
///     Any any;
///     any.PackFrom(foo);
///     ...
///     if (any.UnpackTo(&foo)) {
///       ...
///     }
///
/// Example 2: Pack and unpack a message in Java.
///
///     Foo foo = ...;
///     Any any = Any.pack(foo);
///     ...
///     if (any.is(Foo.class)) {
///       foo = any.unpack(Foo.class);
///     }
///
///  Example 3: Pack and unpack a message in Python.
///
///     foo = Foo(...)
///     any = Any()
///     any.Pack(foo)
///     ...
///     if any.Is(Foo.DESCRIPTOR):
///       any.Unpack(foo)
///       ...
///
///  Example 4: Pack and unpack a message in Go
///
///      foo := &pb.Foo{...}
///      any, err := anypb.New(foo)
///      if err != nil {
///        ...
///      }
///      ...
///      foo := &pb.Foo{}
///      if err := any.UnmarshalTo(foo); err != nil {
///        ...
///      }
///
/// The pack methods provided by protobuf library will by default use
/// 'type.googleapis.com/full.type.name' as the type URL and the unpack
/// methods only use the fully qualified type name after the last '/'
/// in the type URL, for example "foo.bar.com/x/y.z" will yield type
/// name "y.z".
///
///
/// JSON
/// ====
/// The JSON representation of an `Any` value uses the regular
/// representation of the deserialized, embedded message, with an
/// additional field `@type` which contains the type URL. Example:
///
///     package google.profile;
///     message Person {
///       string first_name = 1;
///       string last_name = 2;
///     }
///
///     {
///       "@type": "type.googleapis.com/google.profile.Person",
///       "firstName": <string>,
///       "lastName": <string>
///     }
///
/// If the embedded message type is well-known and has a custom JSON
/// representation, that representation will be embedded adding a field
/// `value` which holds the custom JSON in addition to the `@type`
/// field. Example (for message [google.protobuf.Duration][]):
///
///     {
///       "@type": "type.googleapis.com/google.protobuf.Duration",
///       "value": "1.212s"
///     }
///
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Any")]
pub struct AnyGraphQl {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a [google.protobuf.Type][]
    ///   value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///   URL, or have them precompiled into a binary to avoid any
    ///   lookup. Therefore, binary compatibility needs to be preserved
    ///   on changes to types. (Use versioned type names to manage
    ///   breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    ///
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// `Any` contains an arbitrary serialized protocol buffer message along with a
/// URL that describes the type of the serialized message.
///
/// Protobuf library provides support to pack/unpack Any values in the form
/// of utility functions or additional generated methods of the Any type.
///
/// Example 1: Pack and unpack a message in C++.
///
///     Foo foo = ...;
///     Any any;
///     any.PackFrom(foo);
///     ...
///     if (any.UnpackTo(&foo)) {
///       ...
///     }
///
/// Example 2: Pack and unpack a message in Java.
///
///     Foo foo = ...;
///     Any any = Any.pack(foo);
///     ...
///     if (any.is(Foo.class)) {
///       foo = any.unpack(Foo.class);
///     }
///
///  Example 3: Pack and unpack a message in Python.
///
///     foo = Foo(...)
///     any = Any()
///     any.Pack(foo)
///     ...
///     if any.Is(Foo.DESCRIPTOR):
///       any.Unpack(foo)
///       ...
///
///  Example 4: Pack and unpack a message in Go
///
///      foo := &pb.Foo{...}
///      any, err := anypb.New(foo)
///      if err != nil {
///        ...
///      }
///      ...
///      foo := &pb.Foo{}
///      if err := any.UnmarshalTo(foo); err != nil {
///        ...
///      }
///
/// The pack methods provided by protobuf library will by default use
/// 'type.googleapis.com/full.type.name' as the type URL and the unpack
/// methods only use the fully qualified type name after the last '/'
/// in the type URL, for example "foo.bar.com/x/y.z" will yield type
/// name "y.z".
///
///
/// JSON
/// ====
/// The JSON representation of an `Any` value uses the regular
/// representation of the deserialized, embedded message, with an
/// additional field `@type` which contains the type URL. Example:
///
///     package google.profile;
///     message Person {
///       string first_name = 1;
///       string last_name = 2;
///     }
///
///     {
///       "@type": "type.googleapis.com/google.profile.Person",
///       "firstName": <string>,
///       "lastName": <string>
///     }
///
/// If the embedded message type is well-known and has a custom JSON
/// representation, that representation will be embedded adding a field
/// `value` which holds the custom JSON in addition to the `@type`
/// field. Example (for message [google.protobuf.Duration][]):
///
///     {
///       "@type": "type.googleapis.com/google.protobuf.Duration",
///       "value": "1.212s"
///     }
///
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "AnyInput")]
pub struct AnyGraphQlInput {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a [google.protobuf.Type][]
    ///   value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///   URL, or have them precompiled into a binary to avoid any
    ///   lookup. Therefore, binary compatibility needs to be preserved
    ///   on changes to types. (Use versioned type names to manage
    ///   breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    ///
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::useless_conversion)]
impl From<Any> for AnyGraphQl {
    fn from(other: Any) -> Self {
        let Any {
            type_url, value, ..
        } = other;
        Self {
            type_url: type_url.into(),
            value: value.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<AnyGraphQl> for Any {
    fn from(other: AnyGraphQl) -> Self {
        let AnyGraphQl { type_url, value } = other;
        Self {
            type_url: type_url.into(),
            value: value.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Any> for AnyGraphQlInput {
    fn from(other: Any) -> Self {
        let Any {
            type_url, value, ..
        } = other;
        Self {
            type_url: type_url.into(),
            value: value.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<AnyGraphQlInput> for Any {
    fn from(other: AnyGraphQlInput) -> Self {
        let AnyGraphQlInput { type_url, value } = other;
        Self {
            type_url: type_url.into(),
            value: value.into_iter().map(Into::into).collect(),
        }
    }
}
/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "SourceContext")]
pub struct SourceContextGraphQl {
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    pub file_name: ::prost::alloc::string::String,
}
/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "SourceContextInput")]
pub struct SourceContextGraphQlInput {
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    pub file_name: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<SourceContext> for SourceContextGraphQl {
    fn from(other: SourceContext) -> Self {
        let SourceContext { file_name, .. } = other;
        Self {
            file_name: file_name.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<SourceContextGraphQl> for SourceContext {
    fn from(other: SourceContextGraphQl) -> Self {
        let SourceContextGraphQl { file_name } = other;
        Self {
            file_name: file_name.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<SourceContext> for SourceContextGraphQlInput {
    fn from(other: SourceContext) -> Self {
        let SourceContext { file_name, .. } = other;
        Self {
            file_name: file_name.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<SourceContextGraphQlInput> for SourceContext {
    fn from(other: SourceContextGraphQlInput) -> Self {
        let SourceContextGraphQlInput { file_name } = other;
        Self {
            file_name: file_name.into(),
        }
    }
}
/// A protocol buffer message type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Type")]
pub struct TypeGraphQl {
    /// The fully qualified message name.
    pub name: ::prost::alloc::string::String,
    /// The list of fields.
    pub fields: ::prost::alloc::vec::Vec<FieldGraphQl>,
    /// The list of types appearing in `oneof` definitions in this type.
    pub oneofs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQl>,
    /// The source context.
    pub source_context: ::core::option::Option<SourceContextGraphQl>,
    /// The source syntax.
    pub syntax: SyntaxGraphQl,
}
/// A protocol buffer message type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "TypeInput")]
pub struct TypeGraphQlInput {
    /// The fully qualified message name.
    pub name: ::prost::alloc::string::String,
    /// The list of fields.
    pub fields: ::prost::alloc::vec::Vec<FieldGraphQlInput>,
    /// The list of types appearing in `oneof` definitions in this type.
    pub oneofs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQlInput>,
    /// The source context.
    pub source_context: ::core::option::Option<SourceContextGraphQlInput>,
    /// The source syntax.
    pub syntax: SyntaxGraphQlInput,
}
#[allow(clippy::useless_conversion)]
impl From<Type> for TypeGraphQl {
    fn from(other: Type) -> Self {
        let syntax = other.syntax();
        let Type {
            name,
            fields,
            oneofs,
            options,
            source_context,
            ..
        } = other;
        Self {
            name: name.into(),
            fields: fields.into_iter().map(Into::into).collect(),
            oneofs: oneofs.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<TypeGraphQl> for Type {
    fn from(other: TypeGraphQl) -> Self {
        let TypeGraphQl {
            name,
            fields,
            oneofs,
            options,
            source_context,
            syntax,
        } = other;
        Self {
            name: name.into(),
            fields: fields.into_iter().map(Into::into).collect(),
            oneofs: oneofs.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax as _,
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Type> for TypeGraphQlInput {
    fn from(other: Type) -> Self {
        let syntax = other.syntax();
        let Type {
            name,
            fields,
            oneofs,
            options,
            source_context,
            ..
        } = other;
        Self {
            name: name.into(),
            fields: fields.into_iter().map(Into::into).collect(),
            oneofs: oneofs.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<TypeGraphQlInput> for Type {
    fn from(other: TypeGraphQlInput) -> Self {
        let TypeGraphQlInput {
            name,
            fields,
            oneofs,
            options,
            source_context,
            syntax,
        } = other;
        Self {
            name: name.into(),
            fields: fields.into_iter().map(Into::into).collect(),
            oneofs: oneofs.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax as _,
        }
    }
}
/// A single field of a message type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Field")]
pub struct FieldGraphQl {
    /// The field type.
    pub kind: field::KindGraphQl,
    /// The field cardinality.
    pub cardinality: field::CardinalityGraphQl,
    /// The field number.
    pub number: i32,
    /// The field name.
    pub name: ::prost::alloc::string::String,
    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    pub type_url: ::prost::alloc::string::String,
    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    pub oneof_index: i32,
    /// Whether to use alternative packed wire representation.
    pub packed: bool,
    /// The protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQl>,
    /// The field JSON name.
    pub json_name: ::prost::alloc::string::String,
    /// The string value of the default value of this field. Proto2 syntax only.
    pub default_value: ::prost::alloc::string::String,
}
/// A single field of a message type.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FieldInput")]
pub struct FieldGraphQlInput {
    /// The field type.
    pub kind: field::KindGraphQlInput,
    /// The field cardinality.
    pub cardinality: field::CardinalityGraphQlInput,
    /// The field number.
    pub number: i32,
    /// The field name.
    pub name: ::prost::alloc::string::String,
    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    pub type_url: ::prost::alloc::string::String,
    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    pub oneof_index: i32,
    /// Whether to use alternative packed wire representation.
    pub packed: bool,
    /// The protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQlInput>,
    /// The field JSON name.
    pub json_name: ::prost::alloc::string::String,
    /// The string value of the default value of this field. Proto2 syntax only.
    pub default_value: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<Field> for FieldGraphQl {
    fn from(other: Field) -> Self {
        let kind = other.kind();
        let cardinality = other.cardinality();
        let Field {
            number,
            name,
            type_url,
            oneof_index,
            packed,
            options,
            json_name,
            default_value,
            ..
        } = other;
        Self {
            kind: kind.into(),
            cardinality: cardinality.into(),
            number: number.into(),
            name: name.into(),
            type_url: type_url.into(),
            oneof_index: oneof_index.into(),
            packed: packed.into(),
            options: options.into_iter().map(Into::into).collect(),
            json_name: json_name.into(),
            default_value: default_value.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldGraphQl> for Field {
    fn from(other: FieldGraphQl) -> Self {
        let FieldGraphQl {
            kind,
            cardinality,
            number,
            name,
            type_url,
            oneof_index,
            packed,
            options,
            json_name,
            default_value,
        } = other;
        Self {
            kind: kind as _,
            cardinality: cardinality as _,
            number: number.into(),
            name: name.into(),
            type_url: type_url.into(),
            oneof_index: oneof_index.into(),
            packed: packed.into(),
            options: options.into_iter().map(Into::into).collect(),
            json_name: json_name.into(),
            default_value: default_value.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Field> for FieldGraphQlInput {
    fn from(other: Field) -> Self {
        let kind = other.kind();
        let cardinality = other.cardinality();
        let Field {
            number,
            name,
            type_url,
            oneof_index,
            packed,
            options,
            json_name,
            default_value,
            ..
        } = other;
        Self {
            kind: kind.into(),
            cardinality: cardinality.into(),
            number: number.into(),
            name: name.into(),
            type_url: type_url.into(),
            oneof_index: oneof_index.into(),
            packed: packed.into(),
            options: options.into_iter().map(Into::into).collect(),
            json_name: json_name.into(),
            default_value: default_value.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldGraphQlInput> for Field {
    fn from(other: FieldGraphQlInput) -> Self {
        let FieldGraphQlInput {
            kind,
            cardinality,
            number,
            name,
            type_url,
            oneof_index,
            packed,
            options,
            json_name,
            default_value,
        } = other;
        Self {
            kind: kind as _,
            cardinality: cardinality as _,
            number: number.into(),
            name: name.into(),
            type_url: type_url.into(),
            oneof_index: oneof_index.into(),
            packed: packed.into(),
            options: options.into_iter().map(Into::into).collect(),
            json_name: json_name.into(),
            default_value: default_value.into(),
        }
    }
}
/// Enum type definition.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Enum")]
pub struct EnumGraphQl {
    /// Enum type name.
    pub name: ::prost::alloc::string::String,
    /// Enum value definitions.
    pub enumvalue: ::prost::alloc::vec::Vec<EnumValueGraphQl>,
    /// Protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQl>,
    /// The source context.
    pub source_context: ::core::option::Option<SourceContextGraphQl>,
    /// The source syntax.
    pub syntax: SyntaxGraphQl,
}
/// Enum type definition.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumInput")]
pub struct EnumGraphQlInput {
    /// Enum type name.
    pub name: ::prost::alloc::string::String,
    /// Enum value definitions.
    pub enumvalue: ::prost::alloc::vec::Vec<EnumValueGraphQlInput>,
    /// Protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQlInput>,
    /// The source context.
    pub source_context: ::core::option::Option<SourceContextGraphQlInput>,
    /// The source syntax.
    pub syntax: SyntaxGraphQlInput,
}
#[allow(clippy::useless_conversion)]
impl From<Enum> for EnumGraphQl {
    fn from(other: Enum) -> Self {
        let syntax = other.syntax();
        let Enum {
            name,
            enumvalue,
            options,
            source_context,
            ..
        } = other;
        Self {
            name: name.into(),
            enumvalue: enumvalue.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumGraphQl> for Enum {
    fn from(other: EnumGraphQl) -> Self {
        let EnumGraphQl {
            name,
            enumvalue,
            options,
            source_context,
            syntax,
        } = other;
        Self {
            name: name.into(),
            enumvalue: enumvalue.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax as _,
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Enum> for EnumGraphQlInput {
    fn from(other: Enum) -> Self {
        let syntax = other.syntax();
        let Enum {
            name,
            enumvalue,
            options,
            source_context,
            ..
        } = other;
        Self {
            name: name.into(),
            enumvalue: enumvalue.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumGraphQlInput> for Enum {
    fn from(other: EnumGraphQlInput) -> Self {
        let EnumGraphQlInput {
            name,
            enumvalue,
            options,
            source_context,
            syntax,
        } = other;
        Self {
            name: name.into(),
            enumvalue: enumvalue.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            source_context: source_context.map(Into::into),
            syntax: syntax as _,
        }
    }
}
/// Enum value definition.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumValue")]
pub struct EnumValueGraphQl {
    /// Enum value name.
    pub name: ::prost::alloc::string::String,
    /// Enum value number.
    pub number: i32,
    /// Protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQl>,
}
/// Enum value definition.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "EnumValueInput")]
pub struct EnumValueGraphQlInput {
    /// Enum value name.
    pub name: ::prost::alloc::string::String,
    /// Enum value number.
    pub number: i32,
    /// Protocol buffer options.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<EnumValue> for EnumValueGraphQl {
    fn from(other: EnumValue) -> Self {
        let EnumValue {
            name,
            number,
            options,
            ..
        } = other;
        Self {
            name: name.into(),
            number: number.into(),
            options: options.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueGraphQl> for EnumValue {
    fn from(other: EnumValueGraphQl) -> Self {
        let EnumValueGraphQl {
            name,
            number,
            options,
        } = other;
        Self {
            name: name.into(),
            number: number.into(),
            options: options.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValue> for EnumValueGraphQlInput {
    fn from(other: EnumValue) -> Self {
        let EnumValue {
            name,
            number,
            options,
            ..
        } = other;
        Self {
            name: name.into(),
            number: number.into(),
            options: options.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<EnumValueGraphQlInput> for EnumValue {
    fn from(other: EnumValueGraphQlInput) -> Self {
        let EnumValueGraphQlInput {
            name,
            number,
            options,
        } = other;
        Self {
            name: name.into(),
            number: number.into(),
            options: options.into_iter().map(Into::into).collect(),
        }
    }
}
/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Option")]
pub struct OptionGraphQl {
    /// The option's name. For protobuf built-in options (options defined in
    /// descriptor.proto), this is the short name. For example, `"map_entry"`.
    /// For custom options, it should be the fully-qualified name. For example,
    /// `"google.api.http"`.
    pub name: ::prost::alloc::string::String,
    /// The option's value packed in an Any message. If the value is a primitive,
    /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
    /// should be used. If the value is an enum, it should be stored as an int32
    /// value using the google.protobuf.Int32Value type.
    pub value: ::core::option::Option<AnyGraphQl>,
}
/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "OptionInput")]
pub struct OptionGraphQlInput {
    /// The option's name. For protobuf built-in options (options defined in
    /// descriptor.proto), this is the short name. For example, `"map_entry"`.
    /// For custom options, it should be the fully-qualified name. For example,
    /// `"google.api.http"`.
    pub name: ::prost::alloc::string::String,
    /// The option's value packed in an Any message. If the value is a primitive,
    /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
    /// should be used. If the value is an enum, it should be stored as an int32
    /// value using the google.protobuf.Int32Value type.
    pub value: ::core::option::Option<AnyGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<Option> for OptionGraphQl {
    fn from(other: Option) -> Self {
        let Option { name, value, .. } = other;
        Self {
            name: name.into(),
            value: value.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OptionGraphQl> for Option {
    fn from(other: OptionGraphQl) -> Self {
        let OptionGraphQl { name, value } = other;
        Self {
            name: name.into(),
            value: value.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Option> for OptionGraphQlInput {
    fn from(other: Option) -> Self {
        let Option { name, value, .. } = other;
        Self {
            name: name.into(),
            value: value.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<OptionGraphQlInput> for Option {
    fn from(other: OptionGraphQlInput) -> Self {
        let OptionGraphQlInput { name, value } = other;
        Self {
            name: name.into(),
            value: value.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Syntax> for SyntaxGraphQl {
    fn from(other: Syntax) -> Self {
        match other {
            Syntax::Proto2 => Self::Proto2,
            Syntax::Proto3 => Self::Proto3,
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<SyntaxGraphQl> for Syntax {
    fn from(other: SyntaxGraphQl) -> Self {
        match other {
            SyntaxGraphQl::Proto2 => Self::Proto2,
            SyntaxGraphQl::Proto3 => Self::Proto3,
        }
    }
}
/// The syntax in which a protocol buffer element is defined.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
#[derive(
    :: async_graphql :: Enum,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Syntax")]
pub enum SyntaxGraphQl {
    /// Syntax `proto2`.
    Proto2 = 0,
    /// Syntax `proto3`.
    Proto3 = 1,
}
pub use self::SyntaxGraphQl as SyntaxGraphQlInput;
/// Api is a light-weight descriptor for an API Interface.
///
/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See https://cloud.google.com/apis/design/glossary for
/// detailed terminology.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Api")]
pub struct ApiGraphQl {
    /// The fully qualified name of this interface, including package name
    /// followed by the interface's simple name.
    pub name: ::prost::alloc::string::String,
    /// The methods of this interface, in unspecified order.
    pub methods: ::prost::alloc::vec::Vec<MethodGraphQl>,
    /// Any metadata attached to the interface.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQl>,
    /// A version string for this interface. If specified, must have the form
    /// `major-version.minor-version`, as in `1.10`. If the minor version is
    /// omitted, it defaults to zero. If the entire version field is empty, the
    /// major version is derived from the package name, as outlined below. If the
    /// field is not empty, the version in the package name will be verified to be
    /// consistent with what is provided here.
    ///
    /// The versioning schema uses [semantic
    /// versioning](http://semver.org) where the major version number
    /// indicates a breaking change and the minor version an additive,
    /// non-breaking change. Both version numbers are signals to users
    /// what to expect from different versions, and should be carefully
    /// chosen based on the product plan.
    ///
    /// The major version is also reflected in the package name of the
    /// interface, which must end in `v<major-version>`, as in
    /// `google.feature.v1`. For major versions 0 and 1, the suffix can
    /// be omitted. Zero major versions must only be used for
    /// experimental, non-GA interfaces.
    ///
    ///
    pub version: ::prost::alloc::string::String,
    /// Source context for the protocol buffer service represented by this
    /// message.
    pub source_context: ::core::option::Option<SourceContextGraphQl>,
    /// Included interfaces. See [Mixin][].
    pub mixins: ::prost::alloc::vec::Vec<MixinGraphQl>,
    /// The source syntax of the service.
    pub syntax: SyntaxGraphQl,
}
/// Api is a light-weight descriptor for an API Interface.
///
/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See https://cloud.google.com/apis/design/glossary for
/// detailed terminology.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ApiInput")]
pub struct ApiGraphQlInput {
    /// The fully qualified name of this interface, including package name
    /// followed by the interface's simple name.
    pub name: ::prost::alloc::string::String,
    /// The methods of this interface, in unspecified order.
    pub methods: ::prost::alloc::vec::Vec<MethodGraphQlInput>,
    /// Any metadata attached to the interface.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQlInput>,
    /// A version string for this interface. If specified, must have the form
    /// `major-version.minor-version`, as in `1.10`. If the minor version is
    /// omitted, it defaults to zero. If the entire version field is empty, the
    /// major version is derived from the package name, as outlined below. If the
    /// field is not empty, the version in the package name will be verified to be
    /// consistent with what is provided here.
    ///
    /// The versioning schema uses [semantic
    /// versioning](http://semver.org) where the major version number
    /// indicates a breaking change and the minor version an additive,
    /// non-breaking change. Both version numbers are signals to users
    /// what to expect from different versions, and should be carefully
    /// chosen based on the product plan.
    ///
    /// The major version is also reflected in the package name of the
    /// interface, which must end in `v<major-version>`, as in
    /// `google.feature.v1`. For major versions 0 and 1, the suffix can
    /// be omitted. Zero major versions must only be used for
    /// experimental, non-GA interfaces.
    ///
    ///
    pub version: ::prost::alloc::string::String,
    /// Source context for the protocol buffer service represented by this
    /// message.
    pub source_context: ::core::option::Option<SourceContextGraphQlInput>,
    /// Included interfaces. See [Mixin][].
    pub mixins: ::prost::alloc::vec::Vec<MixinGraphQlInput>,
    /// The source syntax of the service.
    pub syntax: SyntaxGraphQlInput,
}
#[allow(clippy::useless_conversion)]
impl From<Api> for ApiGraphQl {
    fn from(other: Api) -> Self {
        let syntax = other.syntax();
        let Api {
            name,
            methods,
            options,
            version,
            source_context,
            mixins,
            ..
        } = other;
        Self {
            name: name.into(),
            methods: methods.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            version: version.into(),
            source_context: source_context.map(Into::into),
            mixins: mixins.into_iter().map(Into::into).collect(),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ApiGraphQl> for Api {
    fn from(other: ApiGraphQl) -> Self {
        let ApiGraphQl {
            name,
            methods,
            options,
            version,
            source_context,
            mixins,
            syntax,
        } = other;
        Self {
            name: name.into(),
            methods: methods.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            version: version.into(),
            source_context: source_context.map(Into::into),
            mixins: mixins.into_iter().map(Into::into).collect(),
            syntax: syntax as _,
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Api> for ApiGraphQlInput {
    fn from(other: Api) -> Self {
        let syntax = other.syntax();
        let Api {
            name,
            methods,
            options,
            version,
            source_context,
            mixins,
            ..
        } = other;
        Self {
            name: name.into(),
            methods: methods.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            version: version.into(),
            source_context: source_context.map(Into::into),
            mixins: mixins.into_iter().map(Into::into).collect(),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ApiGraphQlInput> for Api {
    fn from(other: ApiGraphQlInput) -> Self {
        let ApiGraphQlInput {
            name,
            methods,
            options,
            version,
            source_context,
            mixins,
            syntax,
        } = other;
        Self {
            name: name.into(),
            methods: methods.into_iter().map(Into::into).collect(),
            options: options.into_iter().map(Into::into).collect(),
            version: version.into(),
            source_context: source_context.map(Into::into),
            mixins: mixins.into_iter().map(Into::into).collect(),
            syntax: syntax as _,
        }
    }
}
/// Method represents a method of an API interface.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Method")]
pub struct MethodGraphQl {
    /// The simple name of this method.
    pub name: ::prost::alloc::string::String,
    /// A URL of the input message type.
    pub request_type_url: ::prost::alloc::string::String,
    /// If true, the request is streamed.
    pub request_streaming: bool,
    /// The URL of the output message type.
    pub response_type_url: ::prost::alloc::string::String,
    /// If true, the response is streamed.
    pub response_streaming: bool,
    /// Any metadata attached to the method.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQl>,
    /// The source syntax of this method.
    pub syntax: SyntaxGraphQl,
}
/// Method represents a method of an API interface.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "MethodInput")]
pub struct MethodGraphQlInput {
    /// The simple name of this method.
    pub name: ::prost::alloc::string::String,
    /// A URL of the input message type.
    pub request_type_url: ::prost::alloc::string::String,
    /// If true, the request is streamed.
    pub request_streaming: bool,
    /// The URL of the output message type.
    pub response_type_url: ::prost::alloc::string::String,
    /// If true, the response is streamed.
    pub response_streaming: bool,
    /// Any metadata attached to the method.
    pub options: ::prost::alloc::vec::Vec<OptionGraphQlInput>,
    /// The source syntax of this method.
    pub syntax: SyntaxGraphQlInput,
}
#[allow(clippy::useless_conversion)]
impl From<Method> for MethodGraphQl {
    fn from(other: Method) -> Self {
        let syntax = other.syntax();
        let Method {
            name,
            request_type_url,
            request_streaming,
            response_type_url,
            response_streaming,
            options,
            ..
        } = other;
        Self {
            name: name.into(),
            request_type_url: request_type_url.into(),
            request_streaming: request_streaming.into(),
            response_type_url: response_type_url.into(),
            response_streaming: response_streaming.into(),
            options: options.into_iter().map(Into::into).collect(),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodGraphQl> for Method {
    fn from(other: MethodGraphQl) -> Self {
        let MethodGraphQl {
            name,
            request_type_url,
            request_streaming,
            response_type_url,
            response_streaming,
            options,
            syntax,
        } = other;
        Self {
            name: name.into(),
            request_type_url: request_type_url.into(),
            request_streaming: request_streaming.into(),
            response_type_url: response_type_url.into(),
            response_streaming: response_streaming.into(),
            options: options.into_iter().map(Into::into).collect(),
            syntax: syntax as _,
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Method> for MethodGraphQlInput {
    fn from(other: Method) -> Self {
        let syntax = other.syntax();
        let Method {
            name,
            request_type_url,
            request_streaming,
            response_type_url,
            response_streaming,
            options,
            ..
        } = other;
        Self {
            name: name.into(),
            request_type_url: request_type_url.into(),
            request_streaming: request_streaming.into(),
            response_type_url: response_type_url.into(),
            response_streaming: response_streaming.into(),
            options: options.into_iter().map(Into::into).collect(),
            syntax: syntax.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MethodGraphQlInput> for Method {
    fn from(other: MethodGraphQlInput) -> Self {
        let MethodGraphQlInput {
            name,
            request_type_url,
            request_streaming,
            response_type_url,
            response_streaming,
            options,
            syntax,
        } = other;
        Self {
            name: name.into(),
            request_type_url: request_type_url.into(),
            request_streaming: request_streaming.into(),
            response_type_url: response_type_url.into(),
            response_streaming: response_streaming.into(),
            options: options.into_iter().map(Into::into).collect(),
            syntax: syntax as _,
        }
    }
}
/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:
///
/// - If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.
///
/// - Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.
///
/// - If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the [root][] path if
///   specified.
///
/// Example of a simple mixin:
///
///     package google.acl.v1;
///     service AccessControl {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v1/{resource=**}:getAcl";
///       }
///     }
///
///     package google.storage.v2;
///     service Storage {
///       rpc GetAcl(GetAclRequest) returns (Acl);
///
///       // Get a data record.
///       rpc GetData(GetDataRequest) returns (Data) {
///         option (google.api.http).get = "/v2/{resource=**}";
///       }
///     }
///
/// Example of a mixin configuration:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///
/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inheriting
/// documentation and annotations as follows:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/{resource=**}:getAcl";
///       }
///       ...
///     }
///
/// Note how the version in the path pattern changed from `v1` to `v2`.
///
/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///         root: acls
///
/// This implies the following inherited HTTP annotation:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///       }
///       ...
///     }
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Mixin")]
pub struct MixinGraphQl {
    /// The fully qualified name of the interface which is included.
    pub name: ::prost::alloc::string::String,
    /// If non-empty specifies a path under which inherited HTTP paths
    /// are rooted.
    pub root: ::prost::alloc::string::String,
}
/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:
///
/// - If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.
///
/// - Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.
///
/// - If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the [root][] path if
///   specified.
///
/// Example of a simple mixin:
///
///     package google.acl.v1;
///     service AccessControl {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v1/{resource=**}:getAcl";
///       }
///     }
///
///     package google.storage.v2;
///     service Storage {
///       rpc GetAcl(GetAclRequest) returns (Acl);
///
///       // Get a data record.
///       rpc GetData(GetDataRequest) returns (Data) {
///         option (google.api.http).get = "/v2/{resource=**}";
///       }
///     }
///
/// Example of a mixin configuration:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///
/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inheriting
/// documentation and annotations as follows:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/{resource=**}:getAcl";
///       }
///       ...
///     }
///
/// Note how the version in the path pattern changed from `v1` to `v2`.
///
/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///         root: acls
///
/// This implies the following inherited HTTP annotation:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///       }
///       ...
///     }
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "MixinInput")]
pub struct MixinGraphQlInput {
    /// The fully qualified name of the interface which is included.
    pub name: ::prost::alloc::string::String,
    /// If non-empty specifies a path under which inherited HTTP paths
    /// are rooted.
    pub root: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<Mixin> for MixinGraphQl {
    fn from(other: Mixin) -> Self {
        let Mixin { name, root, .. } = other;
        Self {
            name: name.into(),
            root: root.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MixinGraphQl> for Mixin {
    fn from(other: MixinGraphQl) -> Self {
        let MixinGraphQl { name, root } = other;
        Self {
            name: name.into(),
            root: root.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Mixin> for MixinGraphQlInput {
    fn from(other: Mixin) -> Self {
        let Mixin { name, root, .. } = other;
        Self {
            name: name.into(),
            root: root.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<MixinGraphQlInput> for Mixin {
    fn from(other: MixinGraphQlInput) -> Self {
        let MixinGraphQlInput { name, root } = other;
        Self {
            name: name.into(),
            root: root.into(),
        }
    }
}
/// A Duration represents a signed, fixed-length span of time represented
/// as a count of seconds and fractions of seconds at nanosecond
/// resolution. It is independent of any calendar and concepts like "day"
/// or "month". It is related to Timestamp in that the difference between
/// two Timestamp values is a Duration and it can be added or subtracted
/// from a Timestamp. Range is approximately +-10,000 years.
///
/// # Examples
///
/// Example 1: Compute Duration from two Timestamps in pseudo code.
///
///     Timestamp start = ...;
///     Timestamp end = ...;
///     Duration duration = ...;
///
///     duration.seconds = end.seconds - start.seconds;
///     duration.nanos = end.nanos - start.nanos;
///
///     if (duration.seconds < 0 && duration.nanos > 0) {
///       duration.seconds += 1;
///       duration.nanos -= 1000000000;
///     } else if (duration.seconds > 0 && duration.nanos < 0) {
///       duration.seconds -= 1;
///       duration.nanos += 1000000000;
///     }
///
/// Example 2: Compute Timestamp from Timestamp + Duration in pseudo code.
///
///     Timestamp start = ...;
///     Duration duration = ...;
///     Timestamp end = ...;
///
///     end.seconds = start.seconds + duration.seconds;
///     end.nanos = start.nanos + duration.nanos;
///
///     if (end.nanos < 0) {
///       end.seconds -= 1;
///       end.nanos += 1000000000;
///     } else if (end.nanos >= 1000000000) {
///       end.seconds += 1;
///       end.nanos -= 1000000000;
///     }
///
/// Example 3: Compute Duration from datetime.timedelta in Python.
///
///     td = datetime.timedelta(days=3, minutes=10)
///     duration = Duration()
///     duration.FromTimedelta(td)
///
/// # JSON Mapping
///
/// In JSON format, the Duration type is encoded as a string rather than an
/// object, where the string ends in the suffix "s" (indicating seconds) and
/// is preceded by the number of seconds, with nanoseconds expressed as
/// fractional seconds. For example, 3 seconds with 0 nanoseconds should be
/// encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should
/// be expressed in JSON format as "3.000000001s", and 3 seconds and 1
/// microsecond should be expressed in JSON format as "3.000001s".
///
///
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Duration")]
pub struct DurationGraphQl {
    /// Signed seconds of the span of time. Must be from -315,576,000,000
    /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
    /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    pub seconds: i64,
    /// Signed fractions of a second at nanosecond resolution of the span
    /// of time. Durations less than one second are represented with a 0
    /// `seconds` field and a positive or negative `nanos` field. For durations
    /// of one second or more, a non-zero value for the `nanos` field must be
    /// of the same sign as the `seconds` field. Must be from -999,999,999
    /// to +999,999,999 inclusive.
    pub nanos: i32,
}
/// A Duration represents a signed, fixed-length span of time represented
/// as a count of seconds and fractions of seconds at nanosecond
/// resolution. It is independent of any calendar and concepts like "day"
/// or "month". It is related to Timestamp in that the difference between
/// two Timestamp values is a Duration and it can be added or subtracted
/// from a Timestamp. Range is approximately +-10,000 years.
///
/// # Examples
///
/// Example 1: Compute Duration from two Timestamps in pseudo code.
///
///     Timestamp start = ...;
///     Timestamp end = ...;
///     Duration duration = ...;
///
///     duration.seconds = end.seconds - start.seconds;
///     duration.nanos = end.nanos - start.nanos;
///
///     if (duration.seconds < 0 && duration.nanos > 0) {
///       duration.seconds += 1;
///       duration.nanos -= 1000000000;
///     } else if (duration.seconds > 0 && duration.nanos < 0) {
///       duration.seconds -= 1;
///       duration.nanos += 1000000000;
///     }
///
/// Example 2: Compute Timestamp from Timestamp + Duration in pseudo code.
///
///     Timestamp start = ...;
///     Duration duration = ...;
///     Timestamp end = ...;
///
///     end.seconds = start.seconds + duration.seconds;
///     end.nanos = start.nanos + duration.nanos;
///
///     if (end.nanos < 0) {
///       end.seconds -= 1;
///       end.nanos += 1000000000;
///     } else if (end.nanos >= 1000000000) {
///       end.seconds += 1;
///       end.nanos -= 1000000000;
///     }
///
/// Example 3: Compute Duration from datetime.timedelta in Python.
///
///     td = datetime.timedelta(days=3, minutes=10)
///     duration = Duration()
///     duration.FromTimedelta(td)
///
/// # JSON Mapping
///
/// In JSON format, the Duration type is encoded as a string rather than an
/// object, where the string ends in the suffix "s" (indicating seconds) and
/// is preceded by the number of seconds, with nanoseconds expressed as
/// fractional seconds. For example, 3 seconds with 0 nanoseconds should be
/// encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should
/// be expressed in JSON format as "3.000000001s", and 3 seconds and 1
/// microsecond should be expressed in JSON format as "3.000001s".
///
///
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "DurationInput")]
pub struct DurationGraphQlInput {
    /// Signed seconds of the span of time. Must be from -315,576,000,000
    /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
    /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    pub seconds: i64,
    /// Signed fractions of a second at nanosecond resolution of the span
    /// of time. Durations less than one second are represented with a 0
    /// `seconds` field and a positive or negative `nanos` field. For durations
    /// of one second or more, a non-zero value for the `nanos` field must be
    /// of the same sign as the `seconds` field. Must be from -999,999,999
    /// to +999,999,999 inclusive.
    pub nanos: i32,
}
#[allow(clippy::useless_conversion)]
impl From<Duration> for DurationGraphQl {
    fn from(other: Duration) -> Self {
        let Duration { seconds, nanos, .. } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<DurationGraphQl> for Duration {
    fn from(other: DurationGraphQl) -> Self {
        let DurationGraphQl { seconds, nanos } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Duration> for DurationGraphQlInput {
    fn from(other: Duration) -> Self {
        let Duration { seconds, nanos, .. } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<DurationGraphQlInput> for Duration {
    fn from(other: DurationGraphQlInput) -> Self {
        let DurationGraphQlInput { seconds, nanos } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
/// `FieldMask` represents a set of symbolic field paths, for example:
///
///     paths: "f.a"
///     paths: "f.b.d"
///
/// Here `f` represents a field in some root message, `a` and `b`
/// fields in the message found in `f`, and `d` a field found in the
/// message in `f.b`.
///
/// Field masks are used to specify a subset of fields that should be
/// returned by a get operation or modified by an update operation.
/// Field masks also have a custom JSON encoding (see below).
///
/// # Field Masks in Projections
///
/// When used in the context of a projection, a response message or
/// sub-message is filtered by the API to only contain those fields as
/// specified in the mask. For example, if the mask in the previous
/// example is applied to a response message as follows:
///
///     f {
///       a : 22
///       b {
///         d : 1
///         x : 2
///       }
///       y : 13
///     }
///     z: 8
///
/// The result will not contain specific values for fields x,y and z
/// (their value will be set to the default, and omitted in proto text
/// output):
///
///
///     f {
///       a : 22
///       b {
///         d : 1
///       }
///     }
///
/// A repeated field is not allowed except at the last position of a
/// paths string.
///
/// If a FieldMask object is not present in a get operation, the
/// operation applies to all fields (as if a FieldMask of all fields
/// had been specified).
///
/// Note that a field mask does not necessarily apply to the
/// top-level response message. In case of a REST get operation, the
/// field mask applies directly to the response, but in case of a REST
/// list operation, the mask instead applies to each individual message
/// in the returned resource list. In case of a REST custom method,
/// other definitions may be used. Where the mask applies will be
/// clearly documented together with its declaration in the API.  In
/// any case, the effect on the returned resource/resources is required
/// behavior for APIs.
///
/// # Field Masks in Update Operations
///
/// A field mask in update operations specifies which fields of the
/// targeted resource are going to be updated. The API is required
/// to only change the values of the fields as specified in the mask
/// and leave the others untouched. If a resource is passed in to
/// describe the updated values, the API ignores the values of all
/// fields not covered by the mask.
///
/// If a repeated field is specified for an update operation, new values will
/// be appended to the existing repeated field in the target resource. Note that
/// a repeated field is only allowed in the last position of a `paths` string.
///
/// If a sub-message is specified in the last position of the field mask for an
/// update operation, then new value will be merged into the existing sub-message
/// in the target resource.
///
/// For example, given the target message:
///
///     f {
///       b {
///         d: 1
///         x: 2
///       }
///       c: [1]
///     }
///
/// And an update message:
///
///     f {
///       b {
///         d: 10
///       }
///       c: [2]
///     }
///
/// then if the field mask is:
///
///  paths: ["f.b", "f.c"]
///
/// then the result will be:
///
///     f {
///       b {
///         d: 10
///         x: 2
///       }
///       c: [1, 2]
///     }
///
/// An implementation may provide options to override this default behavior for
/// repeated and message fields.
///
/// In order to reset a field's value to the default, the field must
/// be in the mask and set to the default value in the provided resource.
/// Hence, in order to reset all fields of a resource, provide a default
/// instance of the resource and set all fields in the mask, or do
/// not provide a mask as described below.
///
/// If a field mask is not present on update, the operation applies to
/// all fields (as if a field mask of all fields has been specified).
/// Note that in the presence of schema evolution, this may mean that
/// fields the client does not know and has therefore not filled into
/// the request will be reset to their default. If this is unwanted
/// behavior, a specific service may require a client to always specify
/// a field mask, producing an error if not.
///
/// As with get operations, the location of the resource which
/// describes the updated values in the request message depends on the
/// operation kind. In any case, the effect of the field mask is
/// required to be honored by the API.
///
/// ## Considerations for HTTP REST
///
/// The HTTP kind of an update operation which uses a field mask must
/// be set to PATCH instead of PUT in order to satisfy HTTP semantics
/// (PUT must only be used for full updates).
///
/// # JSON Encoding of Field Masks
///
/// In JSON, a field mask is encoded as a single string where paths are
/// separated by a comma. Fields name in each path are converted
/// to/from lower-camel naming conventions.
///
/// As an example, consider the following message declarations:
///
///     message Profile {
///       User user = 1;
///       Photo photo = 2;
///     }
///     message User {
///       string display_name = 1;
///       string address = 2;
///     }
///
/// In proto a field mask for `Profile` may look as such:
///
///     mask {
///       paths: "user.display_name"
///       paths: "photo"
///     }
///
/// In JSON, the same mask is represented as below:
///
///     {
///       mask: "user.displayName,photo"
///     }
///
/// # Field Masks and Oneof Fields
///
/// Field masks treat fields in oneofs just as regular fields. Consider the
/// following message:
///
///     message SampleMessage {
///       oneof test_oneof {
///         string name = 4;
///         SubMessage sub_message = 9;
///       }
///     }
///
/// The field mask can be:
///
///     mask {
///       paths: "name"
///     }
///
/// Or:
///
///     mask {
///       paths: "sub_message"
///     }
///
/// Note that oneof type names ("test_oneof" in this case) cannot be used in
/// paths.
///
/// ## Field Mask Verification
///
/// The implementation of any API method which has a FieldMask type field in the
/// request should verify the included field paths, and return an
/// `INVALID_ARGUMENT` error if any path is unmappable.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FieldMask")]
pub struct FieldMaskGraphQl {
    /// The set of field mask paths.
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `FieldMask` represents a set of symbolic field paths, for example:
///
///     paths: "f.a"
///     paths: "f.b.d"
///
/// Here `f` represents a field in some root message, `a` and `b`
/// fields in the message found in `f`, and `d` a field found in the
/// message in `f.b`.
///
/// Field masks are used to specify a subset of fields that should be
/// returned by a get operation or modified by an update operation.
/// Field masks also have a custom JSON encoding (see below).
///
/// # Field Masks in Projections
///
/// When used in the context of a projection, a response message or
/// sub-message is filtered by the API to only contain those fields as
/// specified in the mask. For example, if the mask in the previous
/// example is applied to a response message as follows:
///
///     f {
///       a : 22
///       b {
///         d : 1
///         x : 2
///       }
///       y : 13
///     }
///     z: 8
///
/// The result will not contain specific values for fields x,y and z
/// (their value will be set to the default, and omitted in proto text
/// output):
///
///
///     f {
///       a : 22
///       b {
///         d : 1
///       }
///     }
///
/// A repeated field is not allowed except at the last position of a
/// paths string.
///
/// If a FieldMask object is not present in a get operation, the
/// operation applies to all fields (as if a FieldMask of all fields
/// had been specified).
///
/// Note that a field mask does not necessarily apply to the
/// top-level response message. In case of a REST get operation, the
/// field mask applies directly to the response, but in case of a REST
/// list operation, the mask instead applies to each individual message
/// in the returned resource list. In case of a REST custom method,
/// other definitions may be used. Where the mask applies will be
/// clearly documented together with its declaration in the API.  In
/// any case, the effect on the returned resource/resources is required
/// behavior for APIs.
///
/// # Field Masks in Update Operations
///
/// A field mask in update operations specifies which fields of the
/// targeted resource are going to be updated. The API is required
/// to only change the values of the fields as specified in the mask
/// and leave the others untouched. If a resource is passed in to
/// describe the updated values, the API ignores the values of all
/// fields not covered by the mask.
///
/// If a repeated field is specified for an update operation, new values will
/// be appended to the existing repeated field in the target resource. Note that
/// a repeated field is only allowed in the last position of a `paths` string.
///
/// If a sub-message is specified in the last position of the field mask for an
/// update operation, then new value will be merged into the existing sub-message
/// in the target resource.
///
/// For example, given the target message:
///
///     f {
///       b {
///         d: 1
///         x: 2
///       }
///       c: [1]
///     }
///
/// And an update message:
///
///     f {
///       b {
///         d: 10
///       }
///       c: [2]
///     }
///
/// then if the field mask is:
///
///  paths: ["f.b", "f.c"]
///
/// then the result will be:
///
///     f {
///       b {
///         d: 10
///         x: 2
///       }
///       c: [1, 2]
///     }
///
/// An implementation may provide options to override this default behavior for
/// repeated and message fields.
///
/// In order to reset a field's value to the default, the field must
/// be in the mask and set to the default value in the provided resource.
/// Hence, in order to reset all fields of a resource, provide a default
/// instance of the resource and set all fields in the mask, or do
/// not provide a mask as described below.
///
/// If a field mask is not present on update, the operation applies to
/// all fields (as if a field mask of all fields has been specified).
/// Note that in the presence of schema evolution, this may mean that
/// fields the client does not know and has therefore not filled into
/// the request will be reset to their default. If this is unwanted
/// behavior, a specific service may require a client to always specify
/// a field mask, producing an error if not.
///
/// As with get operations, the location of the resource which
/// describes the updated values in the request message depends on the
/// operation kind. In any case, the effect of the field mask is
/// required to be honored by the API.
///
/// ## Considerations for HTTP REST
///
/// The HTTP kind of an update operation which uses a field mask must
/// be set to PATCH instead of PUT in order to satisfy HTTP semantics
/// (PUT must only be used for full updates).
///
/// # JSON Encoding of Field Masks
///
/// In JSON, a field mask is encoded as a single string where paths are
/// separated by a comma. Fields name in each path are converted
/// to/from lower-camel naming conventions.
///
/// As an example, consider the following message declarations:
///
///     message Profile {
///       User user = 1;
///       Photo photo = 2;
///     }
///     message User {
///       string display_name = 1;
///       string address = 2;
///     }
///
/// In proto a field mask for `Profile` may look as such:
///
///     mask {
///       paths: "user.display_name"
///       paths: "photo"
///     }
///
/// In JSON, the same mask is represented as below:
///
///     {
///       mask: "user.displayName,photo"
///     }
///
/// # Field Masks and Oneof Fields
///
/// Field masks treat fields in oneofs just as regular fields. Consider the
/// following message:
///
///     message SampleMessage {
///       oneof test_oneof {
///         string name = 4;
///         SubMessage sub_message = 9;
///       }
///     }
///
/// The field mask can be:
///
///     mask {
///       paths: "name"
///     }
///
/// Or:
///
///     mask {
///       paths: "sub_message"
///     }
///
/// Note that oneof type names ("test_oneof" in this case) cannot be used in
/// paths.
///
/// ## Field Mask Verification
///
/// The implementation of any API method which has a FieldMask type field in the
/// request should verify the included field paths, and return an
/// `INVALID_ARGUMENT` error if any path is unmappable.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "FieldMaskInput")]
pub struct FieldMaskGraphQlInput {
    /// The set of field mask paths.
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::useless_conversion)]
impl From<FieldMask> for FieldMaskGraphQl {
    fn from(other: FieldMask) -> Self {
        let FieldMask { paths, .. } = other;
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldMaskGraphQl> for FieldMask {
    fn from(other: FieldMaskGraphQl) -> Self {
        let FieldMaskGraphQl { paths } = other;
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldMask> for FieldMaskGraphQlInput {
    fn from(other: FieldMask) -> Self {
        let FieldMask { paths, .. } = other;
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<FieldMaskGraphQlInput> for FieldMask {
    fn from(other: FieldMaskGraphQlInput) -> Self {
        let FieldMaskGraphQlInput { paths } = other;
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
        }
    }
}
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is JSON object.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Struct")]
pub struct StructGraphQl {
    /// Unordered map of dynamically typed values.
    pub fields: ::async_graphql::Json<
        ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ValueGraphQl>,
    >,
}
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is JSON object.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "StructInput")]
pub struct StructGraphQlInput {
    /// Unordered map of dynamically typed values.
    pub fields: ::async_graphql::Json<
        ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ValueGraphQlInput>,
    >,
}
#[allow(clippy::useless_conversion)]
impl From<Struct> for StructGraphQl {
    fn from(other: Struct) -> Self {
        let Struct { fields, .. } = other;
        Self {
            fields: ::async_graphql::Json(fields.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<StructGraphQl> for Struct {
    fn from(other: StructGraphQl) -> Self {
        let StructGraphQl { fields } = other;
        Self {
            fields: fields.0.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Struct> for StructGraphQlInput {
    fn from(other: Struct) -> Self {
        let Struct { fields, .. } = other;
        Self {
            fields: ::async_graphql::Json(fields.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<StructGraphQlInput> for Struct {
    fn from(other: StructGraphQlInput) -> Self {
        let StructGraphQlInput { fields } = other;
        Self {
            fields: fields.0.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of that
/// variants, absence of any variant indicates an error.
///
/// The JSON representation for `Value` is JSON value.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Value")]
pub struct ValueGraphQl {
    /// The kind of value.
    pub kind: ::core::option::Option<value::KindGraphQl>,
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of that
/// variants, absence of any variant indicates an error.
///
/// The JSON representation for `Value` is JSON value.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ValueInput")]
pub struct ValueGraphQlInput {
    /// The kind of value.
    pub kind: ::core::option::Option<value::KindGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<Value> for ValueGraphQl {
    fn from(other: Value) -> Self {
        let Value { kind, .. } = other;
        Self {
            kind: kind.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ValueGraphQl> for Value {
    fn from(other: ValueGraphQl) -> Self {
        let ValueGraphQl { kind } = other;
        Self {
            kind: kind.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Value> for ValueGraphQlInput {
    fn from(other: Value) -> Self {
        let Value { kind, .. } = other;
        Self {
            kind: kind.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ValueGraphQlInput> for Value {
    fn from(other: ValueGraphQlInput) -> Self {
        let ValueGraphQlInput { kind } = other;
        Self {
            kind: kind.map(Into::into),
        }
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ListValue")]
pub struct ListValueGraphQl {
    /// Repeated field of dynamically typed values.
    pub values: ::prost::alloc::vec::Vec<ValueGraphQl>,
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "ListValueInput")]
pub struct ListValueGraphQlInput {
    /// Repeated field of dynamically typed values.
    pub values: ::prost::alloc::vec::Vec<ValueGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ListValue> for ListValueGraphQl {
    fn from(other: ListValue) -> Self {
        let ListValue { values, .. } = other;
        Self {
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ListValueGraphQl> for ListValue {
    fn from(other: ListValueGraphQl) -> Self {
        let ListValueGraphQl { values } = other;
        Self {
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ListValue> for ListValueGraphQlInput {
    fn from(other: ListValue) -> Self {
        let ListValue { values, .. } = other;
        Self {
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ListValueGraphQlInput> for ListValue {
    fn from(other: ListValueGraphQlInput) -> Self {
        let ListValueGraphQlInput { values } = other;
        Self {
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<NullValue> for NullValueGraphQl {
    fn from(other: NullValue) -> Self {
        match other {
            NullValue::NullValue => Self::NullValue,
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<NullValueGraphQl> for NullValue {
    fn from(other: NullValueGraphQl) -> Self {
        match other {
            NullValueGraphQl::NullValue => Self::NullValue,
        }
    }
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
///
///  The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
#[derive(
    :: async_graphql :: Enum,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "NullValue")]
pub enum NullValueGraphQl {
    /// Null value.
    NullValue = 0,
}
pub use self::NullValueGraphQl as NullValueGraphQlInput;
/// A Timestamp represents a point in time independent of any time zone or local
/// calendar, encoded as a count of seconds and fractions of seconds at
/// nanosecond resolution. The count is relative to an epoch at UTC midnight on
/// January 1, 1970, in the proleptic Gregorian calendar which extends the
/// Gregorian calendar backwards to year one.
///
/// All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
/// second table is needed for interpretation, using a [24-hour linear
/// smear](https://developers.google.com/time/smear).
///
/// The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
/// restricting to that range, we ensure that we can convert to and from [RFC
/// 3339](https://www.ietf.org/rfc/rfc3339.txt) date strings.
///
/// # Examples
///
/// Example 1: Compute Timestamp from POSIX `time()`.
///
///     Timestamp timestamp;
///     timestamp.set_seconds(time(NULL));
///     timestamp.set_nanos(0);
///
/// Example 2: Compute Timestamp from POSIX `gettimeofday()`.
///
///     struct timeval tv;
///     gettimeofday(&tv, NULL);
///
///     Timestamp timestamp;
///     timestamp.set_seconds(tv.tv_sec);
///     timestamp.set_nanos(tv.tv_usec * 1000);
///
/// Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
///
///     FILETIME ft;
///     GetSystemTimeAsFileTime(&ft);
///     UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
///
///     // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
///     // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
///     Timestamp timestamp;
///     timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
///     timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
///
/// Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
///
///     long millis = System.currentTimeMillis();
///
///     Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
///         .setNanos((int) ((millis % 1000) * 1000000)).build();
///
///
/// Example 5: Compute Timestamp from Java `Instant.now()`.
///
///     Instant now = Instant.now();
///
///     Timestamp timestamp =
///         Timestamp.newBuilder().setSeconds(now.getEpochSecond())
///             .setNanos(now.getNano()).build();
///
///
/// Example 6: Compute Timestamp from current time in Python.
///
///     timestamp = Timestamp()
///     timestamp.GetCurrentTime()
///
/// # JSON Mapping
///
/// In JSON format, the Timestamp type is encoded as a string in the
/// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
/// format is "{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z"
/// where {year} is always expressed using four digits while {month}, {day},
/// {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
/// seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
/// are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
/// is required. A proto3 JSON serializer should always use UTC (as indicated by
/// "Z") when printing the Timestamp type and a proto3 JSON parser should be
/// able to accept both UTC and other timezones (as indicated by an offset).
///
/// For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
/// 01:30 UTC on January 15, 2017.
///
/// In JavaScript, one can convert a Date object to this format using the
/// standard
/// [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
/// method. In Python, a standard `datetime.datetime` object can be converted
/// to this format using
/// [`strftime`](https://docs.python.org/2/library/time.html#time.strftime) with
/// the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
/// the Joda Time's [`ISODateTimeFormat.dateTime()`](
/// http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime%2D%2D
/// ) to obtain a formatter capable of generating timestamps in this format.
///
///
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Timestamp")]
pub struct TimestampGraphQl {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    pub nanos: i32,
}
/// A Timestamp represents a point in time independent of any time zone or local
/// calendar, encoded as a count of seconds and fractions of seconds at
/// nanosecond resolution. The count is relative to an epoch at UTC midnight on
/// January 1, 1970, in the proleptic Gregorian calendar which extends the
/// Gregorian calendar backwards to year one.
///
/// All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
/// second table is needed for interpretation, using a [24-hour linear
/// smear](https://developers.google.com/time/smear).
///
/// The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
/// restricting to that range, we ensure that we can convert to and from [RFC
/// 3339](https://www.ietf.org/rfc/rfc3339.txt) date strings.
///
/// # Examples
///
/// Example 1: Compute Timestamp from POSIX `time()`.
///
///     Timestamp timestamp;
///     timestamp.set_seconds(time(NULL));
///     timestamp.set_nanos(0);
///
/// Example 2: Compute Timestamp from POSIX `gettimeofday()`.
///
///     struct timeval tv;
///     gettimeofday(&tv, NULL);
///
///     Timestamp timestamp;
///     timestamp.set_seconds(tv.tv_sec);
///     timestamp.set_nanos(tv.tv_usec * 1000);
///
/// Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
///
///     FILETIME ft;
///     GetSystemTimeAsFileTime(&ft);
///     UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
///
///     // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
///     // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
///     Timestamp timestamp;
///     timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
///     timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
///
/// Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
///
///     long millis = System.currentTimeMillis();
///
///     Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
///         .setNanos((int) ((millis % 1000) * 1000000)).build();
///
///
/// Example 5: Compute Timestamp from Java `Instant.now()`.
///
///     Instant now = Instant.now();
///
///     Timestamp timestamp =
///         Timestamp.newBuilder().setSeconds(now.getEpochSecond())
///             .setNanos(now.getNano()).build();
///
///
/// Example 6: Compute Timestamp from current time in Python.
///
///     timestamp = Timestamp()
///     timestamp.GetCurrentTime()
///
/// # JSON Mapping
///
/// In JSON format, the Timestamp type is encoded as a string in the
/// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
/// format is "{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z"
/// where {year} is always expressed using four digits while {month}, {day},
/// {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
/// seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
/// are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
/// is required. A proto3 JSON serializer should always use UTC (as indicated by
/// "Z") when printing the Timestamp type and a proto3 JSON parser should be
/// able to accept both UTC and other timezones (as indicated by an offset).
///
/// For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
/// 01:30 UTC on January 15, 2017.
///
/// In JavaScript, one can convert a Date object to this format using the
/// standard
/// [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
/// method. In Python, a standard `datetime.datetime` object can be converted
/// to this format using
/// [`strftime`](https://docs.python.org/2/library/time.html#time.strftime) with
/// the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
/// the Joda Time's [`ISODateTimeFormat.dateTime()`](
/// http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime%2D%2D
/// ) to obtain a formatter capable of generating timestamps in this format.
///
///
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "TimestampInput")]
pub struct TimestampGraphQlInput {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    pub nanos: i32,
}
#[allow(clippy::useless_conversion)]
impl From<Timestamp> for TimestampGraphQl {
    fn from(other: Timestamp) -> Self {
        let Timestamp { seconds, nanos, .. } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<TimestampGraphQl> for Timestamp {
    fn from(other: TimestampGraphQl) -> Self {
        let TimestampGraphQl { seconds, nanos } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Timestamp> for TimestampGraphQlInput {
    fn from(other: Timestamp) -> Self {
        let Timestamp { seconds, nanos, .. } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<TimestampGraphQlInput> for Timestamp {
    fn from(other: TimestampGraphQlInput) -> Self {
        let TimestampGraphQlInput { seconds, nanos } = other;
        Self {
            seconds: seconds.into(),
            nanos: nanos.into(),
        }
    }
}
#[allow(unused_imports)]
pub use ::prost_types::*;
