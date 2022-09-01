# Simple Example

This is an example of using Prisma Client Rust with a simple Prisma schema. It creates some records, fetches them with some relations, and then deletes them. 

## Running

First generate the Prisma client:

```
$ cargo prisma generate
```

Then push the database migrations:

```bash
$ cargo prisma db push
```

Then run the server:

```
$ cargo run
```
