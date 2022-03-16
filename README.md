# c_import

This is a small crate providing a c_import macro (also a cpp_import macro), which can be used to import C headers into your program. You need [bindgen](https://github.com/rust-lang/rust-bindgen) to be installed in your system.

## Usage
```toml
# Cargo.toml

[depenedencies]
c_import = "0.1"
```

```rust
// src/main.rs
use c_import::c_import;

c_import!("/usr/include/cairo/cairo.h");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=cairo");
}
```

If you don't want to pass the absolute path of a system header, create a new header file, and include the system header in it. It would benefit from bindgen's include paths searchability.

```c
// src/my_header.h
#pragma once
#include <cairo/cairo.h>
```

```rust
use c_import::c_import;
c_import!("src/my_header.h");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

## Limitations
- It should work for simple C++ headers. Headers exposing C++ std types would likely fail.