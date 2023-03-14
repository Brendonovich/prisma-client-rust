# Upserting

Upserting allows you to update a record if it exists, or create it if it does not.

`upsert` takes three arguments:
1. A unique filter
2. A tuple of create arguments
3. A list of update data

The example uses the following Prisma schema:

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
    .upsert(
        // Unique filter
        post::id::equals("upsert".to_string()),
        // 'create' helper function for constructing a create argument tuple
        post::create(
            true,
            "title".to_string(),
            "upsert".to_string(),
            vec![]
        ),
        // Vec of updates to apply if record already exists
        vec![
            post::content::set(Some("new content".to_string())),
            post::views::increment(1)
        ]
    )
    .exec()
    .await
    .unwrap();
```
