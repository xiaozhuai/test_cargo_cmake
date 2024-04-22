# test_cargo_cmake

This is a demo project to show how to use cargo with cmake.

[![ci](https://github.com/xiaozhuai/test_cargo_cmake/actions/workflows/ci.yml/badge.svg)](https://github.com/xiaozhuai/test_cargo_cmake/actions/workflows/ci.yml)

## How To

1. Add `cmake` as build dependencies.
    ```shell
    cargo add cmake --build
    ```
2. Put your library source code in `libtest` which contains `CMakeLists.txt`. Be sure to add a `install` target.
    ```cmake
    cmake_minimum_required(VERSION 3.15)
    project(libtest C)

    add_library(test test.c)

    install(TARGETS test EXPORT test)
    ```
3. Put `build = "build.rs"` in `Cargo.toml`. e.g.
    ```toml
    [package]
    name = "test_cargo_cmake"
    version = "0.1.0"
    edition = "2021"
    build = "build.rs"

    [build-dependencies]
    cmake = "0.1.50"
    ```
4. Create `build.rs`.
    ```rust
    use cmake::Config;
    
    fn main() {
        let dst = Config::new("libtest").build();
    
        // Tell cargo the link directory, dst.display() is the CMAKE_INSTALL_PREFIX, so we need to add /lib
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
    
        // Tell cargo to link the static libtest
        println!("cargo:rustc-link-lib=static=test");
    }
    ```
5. Then you can use the library in your rust code.
    ```rust
    use std::ffi::c_int;

    extern {
        fn add(a: c_int, b: c_int) -> c_int;
    }

    fn main() {
        let result = unsafe { add(1, 2) as i32 };
        println!("result: {}", result);
    }
    ```

**Note:**

You can omit `println!("cargo:rustc-link-lib=static=test");` in `build.rs` 
and add `#[link(name="test", kind="static")]` in your `main.rs`.
