# Pagination

Pagination allows you to specify what range of records are returned from `find_first` and `find_many` queries, and on many relations.

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

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![post::title::contains("Title".to_string())])
    // Only the first 5 records will be returned
    .take(5)
    .exec()
    .await?;

```

## Skip

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![post::title::contains("Title".to_string())])
    // The first 2 records will be skipped
    .skip(2)
    .exec()
    .await?;

```

## Cursor

`cursor` takes a [unique filter](structure#unique-filters) as its argument.

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![])
    .cursor(post::id::equals("abc".to_string()))
    .exec()
    .await?;
```

[`order_by`](order-by.md) can be very useful when combined with cursor pagination.

## Relation Pagination

The above methods can be chained to `fetch` calls for many relations.

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![])
    .with(
        post::comments::fetch(vec![])
            .skip(10)
            .take(5)
            .cursor(comment::id::equals("abc".to_string())),
    )
    .exec()
    .await?;
```
