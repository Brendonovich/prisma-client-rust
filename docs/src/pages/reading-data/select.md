---
title: Selecting Fields
layout: ../../layouts/MainLayout.astro
---

Specific fields can be selected to be returned using a combination of the `select` builder function and a model's `select` macro.
This is usually useful for optimizing queries by only returning exactly the data you need,
but it can also be useful since it provides **complete type-safety** when loading relations.

The examples use the following schema:
```prisma
model User {
    id String @id @default(cuid())

    posts Post[] @relation

    comments Comment[]
}

model Post {
    id        String   @id @default(cuid())
    published Boolean
    title     String
    content   String?
    desc      String?

    author_id String
    author    User   @relation(fields: [author_id], references: [id])

    comments Comment[]
}

model Comment {
    id        String   @id @default(cuid())
    content   String

    author_id String
    author    User   @relation(fields: [author_id], references: [id])

    post   Post   @relation(fields: [postID], references: [id])
    postID String
}
```

A basic select of some fields could look like this:

```rust
use prisma::post;

let post: Option<_> = client
    .post()
    .find_unique(post::is::equals("Test".to_string()))
    .select(post::select! {
        // Scalar fields and relation fields can be selected
        id
        content
        comments
    })
    .exec()
    .await?;
```

The type inside the `Option` is not specified as it is anonymous -
it only exists within the `select` call and cannot be accessed outside it.

## The Macro

A model's `select` macro accepts a syntax similar to GraphQL and outputs a few things
including custom `Data` structs that exactly match what you select,
and your selection converted to data that is sent to the Prisma engines.

How a field can be selected depends on its type.

### Scalar Fields

Just put the name of the field

```rust
post::select! { // Will be custom stuct
    id          // containing an `id: String` field
    published   // and a `published: bool` field
}
```

### Single Relation Fields

#### Field Name Only

Will be the relation model `Data` type

```rust
post::select! {
    author // Will have type `prisma::user::Data`
}
```


#### Field Name and Nested Selection

By adding `{ }` after the name of the relation field,
you can create a selection set inside the relation that goes as deep and nested as you want.

```rust
post::select! {
    author { // Will be custom struct
        id   // containing an `id: String` field
    }
}
```

### Many Relation Fields

#### Field Name Only

Will be a `Vec` of the relation model `Data` type

```rust
post::select! {
    comments // Will have type `Vec<prisma::comment::Data>`
}
```

#### Field Name + Nested Selection

Essentially just a `Vec` of single relation nested selections.

```rust
post::select! {
    comments {  // Will be a Vec of a custom struct
        id      // containing an `id: String` field
        content // and a `content: String` field
    }
}
```

#### Field Name + Query Builder Arguments

Adding `()` to the field name - basically treating it like a function call - 
turns the field name into shorthand for `model::field::fetch()`, so you can pass in filters and call builder functions
like you would when using `with/fetch`!

```rust
post::select! {
    // Becomes post::comments::fetch(..)
    comments(vec![comment::content::contains("fetch!".to_string())])
        .skip(5)
        .take(10) { // You can do nested selections too!
        id
        content
    }
}
```

## The Builder Function

`select` only takes one argument - the output of a model's select macro. 
It should only be called after you've specified all other query arguments,
as the only thing you can do once you call `select` is execute the query.

Since relations can be fetched in the macro, any previous calls to `with` are ignored.
