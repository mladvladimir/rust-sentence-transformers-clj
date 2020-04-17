#!/usr/bin/env bash

set -eo pipefail

# build the rust library

cd rs
cargo build --release
cp target/release/librust_sentence_transformers_clj.so ../clj/rust-sentence-transformers-clj/resources/
cd ..