# Raw Queries

Sometimes the methods exposed by Prisma Client Rust cannot express the query you need. In this case, you can use the client's raw query capabilities to send arbitrary queries to your database.

## Relational Databases

`_query_raw` and `_execute_raw` can be used to send raw SQL to your database with fully sanitised arguments.

The `prisma_client_rust::raw` macro takes an SQL query as its first argument, followed by query variables of type `prisma_client_rust::PrismaValue`.
To specify where in the query the variables should be inserted, use `{}`.
Prisma Client Rust will take care of inserting the correct database specific variable identifier for you.

Even though `raw` appears similar to `format` and `print`, it will not compile-time validate that the number of variables you provide matches the number of `{}` in the query. That will only happen at runtime.

If the arguments you want to provide are constructed dynamically, and as such cannot be specified in the `raw` macro, you can import the `Raw` struct and create one manually by calling `new` with the SQL query and a `Vec` of `PrismaValue`s.

The examples use the following Prisma schema and assume a SQLite database:

```prisma
model Post {
    id        String   @id @default(cuid())
    createdAt DateTime @default(now())
    updatedAt DateTime @updatedAt
    published Boolean
    title     String
    content   String?
    views     Int      @default(0)
}
```

### `_query_raw`

Use `_query_raw` for reading data.
The return type is a `Vec` of a generic you specify, which must implement
[`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html).
The generic represents the shape of a row returned by the query.

See <a href="https://github.com/Brendonovich/prisma-client-rust/blob/0.6.3/src/raw.rs#L119-L139" target="_blank">this enum</a> for a reference of how database types map to Rust types.

```rust
use prisma_client_rust::{raw, PrismaValue};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryReturnType {
    id: String,
    title: String
}

let data: Vec<QueryReturnType> = client
    ._query_raw(raw!(
        "SELECT id, title FROM Post WHERE id != {}",
        PrismaValue::String("NotThisID".to_string())
    ))
    .exec()
    .await?;
```

### `_execute_raw`

Use `_execute_raw` for writing data. It returns the number of rows that were modified.

```rust
use prisma_client_rust::{raw, PrismaValue};

let count = client
    ._execute_raw(raw!(
        "INSERT INTO Post (published, title) VALUES ({}, {})",
        PrismaValue::Boolean(false),
        PrismaValue::String("A Title".to_string())
    ))
    .exec()
    .await?;

assert_eq!(count, 1);
```

## MongoDB

_Available since v0.6.7_

When using MongDB,
the client exposes multiple functions for performing raw queries that use [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)
as arguments.

All of them return a generic type that must implement
[serde::Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html). 

### `_run_command_raw`

Runs an arbitrary command against the database,
accepting all
[MongoDB database commands](https://www.mongodb.com/docs/manual/reference/command/)
except for:

- `find` (use [`find_raw`](#find_raw) instead)
- `aggregate` (use [`aggregate_raw`](#aggregate_raw) instead)

There are a few rules around using this query,
which are documented in [Prisma's documentation](https://www.prisma.io/docs/concepts/components/prisma-client/raw-database-access#runcommandraw)
(their equivalent is `$runCommandRaw`).

```rust
use serde_json::{json, Value};

let data = client
	._run_command_raw::<Vec<Value>>(json!({
        "insert": "Post",
        "documents": [{
            "_id": "1",
            "title": "Post One"
        }]
    }))
	.exec()
	.await?;

assert_eq!(data.len(), 1);
```

### `find_raw`

Returns actual database records for a given model.

**Methods**

- `filter` - Provides the query predicate filter
- `options` - Additional options to pass to the 
[`find` command](https://www.mongodb.com/docs/manual/reference/command/find/#command-fields)


```rust
use serde_json::{json, Value};

let res = client
	.post()
	.find_raw::<Vec<Value>>()
	.filter(json!({ "title": { "$eq": "Some Title" } }))
	.options(json!({ "projection": { "_id": false } }))
	.exec()
	.await?;
```

### `aggregate_raw`

Returns aggregated database records for a given model.

**Methods**

- `pipeline` - An [aggregation pipeline](https://www.mongodb.com/docs/manual/reference/operator/aggregation-pipeline/)
- `options` - Additional options to pass to the 
[`aggregate` command](https://www.mongodb.com/docs/manual/reference/command/aggregate/#command-fields)

```rust
use serde_json::{json, Value};

let res = client
	.post()
	.aggregate_raw::<Vec<Value>>()
	.pipeline(json!([
		{ "$match": { "title": "Title" } },
		{ "$group": { "_id": "$title" } }
	]))
	.exec()
	.await?;
```
