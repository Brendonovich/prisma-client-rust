# Error Handling

The errors produced by executing queries can be confusing, containing very Prisma-specific types. For this reason, some utilities and explanation are provided.

Query errors resemble the following:

```rust
pub enum Error {
    Execute(user_facing_errors::Error),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error)
}
```

`Serialize` and `Deserialize` errors are fairly self-explanatory, and occur while converting the data returned from Prisma into its appropriate structs.
[The serde documentation](https://serde.rs/error-handling.html) can be helpful in handling serde errors.

`Execute` errors take place when sending a query to the Prisma engines, executing it, and receiving the results. The data contained inside them are an error type provided by Prisma, which contain a lot of deeply nested - and likely not useful - data about the specific error that occurred.

To handle this error type nicely, query errors have an `is_prisma_error` function to check if the error is a particular `UserFacingError`.

#### Examples

This example attempts to create a record and checks if a unique key constraint is violated.

```rust
use prisma_client_rust::prisma_errors::query_engine::UniqueKeyViolation;

let user = client
    .user()
    .create(..)
    .exec()
    .await;

match user {
    Ok(user) => println!("User created"),
    Err(error) if error.is_prisma_error::<UniqueKeyViolation>() =>
        println!("Unique key violated")
    Err(error) => println!("Other error occurred")
}
```

This example attempts to update a record and checks if the record being updated does not exist.

```rust
use prisma_client_rust::prisma_errors::query_engine::RecordNotFound;

let user = client
    .user()
    .update(..)
    .exec()
    .await;

match user {
    Ok(user) => println!("User updated"),
    Err(error) if error.is_prisma_error::<RecordNotFound>() =>
        println!("User doesn't exist")
    Err(error) => println!("Other error occurred")
}
```
