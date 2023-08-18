# Structure

## Syntax

After creating an instance of `PrismaClient`, queries can be made like the following:

```rust
client
    .post() // Model to query on
    .find_many(vec![]) // Query to execute
    .exec() // Ends query
    .await; // All queries are async and return Result
```

Queries can be filtered and extended using the generated modifiers.
Each model in your schema gets a corresponding Rust module, with corresponding modules for their fields inside.
Field modules contain functions for constructing modifiers based on each field.


```rust
use prisma::post;

client
    .post()
    .find_many(vec![
//      model::
//            field::
//                   method()
        post::title::equals("Test".to_string())
    ])
    .exec()
    .await;
```   

All model and field module names are converted to `snake_case` as to be consistent with Rust's naming conventions.
