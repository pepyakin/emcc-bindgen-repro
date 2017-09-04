# emcc-bindgen-repro
Repro for https://github.com/rust-lang-nursery/rust-bindgen/issues/947

## Building

Before building you should have emsdk in your env. See [this guide](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde).

```shell
cargo build --target=wasm32-unknown-emscripten
```

You should see [this error](https://gist.github.com/pepyakin/ca869efb1d3dc3850affb36de1ca131c).

## Workaround

[Setting `.trust_clang_mangling(false)`](https://github.com/pepyakin/emcc-bindgen-repro/blob/master/build.rs#L11) worked for me. 
