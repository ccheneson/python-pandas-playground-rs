#!/usr/bin/env bash


rm -r app > /dev/null 2>&1
mkdir app
npm install && npm run build
cargo clean && cargo build --release
cp -pr ./dist app/
cp target/release/python-pandas-playground ./app/


