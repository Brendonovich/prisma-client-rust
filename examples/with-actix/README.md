# Actix Web Example

This is an example of using [Prisma Client Rust](https://github.com/Brendonovich/prisma-client-rust) with [Actix Web](https://actix.rs/) and connect with [Prisma Data Platform](https://www.prisma.io/data-platform).

## Running

Build prisma schema

```bash
cargo prisma generate
```

Push schema to database

```bash
cargo prisma db push
```

Run server

```bash
cargo run
```

the server will be on [http://localhost:3001](http://localhost:3001)
