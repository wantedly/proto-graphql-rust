# Protobuf -> GraphQL Schema Conversion

## Objects that can be converted almost as-is

These Protobuf objects can be converted to GraphQL objects almost as-is.

### Protobuf message -> GraphQL type

Protobuf:

```proto
message Message {
  string value = 1;
}
```

GraphQL:

```graphql
type Message {
  value: String!
}
```

### Protobuf enum -> GraphQL enum

Protobuf:

```proto
enum Enum {
  A = 1;
  B = 2;
}
```

GraphQL:

```graphql
enum Enum {
  A
  B
}
```

## Objects that cannot be converted as-is

### Protobuf oneof -> GraphQL union

#### Problem

GraphQL union is similar to Protobuf oneof, but the type of a field in a GraphQL union must be different from other fields.

For example, the following Protobuf oneof cannot be converted directly to GraphQL union:

```proto
oneof OneOf {
  string A = 1;
  string B = 2;
}
```

```graphql
# ERROR!
union OneOf = String | String
```

#### Solution in this implementation

In this implementation, we avoided this problem by defining a new type for each field.

```graphql
union OneOf = OneOfA | OneOfB
type OneOfA {
  a: String!
}
type OneOfB {
  b: String!
}
```

The original field name is still there as the suffix of the generated type and its field name.

### GraphQL union in input object

#### Problem

Protobuf oneof can be used for both request and response, but GraphQL union can only be used for response (output).

See also:

- [graphql/graphql-spec#488](https://github.com/graphql/graphql-spec/issues/488)
- <https://brunoscheufler.com/blog/2019-05-19-reaching-consensus-graphql-input-union>

#### Solution in this implementation

In this implementation, we use [the way mentioned in async-graphql/async-graphql#373](https://github.com/async-graphql/async-graphql/issues/373#issuecomment-753761917) as workaround.

In this way, for the following union:

```graphql
union Union = A | B
```

Generate the following input type:

```graphql
input UnionInput {
  A: A
  B: B
}
```

This is similar to the way called "Directive" in [graphql/graphql-spec#488](https://github.com/graphql/graphql-spec/issues/488).

This way has the advantage of supporting unions with overlapping field types, but it has some disadvantages such as the difference between the schema representation (type with multiple optional fields) and the actual processing (union).

It also needs to be checked when actually implementing the object transformation to make sure that multiple fields are not specified at the same time.

### Empty objects in GraphQL

#### Problem

Empty messages and services are supported in Protobuf, but empty objects and queries are not supported in GraphQL.

Protobuf:

```proto
message Empty {}
```

GraphQL:

```graphql
# ERROR!
type Empty {}
```

See also:

- [graphql/graphql-spec#568](https://github.com/graphql/graphql-spec/issues/568)
- [graphql/graphql-js#937](https://github.com/graphql/graphql-js/issues/937)

However, some implementations seem to allow it:

- [graph-gophers/graphql-go#209](https://github.com/graph-gophers/graphql-go/issues/209)

Rust's GraphQL server implementations do not currently allow it:

- [async-graphql/async-graphql#315](https://github.com/async-graphql/async-graphql/issues/315)
- [graphql-rust/juniper#172](https://github.com/graphql-rust/juniper/issues/172)

#### Solution in this implementation

The workaround for this is to use Boolean no-op fields, as mentioned in [graphql/graphql-spec#568](https://github.com/graphql/graphql-spec/issues/568) and [graphql/graphql-js#937](https://github.com/graphql/graphql-js/issues/937).

For example, the above Protobuf will be converted to the following GraphQL type:

```graphql
type Empty {
  _noop: Boolean
}
```

The case where no query is defined is not allowed as well, so we define no-op query to avoid this case as well.

```graphql
type NoopQuery {
  _noop: Empty!
}
```

### Map in GraphQL

#### Problem

Types such as Protobuf's [map](https://developers.google.com/protocol-buffers/docs/proto#maps) are not supported in GraphQL.

```proto
message Map {
  map<string, string> map = 1;
}
```

#### Solution in this implementation

This implementation uses [Scalar types](https://graphql.org/learn/schema/#scalar-types) (input/output is a string but can be used as a parsed type in the application).

```graphql
type Map {
  map: JSON!
}
"""
A scalar that can represent any JSON value.
"""
scalar JSON
```

### Namespace in GraphQl

#### Problem

GraphQl does not have a concept like Rust's module or Protobuf's parent message.

So, it is necessary to adjust the name of the GraphQl object to represent the namespace.

#### Solution in this implementation

In this implementation, we treat a single proto file as the root, and all parent messages from it are added as name prefixes in order from the root side.

In this way, the Inner message of the following Protobuf will be "ParentInner".

```proto
message Parent {
  string value = 1;
  message Inner {
    string value = 1;
  }
}
```

```graphql
type Parent {
  value: String!
}
type ParentInner {
  value: String!
}
```

TODO: It might be better if we treat the package name as the root, instead of a single proto file.
