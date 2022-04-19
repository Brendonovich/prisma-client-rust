# Overview

Prisma Client Rust aims to be fully-typesafe in a similar manner to the [Go client](https://github.com/prisma/prisma-client-go), as an alternative to existing ORM solutions such as [Diesel](https://diesel.rs/) and [SeaORM](https://www.sea-ql.org/SeaORM/).

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

## Syntax

The generated client uses a functional API syntax that follows Rust's naming conventions.

After creaing an instance of `PrismaClient`, queries can be made like the following:

```rust
let posts = client
    .post() // Model to query on
    .find_many(vec![]) // Query to execute
    .exec() // Ends query
    .await // All queries are async and return Result
```

Filtering queries based on fields can be done with the generated query builders. Functions are exposed for each field of each model, with each model getting its own Rust module, and each field inside that module is its own module.

```rust
use prisma::post;

let posts = client
    .post()
    .find_many(vec![
//      model::
//            field::
//                   method()
        post::title::equals("Test".to_string())
    ])
    .exec()
    .await
```

All model and field module names are converted to `snake_case` as to be consistent with Rust's naming conventions.


## Up Next

Next, dive into the [find queries](04-find.md) that are the basis for how you use the client.