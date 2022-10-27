# rspc Example

This is an example of using Prisma Client Rust with [rspc](https://rspc.dev).

## Running

Generate the client:

```bash
cargo prisma generate
```

Then run the example:

```bash
cargo run
```

This example does not include a web server to make the router accessible,
it is just a demonstration of using Prisma inside resolvers and generating TypeScript bindings.
