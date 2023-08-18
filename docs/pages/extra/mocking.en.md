# Mocking

_Available since v0.6.4_

When writing tests for function which use the Prisma client,
it can be difficult to have a real database running to test with.
To combat this, you can enable the `mocking` feature on `prisma-client-rust` and `prisma-client-rust-cli`
and create a 'mock' client that runs queries using data you provide beforehand.
This allows you to define the expected result of a query and then perform tests using those expected results.

The examples use the following Prisma schema:

```prisma
model Post {
    id    String   @default(cuid()) @id
    title String
}
```

Say you have a function `get_post_title`:

```rust
use prisma::{PrismaClient, post};
use prisma_client_rust::queries;

async fn get_post_title(
    client: &PrismaClient,
    post_id: String,
) -> queries::Result<Option<String>> {
    let post = client.post().find_unique(post::id::equals(post_id)).await?;

    post.map(|post| post.title)
}
```

To write a unit test,
first create a mock client and mock store with `PrismaClient::_mock`,
define your expectations via the mock store,
and run the test.
The mock client will use data from the mock store to resolve queries.

```rust
#[cfg(test)]
mod test {
	use super::*;

	#[tokio::test]
	async fn gets_title() -> queries::Result<()> {
		let (client, mock) = PrismaClient::_mock().await;

		let id = "123".to_string();
		let expected_title = "Test".to_string();

		mock.expect(
			// First argument is query without calling 'exec'
			client.post().find_unique(post::id::equals(post_id)),
			// Second argument is expected return type.
			// This will fail to compile if it does not match
			// the return type of the query
			post::Data {
				id: id.clone(),
				title: expected_title.to_string(),
			},
		)
		.await;

		 let title = get_post_title(&client, id).await?;

		 assert_eq!(title, Some(expected_title));
	}
}
```
