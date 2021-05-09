### cbindgen demo project

If you're new to Rust FFI, or if you're using [cbindgen](https://github.com/eqrion/cbindgen) for the first time, it may take a few attempts to get things right.

If you're lazy like me, you might want to start with a sample project, already set up for `no_std`, `staticlib`, `panic="abort"`, and a working `build.rs`.

This version does not include the Rust `std` library. If you want a library that does support `std`, check out [cbindgen-std-demo](https://github.com/ericseppanen/cbindgen-std-demo).

This is a simple Rust library, with a few constants and data structures that can be used from C code. It also includes a C program that calls some of that library's functions.

How to build it:
```text
> cargo build --release
> gcc -O2 -Wl,--gc-sections main.c target/release/libdemo.a -o demo
> strip demo
> ./demo
```
