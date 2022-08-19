# Simple Example

This is an example of using Prisma Client Rust with a simple Prisma schema. It creates some records, fetches them with some relations, and then deletes them. 

## Running

First generate the Prisma client:

```
$ cargo prisma generate
```

Push schema to sqlite database

```bash
$ cargo prisma db push
```

Then run the server:

```
$ cargo run
```