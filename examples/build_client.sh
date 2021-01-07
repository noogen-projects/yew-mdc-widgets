#!/bin/sh

mode=${1:+"--release"}
flags=${1:+"-Copt-level=s"}
lto=${1:+"true"}
out_dir=${1:-debug}

RUSTFLAGS=$flags cargo build -p example_client --target wasm32-unknown-unknown $mode
wasm-bindgen --target web --no-typescript --out-dir static/target --out-name example_client target/wasm32-unknown-unknown/${out_dir}/example_client.wasm