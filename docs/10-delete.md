# Delete

Deleting records is as simple as doing a query and chaining `delete()` before the query is executed.

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

## Deleting One Record

To delete a single record, perform a `delete` query.

The following example finds a single post and deletes it, returning the deleted post.

```rust
use prisma::post;

let deleted_post: Option<post::Data> = client
    .post()
    .delete(post::id::equals("id".to_string()))
    .exec()
    .await
    .unwrap();
```

Note that the query returns an `Option` and not just the data directly, since the record to delete may not exist.

## Delete Many Records

To delete many records, perform a `delete_many` query.

The following example finds a group of comments and deletes them, returning the number of deleted comments.

```rust
use prisma::comment;

let deleted_comments_count: usize = client
    .comment()
    .delete_many(vec![
        comment::content::contains("what's up".to_string())
    ])
    .exec()
    .await
    .unwrap();
```

## Up Next

Want to update a record even if it doesn't exist? [Upserting](11-upsert.md) might be useful!
