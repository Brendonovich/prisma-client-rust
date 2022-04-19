# Fetch

The `fetch` method on relation fields allows you to include relation data in your queries, in addition to the original records. Without this you would need 2 separate queries, but `with` and `fetch` allow for it to be done in one.

Accessing a relation field on a model's data cannot be done like regular fields. Instead, an accessor function is available that wraps the relation data inside a `Result` which will be an `Err` if the relation has not been fetched using a `with` call.

Because of this, the loading of relations can only be guaranteed at compile time if the accessor's `Result` is not able to panic - ie. `unwrap`, `expect` etc are not called on it. Some could argue that this is a weakness and that relation fetching should alter the return type, but this cannot be done without a high degree of complexity and usage of macros. This may be explored in the future but for now is out of scope of the project.

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

## Single Relations

In this example, the `post` relation is loaded alongside a comment.

```rust
use prisma::{comment, post};

let comment: Option<comment::Data> = client
    .comment()
    .find_unique(comment::id::equals("0".to_string()))
    .with(comment::post::fetch())
    .exec()
    .await
    .unwrap();

// Since the above query includes a with() call
// the result will be an Ok()
let post: Result<post::Data, String> = comment.post();
```

## Many Relations

In this example, a `post` and its `comments` are loaded.

```rust
use prisma::{comment, post};

let post: Option<post::Data> = client
    .post()
    .find_unique(post::id().equals("0".to_string()))
    .with(post::comments::fetch())
    .exec()
    .await
    .unwrap();

// Since the above query includes a with() call,
// the result will be an Ok()
let comments: Result<Vec<comment::Data>, String> = post.comments();
```

## Up Next

Next, learn how to [paginate your queries](06-pagination.md).