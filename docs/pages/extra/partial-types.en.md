# Partial Types

_Available since v0.6.7_

The `partial_unchecked!` macro can be found in all model modules,
and allows structs to be defined that have a `to_params` function which converts them for use inside `update_unchecked`.
Each field of the generated structs has the same type as the equivalent field in the module's `Data` struct,
just wrapped inside `Option`.

This can be useful for thing like web APIs built with
[`axum`](https://github.com/tokio-rs/axum) or
[`rspc`](https://www.rspc.dev/),
where receiving updates is more ergonomic as structs rather than a list of changes.

A more general `partial!` does not yet exist,
as supporting relations is not possible until [nested writes](https://github.com/Brendonovich/prisma-client-rust/issues/44)
are supported.

## Setup

Using partial macros requires the same setup as [Select & Include](/reading-data/select-include#setup),
as `module_path` must be provided.

## Example

Given the following schema:

```prisma
model Post {
	id Int @id @default(autoincrement())
	title String
	content String
}
```

An updater function can be written like so:

```rust
post::partial_unchecked!(PostUpdateData {
	title
	content
})

pub async fn update_post(
	db: &PrismaClient,
	id: i32,
	data: PostUpdateData
) {
	db.post()
		.update_unchecked(post::id::equals(id), data.to_params())
		.exec()
		.await;
}
```

The above use of `partial_unchecked!` generates something like the following:

```rust
pub struct PostUpdateData {
	title: Option<String>,
	content: Option<String>
}

impl PostUpdateData {
	pub fn to_params(self) -> Vec<post::UncheckedSetParam> {
		[
			self.title.map(post::title::set),
			self.content.map(post::content::set)
		].into_iter().flatten().collect()
	}
}
```
