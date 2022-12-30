
## Building for wasm

### Check:
```
rustup target add wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown
```

### Build and Release
```
wasm-pack build --release --scope aryansw
wasm-pack publish pkg --access public
```
