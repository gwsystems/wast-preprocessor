# Background


WebAssembly is a virtual instruction set architecture that was designed to provide an "assembly language for the web." WebAssembly has a text format designed to be readable by humans and a binary format designed to be efficiently downloaded over a network. Different runtimes interpret this virtual assembly language and generate native machine code that can be executed on native processors such an Intel or ARM. In order to guarantee that WebAssembly code can run on different machines correctly, the WebAssembly specification body has created an extension to the WebAssembly Text format `*.wat` called `*.wast`. This provides a set of assertions used to test standards compliance.

  

Typically, WebAssembly is interpreted or just-in-time compiled. However, or aWsm compiler takes a different approach and uses ahead-of-time compilation. This makes it challenging for us to run `*.wast` tests. When debugging the compiler, the tests were previously manually ported by hand. In general, the declarative assertions of the `*.wast` format was replaced with imperative control flow written by hand in `*.wat`. More information can be find in the README at https://github.com/gwsystems/aWsm/tree/master/tests/wat. In the future, the WebAssembly test suite is likely to expand and evolve, there needs to be a better way to port tests.

  
  

# Goal

  

Write a program in Rust to automate transforming the `*.wast` declarative assertions into a ...

  
  

# File Documentation

* src/main.rs


`src/main.rs` takes the `.*wast` file as input and creates `*.c` and `*.wasm` files of the same name. The `*.c` file contains the assert declarations. The `*.wasm` file contains the functions from the WebAssembly module. These declarations will run the functions that are within the `*.wasm` file in order to test the assertions. A `*.c` and `*.wasm` file pair is made per each module within the `*.wast` file.

* src/add_test.wast

`src/add_test.wast` is a simple WebAssembly programming that executes addition. It is used as a sample program to test the `src/main.rs` program.

  

* add_test_0.c


`add_test_0.c` is the resulting `*.c` file from running `cargo run src/add_test.wast`.

  

* add_test_0.wasm


`add_test_0.wasm` is the resulting `*.wasm` file from running `cargo run src/add_test.wast`.

  

# Installation Steps
* **Installing Rust**: [doc.rust-lang.org](https://doc.rust-lang.org/book/ch01-01-installation.html)

# Running the Program

  

Use the command: `cargo run [ path ]`, where the [ path ] is the relative path of the target `*.wast` file.
