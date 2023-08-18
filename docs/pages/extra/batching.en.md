# Batching

`PrismaClient::_batch` allows you to sequentially execute multiple queries in a single transaction.
If one of the queries fails, all changes will be rolled back.

Data provided to `_batch` falls under two categories:

- Containers: Root level collections that contain all items in the batch.
Can be either a tuple or a type implementing `IntoIter`.

- Items: Either a query or a collection (`Vec` or tuple) of nested queries.

## Containers

Even if your batch doesn't include nested items,
you still need to put queries inside some sort of container.

### Tuple

Using a tuple allows for multiple types of queries to be used at once.
The return type of `_batch` will be a tuple of the results of each item.

```rust
use prisma::user;

let (user_one, user_two, user_count): (user::Data, user::Data, i64) = client
    ._batch((
        client.user().create(..),
        client.user().create(..),
        client.user().count(vec![]),
    ))
    .await?;

assert_eq!(user_count, 2);
```

### Iterator

Using a type that implements `IntoIter` such as `Vec` allows for
a dynamic number of items to be batched together.
The return type will be a `Vec` of the result of the item.

```rust
use prisma::user;

let users: Vec<user::Data> = client
    ._batch(vec![
        client.user().create(..),
        client.user().create(..),
        client.user().create(..)
    ])
    .await?;

assert_eq!(users.len(), 3);
```

`IntoIter` includes regular iterators,
so you can pass them straight into `_batch` and
`collect` will be called internally.

```rust
use prisma::user;

let user_ids = vec![1, 2, 3, 4, 5];

let user_creates = user_ids
    .into_iter()
    .map(|id| client
        .user()
        .create(..)
    ); // _batch will collect internally!

let users: Vec<user::Data> = client
    ._batch(user_creates)
    .await?;

assert_eq!(users.len(), 5);
```

## Items

Items can be either individual queries or a collection.

### Tuple

The same logic applies to a tuple item as a tuple container,
with the item's result being a 1-1 mapping of each query to its result type.

```rust
let data: Vec<(user::Data, post::Data)> = client
	._batch(vec![
		(client.user().create(..), client.post().create(..)),
		(client.user().create(..), client.post().create(..)),
	])
	.await?;
```

### Vec

Unlike containers, only `Vec` can be used for dynamic collections of queries.
Apart from that, the behaviour is the same.

```rust
let data: (Vec<user::Data>, Vec<post::Data>) = client
	._batch((
		vec![client.user().create(..), client.user().create(..)],
		vec![client.post().create(..), client.post().create(..)],
	))
	.await?;
```

### Nesting

Any combination and nesting of items can be put inside a container,
allowing for heavily nested return types.

This example isn't really practical, but it's possible.

```rust
let data: Vec<(
	Vec<(user::Data, post::data)>,
	(Vec<user::Data>, Vec<post::Data>),
)> = client._batch(vec![(
	vec![
		(client.user().create(..), client.post().create(..)),
		(client.user().create(..), client.post().create(..)),
	],
	(vec![client.user().create(..)], vec![client.post().create(..)]),
)]);
```
