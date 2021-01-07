#!/bin/sh

mode=${1:+"--release"}
flags=${1:+"-Copt-level=s"}
lto=${1:+"true"}
out_dir=${1:-debug}

RUSTFLAGS=$flags cargo build -p example_widgets --bin example_widgets --target wasm32-unknown-unknown $mode
wasm-bindgen --target web --no-typescript --out-dir examples/static/target --out-name example_widgets target/wasm32-unknown-unknown/${out_dir}/example_widgets.wasm