### cbindgen demo project

If you're new to Rust FFI, or if you're using [cbindgen](https://github.com/eqrion/cbindgen) for the first time, it may take a few attempts to get things right.

If you're lazy like me, you might want to start with a sample project, already set up for `staticlib`, `panic="abort"`, and a working `build.rs`.

This is a simple Rust library, with a few constants and data structures that can be used from C code. It also includes a C program that calls some of that library's functions.

This version includes Rust `std` library support. If you want a `no_std` library, check out [cbindgen-demo](https://github.com/ericseppanen/cbindgen-demo). There is also a version that adds `alloc` support without the full `std` library: [cbindgen-alloc-demo](https://github.com/ericseppanen/cbindgen-alloc-demo).

How to build it:
```text
> cargo build --release
> gcc -O2 -Wl,--gc-sections -pthread main.c target/release/libdemo.a -ldl -o demo
> strip demo
> ./demo
```
