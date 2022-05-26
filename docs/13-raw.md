# Raw

Sometimes the methods exposed by Prisma Client Rust cannot express the query you need. In this case, the client's `_query_raw` and `_execute_raw` can be used to send raw SQL to your database with fully sanitised arguments.

The `raw` macro exported from `prisma_client_rust` takes a SQL statement as its first argument, followed by `PrismaValue`s, similar to how the `format` and `print` macros operate, using `{}` wherever a variable is used - Prisma Client Rust will update the query with the database-specific variable identifier for you! Unlike `format` and `print` however, `raw` does not validate the number of arguments you provide it at compile time - only at runtime.

If the arguments you want to provide are constructed dynamically, and as such cannot be specified in the `raw` macro, you can import the `Raw` struct and create one manually by calling `new` with the SQL query and a `Vec` of `PrismaValue`s.

The examples use the following Prisma schema:

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

## Query

Use `_query_raw` to query for data. It has a generic type that may be inferred or explicity specified the indicates the return type of the query.
The return type must implement `serde::Deserialize`.

```rust
use prisma_client_rust::raw;
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryReturnType {
    id: String,
    title: String
}

let data: QueryReturnType = client
    ._query_raw(raw!(
        "SELECT id, title FROM Post WHERE id != {}",
        PrismaValue::String("NotThisID".to_string())
    ))
    .exec()
    .await
    .unwrap();
```

## Execute

Use `_execute_raw` to make modifications to data. It returns the number of rows that were created or updated.

```rust
let count = client
    ._execute_raw(raw!(
        "INSERT INTO Post (published, title) VALUES ({}, {})",
        PrismaValue::Boolean(false),
        PrismaValue::Title("A Title".to_string())
    ))
    .exec()
    .await
    .unwrap();

assert_eq!(count, 1);
```

## Up Next

Now that you know all about performing queries, it's worth learning how to handle [when things go wrong](14-error-handling.md)
