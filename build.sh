#!/bin/bash
# Exit script if any command returns non-zero
set -e
set -o pipefail
wasm-pack build --target no-modules
# Add glue code to make loader generated by wasm_bindgen compatible with the previous one that was
# generated by cargo-web
cat << EOF >> pkg/wasm_websocket_upload_big_file.js

let Rust = {};
Rust.wasm_upload_file = wasm_bindgen(
    "./wasm_websocket_upload_big_file_bg.wasm"
).then(
    function(unused) {
        return wasm_bindgen;
    },
    function(err) {
        console.error(err);
    }
);

EOF