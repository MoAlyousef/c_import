# c_import

This is a small proc macro crate providing a c_import macro (also a cpp_import macro), which can be used to import C headers into your program. It leverages [bindgen](https://github.com/rust-lang/rust-bindgen), so bindgen needs to be installed in your system.

## Usage
In your Cargo.toml:
```toml
# Cargo.toml

[depenedencies]
c_import = "0.1"
```

In your Rust source file:
```rust
// src/main.rs
use c_import::c_import;

c_import!(<cairo/cairo.h>);

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

In your Rust build script:
```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=cairo");
}
```

Using non-system headers is also possible via enclosing the header path with quotation marks:
```rust
// src/main.rs
use c_import::c_import;
c_import!("src/my_header.h");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

## Extra clang arguments
You can pass extra clang arguments as extra arguments to the macro:
```rust
// src/main.rs
use c_import::c_import;
c_import!("src/my_header.h", "-DMY_DEFINE", "-I/somepath/include");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

## Usage with C++ headers (limited)

```rust
// src/main.rs
use c_import::cpp_import;

cpp_import!(<FL/Fl.H>);

fn main() {
    let version = unsafe { Fl::api_version() }; // static method of class Fl
    println!("{}", version);
}
```

```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=fltk");
}
```

Another example showing how to deal with C++ namespaces:

```cpp
// src/my_header.hpp
#pragma once

namespace my_namespace {
class MyStruct {
    int version_;
  public:
    MyStruct(int version);
    int version() const;
};
}
```

```rust
// src/main.rs
use c_import::cpp_import;

cpp_import!("src/my_header.hpp");

fn main() {
    let h = unsafe { my_namespace_MyStruct::new(2) };
    println!("{}", unsafe { h.version() });
}
```


## Limitations
- Mostly bindgen limitations with C++ headers.