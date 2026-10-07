# Core-only consumer

This library uses `#![no_std]` and `alloc` to merge caller-fetched provider
results and select providers from the shared catalog. It needs an allocator
provided by its final application. No network transport is included.

```sh
rustup target add wasm32-unknown-unknown thumbv7em-none-eabi
cargo check --manifest-path rust/examples/no-std-consumer/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path rust/examples/no-std-consumer/Cargo.toml --target thumbv7em-none-eabi
```

The second target has no standard library, so it verifies the entire dependency
graph rather than just using the WASM target's available standard library.
