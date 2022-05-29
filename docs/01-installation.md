# Setup

Prisma Client Rust's installation operates in a different way to most Rust projects.

`prisma-client-rust` is used by the generated client and possibly own code, as it provides access to Prisma internals and helper functions.

`prisma-client-rust-cli` contains the code generation and access to the Prisma CLI, but does not provide an executable binary - this must be created yourself. <sup>[why?](#why-is-a-cli-binary-not-provided)</sup>

## Creating a CLI Binary Inside Your Project

First, the main library and CLI package must be added to your project's Cargo.toml:

```toml
[dependencies]
prisma-client-rust = { git = "https://github.com/Brendonovich/prisma-client-rust", tag = "0.5.0" }
prisma-client-rust-cli = { git = "https://github.com/Brendonovich/prisma-client-rust", tag = "0.5.0" }
```

The easiest way to create a binary to access the CLI through is by creating a `src/bin` folder if you don't already have one, and inside it creating a file called something like `prisma.rs` (This will determine the name of your binary). Inside this file insert the following:

```rust
fn main() {
    prisma_client_rust_cli::run();
}
```

Technically, this is all that is required! Just run the following to access the CLI:

```bash
$ cargo run --bin <your binary name> -- <command>
```

This isn't a very friendly command to run, though. Luckily Cargo allows us to define project-wide aliases for running commands! Create a folder at the root of your project called `.cargo` and inside it create a file `config.toml` with the following contents:

```toml
[alias]
prisma = "run --bin <your binary name> --"
```

Now you can run `cargo prisma <command>` anywhere in your project to access the CLI!

## Creating a CLI Binary as a Seprate Package

This approach has a problem, though: `prisma-client-rust-cli` is included as a dependency in your library/application, which is likely not desirable. The solution to this is to move the CLI binary to a separate package and configure your project to use [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). Below is a sample project structure that has one binary target in `src/main.rs`, and a separate package for the CLI named `prisma-cli`, which is included in the [workspace members](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html#:~:text=%5Bworkspace%5D-,members%20%3D%20%5B,-%22adder%22%2C%0A%5D) of `Cargo.toml`.

```
Cargo.toml
.cargo/
    config.toml
src/
    main.rs
prisma-cli/
    Cargo.toml
    src/
        main.rs
```

For the above example, `Cargo.toml` would include `prisma-client-rust` as a dependency as it is required by the generated file, whereas `prisma-cli/Cargo.toml` would include `prisma-client-rust-cli` as a dependency, and so the binary in `src/main.rs` would not be bundled with all the CLI code, only the required library code.

## Why is a CLI Binary not Provided?

In older versions of Prisma Client Rust, it was possible to `cargo install prisma-client-rust-cli` and have a global install of the CLI available to use at any time. This had a major problem though: Versioning. Managing multiple projects that used different versions of Prisma Client Rust got very annoying very quickly, plus it went against the recommmended installation instructions of Prisma Client [JS](https://www.prisma.io/docs/getting-started/setup-prisma/add-to-existing-project/relational-databases-typescript-postgres), [Go](https://github.com/prisma/prisma-client-go/blob/main/docs/quickstart.md), and [Python](https://prisma-client-py.readthedocs.io/en/stable/#installing-prisma-client-python).

Unlike these three languages, Rust (or more specifically Cargo) does not provide a method for executing binaries available inside dependencies. Since installing a globally available binary was ruled out, providing the CLI as a library was seen as the only other option, plus personally I think that being able to run `cargo prisma <command>` is quite a nice experience and matches with clients in other languages.

## Up Next

Next, learn how to [setup your schema](02-setup.md) to generate the client.
