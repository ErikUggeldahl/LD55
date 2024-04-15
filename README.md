# LD55

A game written in Rust for the [WASM-4](https://wasm4.org) fantasy console for Ludum Dare 55.

## Building

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

Or package with

```shell
./package.sh
```
