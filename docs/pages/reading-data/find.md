# Find Queries

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

## Find Many

`find_many` searches for all records of a model matching the provided filters.

```rust
use prisma::post;

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![post::title::equals("Title".to_string())])
    .exec()
    .await
    .unwrap()
```

If no records are found, `find_many` will return an empty vector instead of an error.

## Find First

`find_first` searches for the first record of a model that matches the provided filters. Like `find_many`, it requires a list of filters.

```rust
use prisma::post;

let posts: Option<post::Data> = client
    .post()
    .find_first(vec![post::title::id("123".to_string())])
    .exec()
    .await
    .unwrap()
```

## Find Unique

`find_unique` searches for a single record of a model matching the provided unique filter. A unique filter is an `equals()` filter of a unique field.

If a matching record is not found, the result of the query will be `None` rather than throwing an error.

```rust
use prisma::post;

let posts: Option<post::Data> = client
    .post()
    .find_unique(post::id::equals("123".to_string()))
    .exec()
    .await
    .unwrap()
```

## Filtering on Relations

Filtering on relations can be done in a similar way to filtering on scalars, it just takes some extra functions.

### Single Relations

For single relations, there is the `is` and `is_not` filters.

The following example gets all comments whose post has the title "My Title":

```rust
use prisma::{comment, post};

let comments: Vec<comment:Data> = client
    .comment()
    .find_many(vec![
        comment::post::is(vec![
            post::title::equals("My Title".to_string())
        ])
    ])
    .exec()
    .await
    .unwrap();
```

### Many Relations

For many relations, there are the `some`, `every` and `none` filters.

The following example gets posts which have at least one comment with the content "My Content" and whose titles are all "My Title"

```rust
use prisma::{post, comment};

let posts: Vec<post::Data> = client
    .post()
    .find_many(vec![
        post::title::equals("My Title".to_string()),
        post::comments::some(vec![
            comment::content::equals("My Content".to_string())
        ])
    ])
    .exec()
    .await
    .unwrap();
```

Note that an empty `some` filter will match every record with at least one linked record, and an empty `none` filter will match every record with no linked records.

## Operator Filters

The operators `and`, `or` and `not` can be used inside any query. The `prisma_client_rust` library exports the `Operator` enum and some helper functions from `prisma_client_rust::operator`, and are one way to use the operators:

```rust
use prisma::post;
use prisma_client_rust::operator::not;

let posts: Option<post::Data> = client
    .post()
    .find_first(vec![
        not(vec![post::title::id("123".to_string()))
    ])
    .exec()
    .await
    .unwrap()
```

This syntax leaves something to be desired, however, since all the helper functions take a `Vec` of filters, leaving a bunch of `vec!` macros in the query.

To aid this, `prisma_client_rust` also exports the `and!`, `or!` and `not!` macros from its root, which can be used in place of an operator and its `vec!`:

```rust
use prisma::post;
use prisma_client_rust::not;

let posts: Option<post::Data> = client
    .post()
    .find_first(vec![
        not![post::title::id("123".to_string())]
    ])
    .exec()
    .await
    .unwrap()
```

Keep in mind that an operator macro must still be within a `vec!`, since it resolves to a single filter.
