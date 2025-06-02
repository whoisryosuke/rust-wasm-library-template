# Rust WASM Library Template

Write Rust code and compile to WASM for use on the web.

Based on [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template/tree/master) and [this guide.](rustwasm.github.io/docs/book/game-of-life/hello-world.html#build-the-project)

## Requirements

- Rust
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

## Development

Write code (or import it into) in `lib.rs`.

## Building

Build project using `wasm-pack build`.

This generates a WASM build, and nice JS + TS glue files, inside the `/pkg` folder.

## Using code

1. Build project (see above)
1. Go into build folder: `cd pkg`
1. Create a link to the module: `yarn link`
1. Go into another JS project and link the module: `yarn link your-wasm-library-name`
