---
title: Update Queries
desc: Update query documentation
layout: ../../layouts/MainLayout.astro
---

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

## Update

`update` accepts a single unique filter and a `Vec` of updates to apply, returning the data of the updated record.

The following example finds and updates an existing post, with the resulting post data being returned.

```rust
use prisma::post;

let updated_post: Option<post::Data> = client
    .post()
    .update(
        post::id::equals("id".to_string()), // Unique filter
        vec![post::title::set("new title".to_string())] // Vec of updates
    )
    .exec()
    .await
    .unwrap();
```

## Update Many

`update_many` accepts a `Vec` of filters (not just unique filters), and a `Vec` of updates to apply to all records found.
It returns the number of records updated, not the data of those records.

The following example finds and updates a set of posts. The number of updated records is returned.

```rust
use prisma::post;

let updated_posts_count: usize = client
    .post()
    .update_many(
        vec![post::id::contains("id".to_string())], // Vec of unique filters
        vec![post::content::set("new content".to_string())] // Updates to be applied to each record
    )
    .exec()
    .await
    .unwrap();
```

## Updating Relations

Using `connect` and `disconnect`, relations can be modified inside `update` queries.

IMPORTANT: Updating a relation this way with `update_many` will cause the query to always return an error.
To avoid this, set the relation's scalar fields directly.
An effort to create stricter types to avoid this is being [tracked]().

### Single Record

The following example find a comment and disconnects the post that it is related to.

```rust
use prisma::{comment, post};

let updated_comment: comment::Data = client
    .post()
    .update(
        comment::id::equals("id".to_string()),
        vec![comment::post::disconnect()]
    )
    .exec()
    .await
    .unwrap();
```

### Many Records

The following example finds all comments on a post and updates the post they are linked to, but does so by modifying the relation column directly.

```rust
use prisma::{comment, post};

let updated_comment: comment::Data = client
    .post()
    .update_many(
        vec![comment::post::is(
            post::id::equals("id".to_string())
        )],
        vec![comment::post_id::set("post".to_string())]
    )
    .exec()
    .await
    .unwrap();
```
