# Composite Types

_Available since v0.6.7_

When using MongoDB you will likely need to use [`embedded documents`](https://www.mongodb.com/docs/manual/core/data-model-design/#std-label-data-modeling-embedding),
which Prisma calls 'Composite Types'.
Prisma Client Rust will generate field & type modules whenever you use composite types,
allowing you to perform CRUD operations on them.

These docs will only focus on Rust-specific details,
checkout [Prisma's documentation](https://www.prisma.io/docs/concepts/components/prisma-client/composite-types#changing-a-single-composite-type)
for a comprehensive guide including a list of all available operations, filters,
and some caveats to keep in mind.

All examples will use the following schema:

```prisma
model Product {
  id     String  @id @default(auto()) @map("_id") @db.ObjectId
  name   String  @unique
  price  Float
  colors Color[]
  sizes  Size[]
  photos Photo[]
  orders Order[]
}

model Order {
  id              String   @id @default(auto()) @map("_id") @db.ObjectId
  product         Product  @relation(fields: [productId], references: [id])
  color           Color
  size            Size
  shippingAddress Address
  billingAddress  Address?
  productId       String   @db.ObjectId
}

enum Color {
  Red
  Green
  Blue
}

enum Size {
  Small
  Medium
  Large
  XLarge
}

type Photo {
  height Int    @default(200)
  width  Int    @default(100)
  url    String
}

type Address {
  street String
  city   String
  zip    String
}
```

## Filtering

To find records with matching composite types,
use the field's filter functions in combination with the type's field modules' `equals` functions.

```rust
let orders = client
	.order()
	.find_many(vec![
		order::shipping_adress::is(vec![
			address::street::equals("555 Candy Cane Lane".to_string()),
			address::city::equals("Wonderland".to_string()),
			address::street::equals("52337".to_string()),
		])
	])
	.exec()
	.await?;
```

## Create

To create a new composite type, use its `create` type module function.

```rust
let order = client
	.order()
	.create(
		..,
		vec![
			order::shipping_adress::set(
				address::create(
					"1084 Candycane Lane".to_string(),
					"Silverlake".to_string(),
					"84323".to_string(),
					vec![]
				)
			),
			order::billing_address::set(None),
			order::photos::set(vec![
				photo::create(100, 200, "photo.jpg".to_string());
				3
			])
		]
	)
```

## Update

To update an existing composite type, there are a few type module functions available.

### Single Fields

```rust
// overwrite entire type
order::shipping_address::set(address::create(
	..
))

// update certain fields
order::shipping_address::update(vec![
	address::zip::set("41232".to_string())
])

// attempt to update certain fields,
// creating a new type if one doesn't exist
order::billing_address::upsert(
	address::create(..),
	// list of updates to attempt to apply
	vec![address::zip::set("41232".to_string())]
)

// removes the field entirely
order::billing_address::unset()
```

### List Fields

```rust
// overwrite entire list
product::photos::set(vec![
	photo::create(..),
	photo::create(..),
])

// push values to the end of the list
product::photos::push(vec![
	photo::create(..),
	photo::create(..),
])

// update multiple values in the list
product::photos::update_many(
	// filter
	vec![photo::url::equals("1.jpg".to_string())],
	// updates
	vec![photo::url::set("2.jpg".to_string())]
)

// update multiple values from the list
product::photos::delete_many(
	// filter
	vec![photo::url::equals("1.jpg".to_string())],
)
```

## Ordering

It is possible to sort results based on the order of fields in composite types.

```rust
use prisma_client_rust::Direction;

let orders = client
	.order()
	.find_many(vec![])
	.order_by(order::shipping_address::order(
		address::city::order(SortOrder::Asc)
	))
	.exec()
	.await?;
```
