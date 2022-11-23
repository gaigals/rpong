// // Load WASM ...
WebAssembly.instantiateStreaming(fetch("/static/wasm/rpong.wasm"))
.then(wasmModule => {
    // this saves the exported function from WASM module for use in JS
    wasm_add = wasmModule.instance.exports.add;
    // default = wasmModule.instance.exports.default;
});

