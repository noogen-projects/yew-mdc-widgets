# Yew MDC widgets

The [Material Design Components](https://material.io/develop/web) widgets for the [Yew](https://github.com/yewstack/yew).
[Live demo](https://noogen-projects.github.io/yew-mdc-widgets/)

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
cargo make run
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
cargo make checkfmt
cargo make fmt
```

To run clippy, use the following command:

```shell script
cargo make clippy
```
