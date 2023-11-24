# Todos API

This is a simple Todos API made in Rust using Axum. This is a learning project with a blog post explaining this: [Creating APIs with Rust and Axum](https://ryan17.dev/blog/apis-with-rust-and-axum).

## Commands

Just run `cargo run` in the terminal or use [cargo-watch](https://crates.io/crates/cargo-watch) to watch for changes. It will watch for changes in the `src` folder where our Rust code is updated.

To install `cargo-watch`:

```sh
cargo install cargo-watch
```

To run the server in watch mode:

```sh
cargo watch -c -w src -x run
```
