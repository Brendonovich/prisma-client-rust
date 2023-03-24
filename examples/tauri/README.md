# Tauri Example

This example demonstrates how to combine Prisma Client Rust and Tauri,
with [`tauri-specta`](https://github.com/oscartbeaumont/tauri-specta) being used to provide end-to-end typesafety.

[SolidJS](https://www.solidjs.com/) is being used to render the UI,
and [`pnpm`](https://pnpm.io/) is being used as a package manager (but using npm/yarn should work too).

## Runing

Generate the client:

```bash
cd src-tauri && cargo prisma generate
```

Then run the app:

```bash
pnpm tauri dev
```
