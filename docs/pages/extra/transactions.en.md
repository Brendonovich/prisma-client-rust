# Transactions

_Available since v0.6.4_

While batching can cover most use cases where queries need to succeed or fail together,
it doesn't allow you to run code that executes between each query.
Instead, you can use `PrismaClient::_transaction`,
which provides both closure-based and manual methods of executing individual queries and arbitrary code inside a transaction.

Both methods provide the ability to commit and roll back a transaction,
and produce a dedicated instance of `PrismaClient` that must be used while executing the transaction.

## Transaction Closures

Running your transaction in a closure is the approach used by the official
[Prisma client](https://www.prisma.io/docs/concepts/components/prisma-client/transactions#interactive-transactions).
It can be nice as all of your transaction's code can be kept in one place,
but it has the downside that closures can be tricky to work with.

To perform a transaction this way,
just call `client._transaction().run(..)`
and provide a closure returning an `async move` block to `run()`.
The closure should accept one argument (the dedicated `PrismaClient` instance),
and return a `Result`.

If the closure returns `Ok`,
the transaction will attempt to commit itself,
and if it returns `Err` it will attempt to roll back.

```rust
let (user, post) = client
	._transaction()
	.run(|client| async move {
		let user = client
			.user()
			.create("brendan".to_string(), vec![])
			.exec()
			.await?;

		client
			.post()
			.create(
				"test".to_string(),
				true,
				vec![post::author::connect(
					user::id::equals(user.id.clone())
				)],
			)
			.exec()
			.await
			// if query succeeds, return user + post from transaction
			.map(|post| (user, post))
	})
	.await?;
```

### Error Types

Transaction closures must return a `Result`,
but the `Err` generic can be almost anything,
and the `Ok` generic is not restricted at all.

To allow using `?` inside transaction closures,
error types must implement `From<prisma_client_rust::QueryError>`
(this includes `QueryError` itself if you don't need a custom error type).
This can be done either with a manual implementation:

```rust
use prisma_client_rust::QueryError;

enum CustomError {
		QueryError(QueryError)
}

impl From<QueryError> for CustomError {
		fn from(e: QueryError) { ... }
}
```

or via a library like [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) with its `#[from]` attribute:

```rust
#[derive(thiserror::Error)]

enum CustomError {
		#[error("Database error occurred")]
		QueryError(prisma_client_rust::QueryError),
		...
}
```

### Specifying The Error Type

1. Use the generic parameter directly. This works,
but requires `_` for the rest of the `run`'s generic parameters,
which probably isn't desirable.

```rust
cilent
		._transaction()
		.run::<CustomError, _, _, _>(..)
		.await?;
```
2. Type casting. If your closure returns `Ok`,
you can cast it to a `Result` with the appropriate error type.

```rust
client
		._transaction()
		.run(|client| async move {
				let user = client
						.user()
						.create("brendan".to_string(), vec![])
						.exec()
						.await?;

				Ok(user) as Result<_, CustomError>;
		})
		.await?
```

3. Returning a query's `Result` -
this is probably the nicest looking solution.
If you are using a custom error type,
use `map_err` after `await` to transform the `QueryError` into your custom error type.

```rust
client
		._transaction()
		.run(|client| async move {
				client
						.user()
						.create("brendan".to_string(), vec![])
						.exec()
						// No `?` so that `Result` with error type is returned
						.await
		})
		.await?
```


## Manual Transactions

If you'd prefer to manually control when the transaction commits and rolls back,
use `client._transaction().begin()` to not only get a dedicated `PrismaClient`,
but also a `TransactionManger` instance that you can `commit` and `rollback` with:

```rust
let (tx, client) = client
		._transaction()
		.begin()
		.await?;
```

The above example names the client instance `client`,
meaning that it would shadow the original client it was created from,
making it inaccessible.
You could give the client instance a name like `tx_client`,
or put all transaction logic inside a block so that the original `client` variable
isn't shadowed in the rest of your code.

`commit` and `rollback` consume the client created by `begin` as their only argument.
This is done because those functions need to do things with the client,
and as an extra precaution against the transaction-specific client being used once the transaction is complete.

```rust
tx.commit(client).await?;
// or 
tx.rollback(client).await?;
```


### Error Handling

Care must be taken when handling errors using this method.
Simply using `?` could result in your code returning before `commit` or `rollback` is ran.
An easy way to avoid this is to put your transaction logic in a function where it is safe to use `?`,
and then `commit` or `rollback` based on the result of the function.
```rust
let (tx, client) = client
		._transaction()
		.begin()
		.await?;

async fn do_stuff(client: &PrismaClient) -> ... {
		let user = client
				.user()
				.create("brendan".to_string(), vec![])
				.exec()
				.await?; // Early return won't escape transaction

		...
}

// This is very similar to the closure method's internals
let result = match do_stuff(client).await {
		Ok(v) => {
				tx.commit(client).await?;
				Ok(v)
		},
		Err(e) => {
				tx.rollback(client).await?;
				Err(e)
		}
};
```
