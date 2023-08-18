# Query Traits

_Available since 0.6.4_

As your app grows, you may find yourself writing repetitive queries or conditions.
A common way to reduce this repetition is by putting queries inside functions to use elsewhere.
This is a bit limiting, though, as it requires a new function per query type,
and isn't very extensible.

To aid with this,
PCR's query builders implement a set of traits that allow modification in a way that is typesafe,
but also decoupled from the specific query you're creating.

## Base Traits

### `Query`

This probably won't be very useful,
it is the core trait implemented by every query builder.

### `ModelQuery`

This is implemented by all query builders that operate on a specific model,
ie. every query except raw queries.

It is used to hold types and data corresponding to each model,
and is implemented for each model module's `Types` struct.

## Specific Traits

Each of these traits expose functions for adding additional parameters.

### `WhereQuery`

- `add_where`: adds one `WhereParam`

Implemented for `Count`, `FindMany`, `FindFirst`, `UpdateMany`, and `DeleteMany`

### `WithQuery`

- `add_with`: adds one `WithParam`

Implemented for `FindUnique`, `FindMany`, `FindFirst`, `Create`, `Update`, `Upsert`, and `Delete`

### `OrderByQuery`

- `add_with`: adds one `OrderByParam`

Implemented for `FindMany`, `FindFirst`, and `Count`

### `PaginatedQuery`

- `add_cursor`: adds one `UniqueWhereParam` as a cursor
- `set_skip`: sets the number of records to skip
- `set_take`: sets the number of records to take

Implemented for `Count`, `FindFirst` and `FindMany`.

### `SetQuery`

- `add_set`: adds one `SetParam`

Implemented for `Create`, `Update`, and `Upsert`
