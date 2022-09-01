---
title: Delete Queries
desc: Delete query documentation
layout: ../../layouts/MainLayout.astro
---

The examples use the following Prisma schema:

```prisma
model Post {
    id        String   @id @default(cuid())
    createdAt DateTime @default(now())
    updatedAt DateTime @updatedAt
    published Boolean
    title     String
    content   String?

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

## Delete

`delete` will delete the record referenced by a single unique filter,
returning the record's data if it existed.

The following example finds a single post and deletes it, returning the deleted post.

```rust
use prisma::post;

let deleted_post: Option<post::Data> = client
    .post()
    .delete(post::id::equals("id".to_string()))
    .exec()
    .await?;
```

## Delete Many

`delete_many` will delete the records referenced by a `Vec` of any filters and return the number of deleted records.

The following example finds a group of comments and deletes them, returning the number of deleted comments.

```rust
use prisma::comment;

let deleted_comments_count: i64 = client
    .comment()
    .delete_many(vec![
        comment::content::contains("some text".to_string())
    ])
    .exec()
    .await;
```
