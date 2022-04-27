# Upsert

Upserting allows you to update a record if it exists, or create it if it does not.

To perform an upsert, first do a `upsert` query for a record with a unique filter, then specify the upsert behaviours using the `create` and `update` functions.

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

The following example searches for a post, updates it if it is found, and creates it if not.

```rust
use prisma::post;

let post: post::Data = client
    .post()
    .upsert(post::id::equals("upsert".to_string()))
    .create(
        post::published::set(true),
        post::title::set("title".to_string()),
        post::id::set("upsert".to_string()),
        vec![]
    )
    .update(vec![
        post::content::set(Some("new content".to_string())),
        post::views::increment(1)
    ])
    .exec()
    .await
    .unwrap();
```

## Up Next

Need to work with scalar lists? [We've got you covered!](12-scalar-lists.md)
