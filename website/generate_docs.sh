#!/bin/bash

cd ../ &&
cargo clean &&
cargo doc --features="freetype" &&
rm -rf ./website/out/doc/ &&
mv ./target/doc ./website/out/doc
