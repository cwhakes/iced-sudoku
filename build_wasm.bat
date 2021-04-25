cargo build --release --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/release/sudoku.wasm --out-dir wasm --web
httplz wasm
