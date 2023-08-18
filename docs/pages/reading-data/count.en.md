# Count

`count` returns the number of records that fit a set of filters.

The examples use the following schema:

```prisma
model Post {
    id        String   @id @default(cuid())
    title     String
    content   String
}
```

```rust
use prisma::comment;

// Passing no filters will count all records
let all_count: usize = client
    .comment()
    .count(vec![])
    .exec()
    .await?;

// Number of records whose title starts with "Post"
let filtered_count: usize = client
    .comment()
    .count(vec![comment::title::starts_with("Post".to_string())])
    .exec()
    .await?;
```
