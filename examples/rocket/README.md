# Rocket Example

This is an example of using [Prisma Client Rust](https://github.com/Brendonovich/prisma-client-rust) with [Rocket](https://rocket.rs/).

## Running

Generate the client:

```bash
cargo prisma generate
```

Then run the server:

```bash
cargo run
```

Server is configured using `Rocket.toml` or environment variables.
By default, this server is configured for [http://localhost:8080](http://localhost:8080).
Read about Rocket configuration [here](https://rocket.rs/v0.5-rc/guide/configuration/).

Written by [Aaron Leopold](https://github.com/aaronleopold)

## Examples

[http://localhost:8080/api/users](http://localhost:8080/api/users)

```json
[
  {
    "id": "5ab80953-c38c-4ec8-8b4b-3ecc4bc1196f",
    "displayName": "oromei",
    "posts": [
      {
        "id": "f001144f-438f-4fdb-9ed7-23f2cc5fffa7",
        "content": "Woah there!",
        "user": null,
        "userId": "5ab80953-c38c-4ec8-8b4b-3ecc4bc1196f"
      }
    ]
  }
]
```

[http://localhost:8080/api/users?load_posts=false](http://localhost:8080/api/users?load_posts=false)

```json
[
  {
    "id": "5ab80953-c38c-4ec8-8b4b-3ecc4bc1196f",
    "displayName": "oromei",
    "posts": null
  }
]
```

[http://localhost:8080/api/posts](http://localhost:8080/api/posts)

```json
[
  {
    "id": "f001144f-438f-4fdb-9ed7-23f2cc5fffa7",
    "content": "Woah there!",
    "user": null,
    "userId": "5ab80953-c38c-4ec8-8b4b-3ecc4bc1196f"
  }
]
```
