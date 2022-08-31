---
title: Structure
desc: Basic syntax explanation
layout: ../../layouts/MainLayout.astro
---

## Syntax

The generated client uses a functional API syntax that follows Rust's naming conventions.

A instance of `PrismaClient` can be created by calling `new_client`:

```rust
// Don't actually unwrap, handle errors properly!
let client = prisma::new_client().await.unwrap();
```

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

## Client

The generated client is one file containing many modules, types and function.
At the root level there is:

- `new_client()` & `new_client_with_url()`: Functions for creating an instance of the client

- `_prisma` module: Contains the `PrismaClient` struct and various internal enums

- Model modules: One for each model in your Prisma schema

## Model Modules

These modules contain types & functions for queries related to their respective model and its fields.
Their name is their respective model's name converted to `snake_case`.

### Functions

#### `_outputs`

Used by the client to get a list of all the model's scalar fields.

#### Compound Field Filters

Generated for each compound unique index (`@@id([])` and `@@unique([])` in a Prisma schema) on the model.
Compound unique indexes don't get their own [field modules](#field-module) since the only thing they would contain is an `equals` filter,
so a single function with the name of all fields in the compound index combined is generated as a replacement.

### Enums

#### WithParam

Contains all relations that can be fetched for a model,
with query builders for nested querying.

#### SetParam 

Contains all possible modifications to the model and their associated data.

#### OrderBy

Contains a variant with a `prisma_client_rust::Direction` for each field that can be used for ordering.

#### WhereParam

Contains all possible filters for the model and their associated data.

#### UniqueWhereParam

Contains a subsection of `WhereParam` that can be used to uniquely identify individual records.
Usually only contains `Equals` variants.

### Structs

#### Actions

The struct returned by the client's function for making queries on the model.
Implements functions that return the query builders defined in the `prisma_client_rust` crate.

### Modules

For each of the model's fields, [a module](#field-modules) is generated that contains types and functions for operating on specific fields.

### Macros

Each module gets a macro generated for doing [field selection](/reading-data/select#the-macro).

## Field Modules

### Structs

#### Fetch

Generated for relation fields.
Wraps a query builder for a relation's model in a unique struct,
which can then be converted to a `WithParam`.
This wrapping is necessary as some models may contain multiple relations to the same model,
which would result in multiple `WithParam` conversions being implemented on the same query builder.

#### Other

Some operations such as `connect` and `set` have individual structs generated for them.
They simply wrap the operations' arguments in a unique struct for conversion to an appropriate enum.

### Functions

#### Equals

`equals` is a special function generated for scalar fields that can return multiple types depending on where it is used. Its implementation depends on the type of the scalar field:

- Required Field: Takes a value with the same type as the field and returns a `WhereParam`

- Required Unique Field: Takes a value with the same type as the field and returns a generic type,
allowing either `UniqueWhereParam` or `WhereParam` to be returned depending on where it is used.

- Optional Unique Field: If being used somewhere expecting a `UniqueWhereParam`, takes a value matching the type of the field.
If being used somewhere expecting a `WhereParam`, however, takes an `Option` of the type of the field.
This is necessary as `find_unique` does not accept SQL `NULL` as a filter, but all other find methods do.

#### Other Filters & Operations

Functions are generated for type-specific filters and some other operations.
