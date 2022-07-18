# REST Axum Example

This is an example of how you could use [Prisma Client Rust](https://github.com/Brendonovich/prisma-client-rust) in a REST API, written by [kr4xkan](https://github.com/kr4xkan). 

## Running

First generate the Prisma client:

```
$ cargo prisma generate
```

Setup database:

```
$ cargo prisma db push
```

Then run the server:

```
$ cargo run
```

## Notes

In addition to showing you how to use this crate in a REST API backend, it also gives you a way too catch errors from Prisma in one single place.

## Endpoints

Base URL: `localhost:5000/api`

`/user` :
- `GET` : Lists all users
- `POST` : Create a user
  - ```json
    INPUT
    {
        "username": string
        "email": string
    }
`/user/<username>` :
- `PUT` : Update a user
  - ```json
    INPUT
    {
        "username": string
        "email": string
    }
- `DELETE` : Delete a user

`/comment` :
- `POST` : Create a comment linked to a user
  - ```json
    INPUT
    {
        "user": int
        "message": string
    }