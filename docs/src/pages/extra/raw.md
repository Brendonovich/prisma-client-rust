---
title: Raw Queries
desc: Raw queries documentation
layout: ../../layouts/MainLayout.astro
---

Sometimes the methods exposed by Prisma Client Rust cannot express the query you need. In this case, the client's `_query_raw` and `_execute_raw` can be used to send raw SQL to your database with fully sanitised arguments.

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

## `_query_raw`

Use `_query_raw` for reading data.
The return type is a `Vec` of a generic you specify, which must implement `serde::Deserialize`.
The generic represents the shape of a row returned by the query.

See <a href="https://github.com/Brendonovich/prisma-client-rust/blob/0.6.2/src/raw.rs#L119-L139" target="_blank">this enum</a> for a reference of how database types map to Rust types.

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

## `_execute_raw`

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
