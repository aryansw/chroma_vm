## Running for Windows

```
cargo run --target x86_64-pc-windows-msvc -- -p 'program.jpg' -i 'input.png'
```

## Building for wasm

I'll eventually get around to making a CI build script for this. But these are the general steps:

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
