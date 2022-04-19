# Setup

If you have completed the [installation steps](01-installation.md) and setup the `cargo prisma <command>` alias, you are ready to add the Prisma Cliet Rust generator to your [Prisma schema](https://www.prisma.io/docs/concepts/components/prisma-schema). Below is an example of a schema that exists at the root of the project, uses a SQLite database and generates the client at `src/prisma.rs`:

```prisma
datasource db {
    provider = "sqlite"
    url      = "file:dev.db"
}

generator client {
    provider      = "cargo prisma" // Corresponds to the cargo alias created earlier
    output        = "./src/prisma.rs" // The location to generate the schema. Is relative to the position of the schema
}

model User {
    id          String  @id
    displayName String
}
```

### Naming Clashes

Rust has a [reserved set of keywords](https://doc.rust-lang.org/reference/keywords.html) that cannot be used as names in your code. If you name a model or field something that after conversion to `snake_case` will be a restricted keyword, you will almost assuredly not be able to compile your project.
While this is annoying, it is an unavoidable consequence of using Rust.

## Up Next

Next, look at a general overview of [how to use your generated client](03-overview.md)
