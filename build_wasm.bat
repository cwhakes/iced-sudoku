cargo build --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/sudoku.wasm --out-dir wasm --web
httplz wasm
