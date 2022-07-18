# Update

Updating a record can be done with `update` or `update_many` by specifying which records you would like to update and a `Vec` of all the updates you want to make.

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

## Updating a Unique Record

The following example finds and updates an existing post, with the resulting post data being returned.

```rust
use prisma::post;

let updated_post: post::Data = client
    .post()
    .update(
        post::id::equals("id".to_string()), // Unique filter
        vec![post::title::set("new title".to_string())] // Vec of updates
    )
    .exec()
    .await
    .unwrap();
```

## Updating Many Records

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

Using `link`, relations can be created inside `update` queries.

IMPORTANT: Updating a relation this way should only be done within `update`. Doing so with `update_many` will cause the query to always return an `Err`. To avoid this, set the relation's scalar fields directly.

### Update in a Find Unique

The following example find a comment and updates the post that it is linked to.

```rust
use prisma::{comment, post};

let updated_comment: comment::Data = client
    .post()
    .update(
        comment::id::equals("id".to_string()),
        vec![comment::post::link(
            post::id::equals("post".to_string())
        )]
    )
    .exec()
    .await
    .unwrap();
```

### Update in a Find Many

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

### Unlink Optional Relations

For optional relations, the `unlink` method is available to remove relations and set the relation's scalar field to `NULL` in `update` queries.

The same caveat for `update_many` applies, so setting the scalar fields to `None` shoud be done instead.

## Up Next

Once you're done with your data, it can be helpful to [delete it](10-delete.md)
