# rspc Example

This is an example of using Prisma Client Rust with [rspc](https://rspc.dev).

## Running

First, uncomment the `rspc` feature in `prisma-cli` (at the root of this repository).

Then generate the prisma client

```bash
cargo prisma generate
```

Push schema to sqlite database

```bash
cargo prisma db push
```

Run router & generate TypeScript bindings

```bash
cargo run
```

This example does not include a web server to make the router accessible,
it is just a demonstration of using Prisma inside resolvers and generating TypeScript bindings.
