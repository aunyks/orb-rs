# orb-rs

Web server boilerplate in Rust. Use it when speed or size is _paramount_.

Looking to get started? Find and replace the string `orb` with your new module name and you'll be in your way. All `cargo` commands should just work by default.

## Unit Tests

To run unit tests:

```
cargo test
```

## Local Development

To build and run a development version of the server:

```
cargo run
```

## Build for Release

To build a release binary:

```
cargo build --release
```

Note that you can set the optimization levels for release builds with the `opt-level` value in `Cargo.toml`. More details can be found in [the Cargo reference](# https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level).
