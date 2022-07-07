# Scalar Lists

Prisma Client Rust supports filtering and writing scalar list fields with a variety of methods.

NOTE: Not all of these methods are available for every database: SQLite does not support scalar lists at all, and some methods are only available in PostgreSQL and MongoDB. Check the [Prisma Client reference](https://www.prisma.io/docs/reference/api-reference/prisma-client-reference) for more information.

NOTE: `NULL` values in scalar lists [require some extra consideration](https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-scalar-lists-arrays#filtering-scalar-lists)

The examples use the following Prisma schema:

```prisma
model Post {
    id    String @id @default(cuid())
    title String
    tags  String[]
}
```

## Filtering

The `has`, `has_every` and `has_some` methods allow for filtering on whether a list contains the given elements.

```rust
use prisma::post;

let post: Option<post::Data> = client
    .post()
    .find_first(vec![
        // Whether the list contains a single value
        post::tags::has("coffee".to_string()),
        // Whether the list cotains all given values
        post::tags::has_every(vec![
            "coffee".to_string(),
            "juice".to_string()
        ]),
        // Whether the list contains at least one of the given values
        post::tags::has_some(vec![
            "coffee".to_string(),
            "tea".to_string()
        ])
    ])
    .exec()
    .await
    .unwrap();
```

There is also the `is_empty` method which filters on if a list is empty or not.

```rust
use prisma::post;

let post: Option<post:Data> = client
    .post()
    .find_first(vec![
        // Makes sure tags is empty
        post::tags::is_empty(true)
    ])
    .exec()
    .await
    .unwrap()
```

## Writing

The `set` method overwrites the existing list value.

```rust
use prisma::post;

let post: Option<post::Data> = client
    .post()
    .find_unique(post::id::equals("123".to_string()))
    .update(post::tags::set(vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string()
    ]))
    .exec()
    .await
    .unwrap();
```

The `push` method adds items to the end of a list.

```rust
use prisma::post;

let post: Option<post::Data> = client
    .post()
    .find_unique(post::id::equals("123".to_string()))
    .update(post::tags::push(vec![
        "a".to_string(),
        "b".to_string()
    ]))
    .exec()
    .await
    .unwrap();
```

## Up Next

If the Rust client doesn't support a query you want to do, you can use [raw queries](13-raw.md).
