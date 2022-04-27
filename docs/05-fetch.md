# Fetch

The `fetch` method on relation fields allows you to include relation data in your queries, in addition to the original records. Without this you would need 2 separate queries, but `with` and `fetch` allow for it to be done in one. Loading multiple relations can be done by calling `with` multiple times, each calling `fetch` for a different relation.

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
    .find_unique(post::id::equals("0".to_string()))
    .with(post::comments::fetch())
    .exec()
    .await
    .unwrap();

// Since the above query includes a with() call,
// the result will be an Ok()
let comments: Result<Vec<comment::Data>, String> = post.comments();
```

## Nested Relations

Starting with Prisma Client Rust v0.4.1, relations can be fetched to an unlimited depth.

In this example, a post is loaded with its comments, and each comment is loaded with the original post.

```rust
use prisam::{comment, post};

let post: post::Data = client
    .post()
    .find_unique(post::id::equals("0".to_string()))
    .with(post::comments::fetch().with(comment::post::fetch()))
    .exec()
    .await
    .unwrap()
    .unwrap();

// Safe since post::comments::fetch() has been used
for comment in post.comments().unwrap() {
    // Safe since comment::post::fetch() has been used
    let post = comment.post().unwrap();

    assert_eq!(post.id, "0");
}
```

## Up Next

Next, learn how to [paginate your queries](06-pagination.md).
