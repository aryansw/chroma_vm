## VM Design

### Hex Code representation

Each pixel is meant to represent an instruction here. For the 24-bits in an 8-bit Hex:

- The first 6 bits represent the opcode. Depending on the op-code, the next 18 bits can be used for some of the following purposes:
  - 6 bits can be used to represent a single register. Within these 6 bits:
    - The first bit represents whether the register's value or a dereference of the register's value
    - The next 5 bits represent the register number. We hence have 32 registers
  - 12 bits can be used to reference a label
  - 6/12 bits can be used to store a small immediate value
- Full values can be stored using a complete Hex value (24-bits)
- Full addresses can be stored using a complete Hex value too (12-bits, 12-bits), or hence referencing locations until (4096, 4096).
- As slightly discussed above, registers can be used to store values or addresses. If we choose to use the register as an address, we can use the first bit to determine whether we want to dereference the address or not. Alternatively, we can use the register just to hold a normal value.

## VM

- The program counter is a 24-bit value, representing the current instruction/pixel being executed.
- The program counter moves left to right, and then down to the next row.
- Registers are 24-bit values, and can be used to store values or addresses. Initially, the registers are all set to 0.
- We have 32 registers. There are some special purpose registers though:
  - `r32` is the program counter
  - `r31` is the stack pointer
  - `r29` and `r30` are registers to actually store references to input and output images.
    - The input image is read-only, and the output image allows read/write operations.
    - The first hex value that is read from either of these images represents the image's dimensions, (12-bits, 12-bits).
- The VM has a special instruction to allocate memory, which places new white pixels at the end of the image.

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
