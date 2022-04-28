# yew-mdc-widgets

The widgets for the [Yew](https://github.com/yewstack/yew) of the [Material Design Components](https://material.io/develop/web).


## Run example

Setup dependencies:

```shell script
cargo install wasm-bindgen-cli cargo-make
```

Build example wasm client:

```shell script
cargo make example_client
```

Run example server:

```shell script
cargo run -p example_server --release
```


## Development notes

To check the project, use the following command:

```shell script
cargo check --workspace --all-features --all-targets
```

To run all tests, use the following command:

```shell script
cargo test --workspace --all-features --all-targets
```

To check and perform formatting, use the following commands:

```shell script
cargo +nightly fmt -- --check
cargo +nightly fmt
```

To enable autoformatting for IntelliJ IDEA with the Rust plugin:

`File -> Settings -> Languages & Frameworks -> Rust -> Rustfmt, check "Run rustfmt on Save"`

To run clippy, use the following command:

```shell script
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

To setup git hook, use the following command:

```shell script
cp .git-pre-push.sh .git/hooks/pre-push
```