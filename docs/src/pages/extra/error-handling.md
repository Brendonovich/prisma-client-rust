---
title: Error Handling
layout: ../../layouts/MainLayout.astro
---

The errors produced by executing queries can be confusing, containing very Prisma-specific types. For this reason, some utilities and explanation are provided.

Query errors resemble the following:

```rust
pub enum Error {
    Execute(user_facing_errors::Error),
    Serialize(serde_json::Error),
    Deseiralize(serde_json::Error)
}
```

`Serialize` and `Deserialize` errors are fairly self-explanatory, and occur while converting the data returned from Prisma into its appropriate structs. Looking at the [serde documentation](https://serde.rs/error-handling.html) can be helpful in handling serde errors.

`Execute` errors take place when sending a query to the Prisma engines, executing it, and receiving the results. The data contained inside them are an error type provided by Prisma, which contain a lot of deeply nested - and likely not useful - data about the specific error that occurred.

To handle this error type nicely, Prisma Client Rust exports the `error_is_type` function to check if a general `UserFacingError` is a specific Prisma error.
It works by checking first if the provided error is a `KnownError` - one that Prisma provides an error code and information for - 
and if so, whether the error is the type that you provide in its generic argument,
which must be a type of `UserFacingError` provided by `prisma_client_rust::prisma_errors`.

Below is an example of how to check if an error is the result of a unique constraint being violated.
It checks for the `UniqueKeyViolation` error from `prisma_client_rust::user_facing_errors::query_engine`.

```rust
use prisma_client_rust::{
    prisma_errors::query_engine::UniqueKeyViolation,
    error_is_type
};

// ...

if error_is_type::<UniqueKeyViolation>(error) {
    // error results from a create/update violating a unique constraint
}
```
