# Pagination

Pagination allows you to alter which records are returned from `find_first` and `find_many` queries.

All of these methods can used together, and the order they are used in does not matter.

The examples use the following schema:

```prisma
generator client {
    provider = "cargo prisma"
    output = "src/prisma.rs"
}

model Post {
    id        String   @id @default(cuid())
    createdAt DateTime @default(now())
    updatedAt DateTime @updatedAt
    published Boolean
    title     String
    content   String?
    desc      String?

    comments Comment[]
}

model Comment {
    id        String   @id @default(cuid())
    createdAt DateTime @default(now())
    content   String

    post   Post   @relation(fields: [postID], references: [id])
    postID String
}
```

## Take

To limit the number of records returned, use `take()`:

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![post::title::contains("Title".to_string())])
    .take(5) // query will return the first 5 records, instead of every record
    .exec()
    .await
    .unwrap()

```

## Skip

To skip a number of records, use `skip()`:

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![post::title::contains("Title".to_string())])
    .skip(2) // query will skip the first two records, returning the rest
    .exec()
    .await
    .unwrap()

```

## Cursor

To get the records after a unique field value, use `cursor()`.

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![])
    .cursor(post::id::cursor("abc".to_string()))
    .exec()
    .await
    .unwrap()
```

The [`order_by` method](07-order-by.md) can be very useful when combined with cursor pagination.

## Up Next

Next, check out how to [order queries](07-order-by.md)
