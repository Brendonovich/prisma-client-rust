---
title: Batch Queries
layout: ../../layouts/MainLayout.astro
---

`PrismaClient::_batch` allows you to sequentially execute multiple queries in a single transaction.
If one of the queries fails, all changes will be rolled back.

When providing queries to `_batch` there is no need to call `exec()`,
but all queries must be put inside a valid container type.

## Container Types

### Tuple

Using a tuple allows for multiple types of queries to be used at once.
The return type of `_batch` will be a tuple of the results of each query in the input tuple.

```rust
use prisma::user;

let (user_one, user_two, user_count): (user::Data, user::Data, i64) = client
    ._batch((
        client.user().create(..),
        client.user().create(..),
        client.user().count(),
    ))
    .exec()
    .await?;

assert_eq!(user_count, 2);
```

### Iterator

Using an iterator such as `Vec` allows for a dynamic number of a single type of query to be batched.
The return type will be a `Vec` of the result of the input query type.

```rust
use prisma::user;

let users: Vec<user::Data> = client
    ._batch(vec![
        client.user().create(..),
        client.user().create(..),
        client.user().create(..)
    ])
    .exec()
    .await?;


assert_eq!(users.len(), 3);
```

Since `_batch` accepts any iterator, queries can be constructed from external data without collecting into a `Vec`.

```rust
use prisma::user;

let user_ids = vec![1, 2, 3, 4, 5];

let users: Vec<user::Data> = client
    ._batch(user_ids
        .into_iter()
        .map(|id| client
            .user()
            .create(..)
        ) // _batch will collect internally!
    )
    .exec()
    .await?;


assert_eq!(users.len(), 5);
```
