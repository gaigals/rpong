#!/bin/bash

PROJECT_DIR=$(pwd)

cargo build --target wasm32-unknown-unknown

cd target/wasm32-unknown-unknown/debug/
wasm-bindgen --target web --no-typescript --out-dir . rpong.wasm
mv rpong_bg.wasm rpong.wasm
wasm-gc rpong.wasm

# MacOS workaround (sed - 's/old/new/g' is not working correctly).
sed -E -i ".js" "s/rpong_bg\.wasm/\..\/wasm\/rpong\.wasm/g" rpong.js
rm -f rpong.js.js

cp rpong.wasm $PROJECT_DIR/server/www/static/wasm/
cp rpong.js $PROJECT_DIR/server/www/static/js/

cd $PROJECT_DIR/server
go build -o $PROJECT_DIR/rpong
cd $PROJECT_DIR
sudo ./rpong