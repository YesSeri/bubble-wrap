# Bubblewrap

A game written in Rust for the [WASM-4](https://wasm4.org) fantasy console. Music was made with [RustMusic4Wasm4](https://github.com/YesSeri/RustMusic4Wasm4)

## Smaller size tips

### cargo.toml

```toml
[profile.release]
opt-level = "z"
lto = true
strip = true
```

### vector

`wasm-opt -Oz --zero-filled-memory --strip-producers --dce -o target/wasm-snip-output.wasm target/cart.wasm`

Use heapless vec. I compared [arrayvec](https://github.com/bluss/arrayvec) and [heapless](https://github.com/japaric/heapless) and for my binary heapless was smaller.

cargo.toml

```toml
[dependencies]
heapless = "0.7.15"
```

### Strings and formatting

Remove all string formatting.

I use this for debugging: `trace(format!("{:?}", (&self)));`. Make sure all of these are removed when building. Otherwise the binary size becomes HUGE!

### panic!

Don't panic. Dont use `unwrap()`, or other statements that panic. Use `unwrap_or(x)` or `unwrap_or_else(|| x*2)` or `unwrap_unchecked()` Even if you use wasm-snip the binary is still bigger than if you just didn't use `unwrap()` at all.

## Idea

Bubble shooter. A little figure is running trying to shot bubbles. The bubbles bounce on the floor. The floor is wrapping, and it is two levels. 

```
  ┌────────────────────────────────┐
  │   ┌────────────────────────┐   │
  │   │                        │   │
  │   │                        │   │
┌─┼───►                       ◄├───┘
│ │   │                        │
│ │   │                        │
│ │   ├────────────────────────┤
│ │   │                        │
│ │   │                        │
│ │   │                        │
│ └───►                       ◄├────────┐
│     │                        │        │
│     └────────────────────────┘        │
└───────────────────────────────────────┘
```

## Building

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

## Links

- [Documentation](https://wasm4.org/docs): Learn more about WASM-4.
- [Snake Tutorial](https://wasm4.org/docs/tutorials/snake/goal): Learn how to build a complete game
  with a step-by-step tutorial.
- [GitHub](https://github.com/aduros/wasm4): Submit an issue or PR. Contributions are welcome!
