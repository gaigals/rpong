#!/bin/bash

ORIGINAL_DIR=$(pwd)

# Make sure that it does not matter where you call this script.
PROJECT_DIR="$(dirname "$(readlink -f "$0")")"

cd $PROJECT_DIR

cargo build --target wasm32-unknown-unknown
if [ $? -ne 0 ]; then
    cd $ORIGINAL_DIR
    exit
fi

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
cd $ORIGINAL_DIR
sudo $PROJECT_DIR/rpong