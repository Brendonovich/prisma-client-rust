---
title: Setup
desc: Setup instructions
layout: ../../layouts/MainLayout.astro
---

If you have completed the [installation steps](installation) and setup the `cargo prisma <command>` alias, you are ready to add the Prisma Client Rust generator to your [Prisma schema](https://www.prisma.io/docs/concepts/components/prisma-schema). Below is an example of a schema that exists at the root of the project, uses a SQLite database and generates the client at `src/prisma.rs`:

```prisma
datasource db {
    provider = "sqlite"
    url      = "file:dev.db"
}

generator client {
    // Corresponds to the cargo alias created earlier
    provider      = "cargo prisma"
    // The location to generate the schema. Is relative to the position of the schema
    output        = "./src/prisma.rs"
}

model User {
    id          String  @id
    displayName String
}
```

Next, run `cargo prisma generate` to generate the client that will be used in your Rust code. If you have `rustfmt` installed, the generated code will be formatted for easier exploration and debugging.

## Creating the Client

First, make sure you are using the [Tokio](https://github.com/tokio-rs/tokio) async runtime. Other runtimes have not been tested, but since the [Prisma Engines](https://github.com/prisma/prisma-engines) use it there is likely no other option.

Using the above schema for reference, this is how to create an instance of the Prisma client in a `main.rs` file right next to `prisma.rs`:

```rust
mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[tokio::main]
async fn main() {
    let client: Result<PrismaClient, NewClientError> = prisma::new_client().await;
}
```

## Naming Clashes

Rust has a [reserved set of keywords](https://doc.rust-lang.org/reference/keywords.html) that cannot be used as names in your code. If you name a model or field something that after conversion to `snake_case` will be a restricted keyword, you will almost assuredly not be able to compile your project.
While this is annoying, it is an unavoidable consequence of using Rust.
