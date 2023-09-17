# Fetching Relations

Relations can be fetched by calling a query builder's `with` function.
The field modules of relation fields will contain `fetch` functions, the results of which can be passed to `with`.
The `fetch` function of a many relation takes a `Vec` of `WhereParam`, while a single relation's `fetch` function takes no arguments.

Once fetched, relation data can be accessed in two ways:
1. (Recommended) Use relation access functions on the parent struct.
These will give you a `Result` containing a reference to the relation data.
This is the recommended approach as the `Result` error type will inform you that the field hasn't been fetched using `with`.

2. Use the data directly on the parent struct.
This gives you direct access to the relation data, where it is wrapped inside an `Option` to determine whether it has been fetched.
Doing this only recommended if you need to take ownership of the relation data, as dealing with nested options can be tricky and not as descriptive as the errors provided by accessor functions.

Whether a relation has been loaded can only be guaranteed at compile time if the accessor's `Result` is not able to panic - ie. `unwrap`, `expect` etc are not called on it.
See [Select & Include](select-include) for completely type-safe ways of fetching relations.

The examples use the following schema:

```prisma
model Post {
    id        String   @id @default(cuid())
    published Boolean
    title     String
    content   String?
    desc      String?

    comments Comment[]
}

model Comment {
    id        String   @id @default(cuid())
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
    // Many relation requires Vec of filters as argument
    .with(post::comments::fetch(vec![]))
    .exec()
    .await
    .unwrap();

let comments: Result<Vec<comment::Data>, String> = post.comments();
```

## Nested Relations

`fetch` returns a query builder, so you can nest calls like `with` to fetch nested relations.

In this example, a post is loaded with its comments, and each comment is loaded with the original post.

```rust
use prisma::{comment, post};

let post: post::Data = client
    .post()
    .find_unique(post::id::equals("0".to_string()))
    .with(post::comments::fetch(vec![])
      .with(comment::post::fetch())
    )
    .exec()
    .await
    .unwrap()
    .unwrap();

// Safe since post::comments::fetch has been used
for comment in post.comments().unwrap() {
    // Safe since comment::post::fetch has been used
    let post = comment.post().unwrap();

    assert_eq!(post.id, "0");
}
```
