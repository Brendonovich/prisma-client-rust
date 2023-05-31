# Ordering

Ordering can be performed on any field, though it is recommended to only order by indexed fields for improved performance.

Order is defined using a field module's `order` function, which takes a `prisma_client_rust::Direction`.
It can be performed on `find_first` and `find_many` queries, as well as being chained onto `fetch` calls for many relations in a similar manner to [relation pagination](pagination#relation-pagination).

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

## Indexed Fields

The following exaple will be order `posts` by `id` from lowest to highest

```rust
use prisma::post;
use prisma_client_rust::Direction;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![])
    .order_by(post::id::order(Direction::Asc))
    .exec()
    .await
    .unwrap();
```

## Non-Indexed Fields

The following example will be order `posts` by `created_at`, even though it is not an indexed field.

```rust
use prisma::post;
use prisma_client_rust::Direction;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![])
    .order_by(post::created_at::order(Direction::Asc))
    .exec()
    .await
    .unwrap();
```

## Combining With Pagination

The following example will order all `post` records and then paginate a selection of them.

```rust
use prisma::post;
use prisma_client_rust::Direction;

let posts: Vec<post::Data> = client
    .post()
    .find_many()
    .order_by(post::created_at::order(Direction::Desc))
    .cursor(post::id::equals("abc".to_string()))
    .take(5)
    .exec()
    .await
    .unwrap();
```
