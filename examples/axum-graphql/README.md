# Axum + GraphQL Example

This is an example of how you could use Prisma Client Rust in a GraphQL backend, written by [Aaron Leopold](https://github.com/aaronleopold). 

## Running

Generate the client:

```bash
cargo prisma generate
```

Then run the server:

```bash
cargo run
```

## Notes

The simple use of `async_graphql` means that queries are done in a less efficient manner than could be,
since `with` is never utilised and relations are loaded separately.
Additionally, dataloader is not utilised because I can't be bothered.

The requirement to redefine all your GraphQL models may be seen by some as a hassle and a downside,
but personally I believe it makes for cleaner separation of database and API as well as cleaner code,
since the model structs will not need to be annotated with GraphQL _and_ ORM attribute macros.
