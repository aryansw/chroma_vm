# Chroma VM

A simple VM that runs on images.

## VM Design

### Instructions

- Here's a basic set of instructions:

### Basic Moves, Arithmetic, and Logic

Something to rethink: Maybe it might be better to allow jumps to not require a register to store the address, by making it a "two-hex" instruction, the second hex being the address to jump to.

| Instruction | First | Second    | Third | Description                                                 |
| ----------- | ----- | --------- | ----- | ----------------------------------------------------------- |
| `LOADL`     | r1    | immediate |       | Move a 12-bit value into a register into its lower bits     |
| `LOADH`     | r1    | immediate |       | Move a 12-bit value into its higher bits                    |
| `MOVE`      | r1    | r2        |       | Copy value from one register into another register          |
| `ADD`       | r1    | r2        | r3    | Add two registers together (r1 = r2 + r3)                   |
| `SUB`       | r1    | r2        | r3    | Subtract two registers (r1 = r2 - r3)                       |
| `MUL`       | r1    | r2        | r3    | Multiply two registers (r1 = r2 \* r3)                      |
| `DIV`       | r1    | r2        | r3    | Divide two registers (r1 = r2 / r3)                         |
| `MOD`       | r1    | r2        | r3    | Modulo two registers (r1 = r2 % r3)                         |
| `AND`       | r1    | r2        | r3    | Bitwise AND two registers (r1 = r2 & r3)                    |
| `OR`        | r1    | r2        | r3    | Bitwise OR two registers (r1 = r2 \| r3)                    |
| `CEQ`       | r1    | r2        | r3    | Compare two registers (r1 = r2 == r3), 1 if true 0 if false |
| `CNE`       | r1    | r2        | r3    | Compare two registers (r1 = r2 != r3)                       |
| `CGT`       | r1    | r2        | r3    | Compare two registers (r1 = r2 > r3)                        |
| `CLT`       | r1    | r2        | r3    | Compare two registers (r1 = r2 < r3)                        |
| `CGE`       | r1    | r2        | r3    | Compare two registers (r1 = r2 >= r3)                       |
| `CLE`       | r1    | r2        | r3    | Compare two registers (r1 = r2 <= r3)                       |
| `NOP`       |       |           |       | No operation                                                |
| 'HALT'      |       |           |       | Halt the program                                            |

### Jumps and Function Calls

| Instruction | First | Second | Third | Description                              |
| ----------- | ----- | ------ | ----- | ---------------------------------------- |
| `JMP`       | r1    |        |       | Jump to register r1's address            |
| `JMPIF`     | r1    | r2     |       | Jump to register r1's address if r2 != 0 |
| `PUSH`      | r1    |        |       | Push a value onto the stack (at r30)     |
| `POP`       | r1    |        |       | Pop a value from the stack (at r30)      |
| `CALL`      | r1    |        |       | Call a function at register r1           |
| `RET`       |       |        |       | Return from a function                   |

### IO and Memory

As discussed below, the same registers can be used to store values or addresses. We can use the first bit to determine whether we want to dereference the address or not. This makes memory access a bit more straightforward.

| Instruction | First | Second | Third | Description                                                                                 |
| ----------- | ----- | ------ | ----- | ------------------------------------------------------------------------------------------- |
| `ALLOC`     | r1    | r2     |       | Allocate memory at the end of the image, of size r2 and store address at r1                 |
| `CURR`      | r1    |        |       | Store the current address in register r1                                                    |
| `MCPY`      | r1    | r2     | r3    | Copy r3 bytes from address r1 to address r2 (can be used with registers r28 and r29) for IO |

### Hex Code representation

Each pixel is meant to represent an instruction here. For the 24-bits in an 8-bit Hex:

- The first 6 bits represent the opcode. Depending on the op-code, the next 18 bits can be used for some of the following purposes:
  - 6 bits can be used to represent a single register. Within these 6 bits:
    - The first bit represents whether we wish to change the register's value or dereference the register's value.
    - In other words, this determines if r1 = 10 vs \*r1 = 10.
    - The next 5 bits represent the register number. We hence have 32 registers (0 to 31).
  - 12 bits can be used to reference a label
  - 6/12 bits can be used to store a small immediate value
- Full values can be stored using a complete Hex value (24-bits)
- Full addresses can be stored using a complete Hex value too (12-bits, 12-bits), or hence referencing locations until (4096, 4096).
- All values are treated as unsigned here
- As slightly discussed above, registers can be used to store values or addresses. If we choose to use the register as an address, we can use the first bit to determine whether we want to dereference the address or not. Alternatively, we can use the register just to hold a normal value.

### VM

- The program counter is a 24-bit value, representing the current instruction/pixel being executed.
- The program counter moves left to right, and then down to the next row.
- Registers are 24-bit values, and can be used to store values or addresses. Initially, the registers are all set to 0.
- We have 32 registers. There are some special purpose registers though:
  - `r31` is the program counter
  - `r30` is the stack pointer
  - `r28` and `r29` are registers to actually store references to input and output images.
    - The input image is read-only, and the output image allows read/write operations.
    - The first hex value that is read from either of these images represents the image's dimensions, (12-bits, 12-bits).
- The VM has a special instruction to allocate memory, which places new white pixels at the end of the image.
  - `r30`'s stack pointer is simply an address to some heap space allocated in the end of the image. As we push and pop values, we move the stack pointer up and down.

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
